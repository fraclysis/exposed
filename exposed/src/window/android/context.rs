use std::{
    io::Error,
    mem::zeroed,
    ptr::null,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering},
};

use exposed_macro::log_error;
use libc::{
    pthread_create, pthread_join, pthread_t, sem_destroy, sem_init, sem_post, sem_t, sem_wait,
};
use ndk_sys::{
    AInputQueue, AInputQueue_attachLooper, AInputQueue_detachLooper, ALooper, ALooper_wake,
    ANativeActivity, ANativeWindow,
};
use unsafe_utilities::{
    allocate::{allocate_zeroed, deallocate},
    broke_checker::AsReference,
};

use crate::{
    destroy::Destroy,
    window::{platform::main, Event},
};

use super::panic_last_error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context(pub *mut ActivityContext);

impl Context {
    pub fn get(self) -> &'static mut ActivityContext {
        unsafe { &mut *self.0 }
    }
}

pub struct ActivityContext {
    pub android_activity: &'static mut ANativeActivity,
    pub thread: pthread_t,
    pub receiver_wait: sem_t,
    pub event_wait: sem_t,
    pub waiting_at: AtomicU32,
    pub native_window: AtomicPtr<ANativeWindow>,
    pub input_queue: AtomicPtr<AInputQueue>,
    pub window_created: AtomicBool,
    pub show_called: bool,
    pub unhandled_messages: Vec<u8>,
    pub looper: AtomicPtr<ALooper>,
}

impl Destroy for ActivityContext {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        unsafe {
            let mut ret_val = zeroed();
            let err = pthread_join(self.thread, &mut ret_val);
            if err != 0 {
                log_error!("Exposed", "{}", Error::from_raw_os_error(err));
            }

            log_sem_destroy(&mut self.event_wait);
            log_sem_destroy(&mut self.receiver_wait);

            std::ptr::drop_in_place(&mut self.unhandled_messages);
            deallocate(self);
            Ok(())
        }
    }
}

impl ActivityContext {
    pub fn init<E: Event>(activity: *mut ANativeActivity) -> Result<&'static mut Self, Error> {
        unsafe {
            let this_ptr = allocate_zeroed::<Self>();

            let this = this_ptr.to_ref();
            this.android_activity = activity.to_ref();

            if sem_init(&mut this.event_wait, 0, 0) == -1 {
                let err = Error::last_os_error();
                deallocate(this_ptr);
                return Err(err);
            }

            if sem_init(&mut this.receiver_wait, 0, 0) == -1 {
                let err = Error::last_os_error();
                log_sem_destroy(&mut this.event_wait);
                deallocate(this_ptr);
                return Err(err);
            }

            let res = pthread_create(&mut this.thread, null(), main::<E>, this_ptr as _);
            if res != 0 {
                log_sem_destroy(&mut this.event_wait);
                log_sem_destroy(&mut this.receiver_wait);
                deallocate(this_ptr);
                return Err(Error::from_raw_os_error(res));
            }

            activity.to_ref().instance = this_ptr as _;
            std::ptr::write(&mut this.unhandled_messages, Vec::new());

            Ok(this)
        }
    }

    pub fn set_wait(&mut self, wait: WaitState) {
        self.waiting_at.store(wait as u32, Ordering::Release);
    }

    pub fn waits_at(&mut self) -> WaitState {
        unsafe { std::mem::transmute(self.waiting_at.load(Ordering::Acquire)) }
    }

    pub fn post(&mut self) {
        unsafe {
            if sem_post(&mut self.receiver_wait) == -1 {
                panic_last_error();
            }

            if sem_wait(&mut self.event_wait) == -1 {
                panic_last_error();
            }
        }
    }

    pub fn post_receiver(&mut self) -> Result<(), Error> {
        self.set_wait(WaitState::Running);
        if unsafe { sem_post(&mut self.receiver_wait) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn post_event(&mut self) -> Result<(), Error> {
        if unsafe { sem_post(&mut self.event_wait) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn event_wait(&mut self) -> Result<(), Error> {
        unsafe {
            if sem_wait(&mut self.event_wait) == -1 {
                return Err(Error::last_os_error());
            }

            Ok(())
        }
    }

    pub fn receiver_wait(&mut self) -> Result<(), Error> {
        unsafe {
            let looper = self.looper.load(Ordering::Acquire);

            ALooper_wake(looper);

            if sem_wait(&mut self.receiver_wait) == -1 {
                return Err(Error::last_os_error());
            }

            Ok(())
        }
    }

    pub const INPUT_QUEUE_IDENT: i32 = 11;

    pub fn looper_attach(&mut self) {
        let queue = self.input_queue.load(Ordering::Acquire);

        // We own the looper
        let looper = self.looper.load(Ordering::Relaxed);

        unsafe { AInputQueue_attachLooper(queue, looper, Self::INPUT_QUEUE_IDENT, None, 0 as _) };
    }

    pub fn looper_detach(&mut self) {
        let queue = self.input_queue.load(Ordering::Acquire);

        unsafe { AInputQueue_detachLooper(queue) };
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitState {
    None,
    Running,
    OnCreate,
    OnDestroy,
    OnSaveInstanceState,
    OnWindowFocusChanged,
    OnNativeWindowCreated,
    OnNativeWindowResized,
    OnNativeWindowRedrawNeeded,
    OnNativeWindowDestroyed,
    OnInputQueueCreated,
    OnInputQueueDestroyed,
    OnContentRectChanged,
    OnConfigurationChanged,
    OnLowMemory,
}

unsafe fn log_sem_destroy(sem: *mut sem_t) {
    if sem_destroy(sem) == -1 {
        let err = Error::last_os_error();
        log_error!("Exposed", "{}", err);
    }
}
