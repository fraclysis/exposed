use std::{io::Error, ptr::null_mut, sync::atomic::Ordering};

use exposed_macro::log_verbose_;
use libc::{sem_wait, size_t};
use ndk_sys::{
    AInputQueue, ANativeActivity, ANativeWindow, ARect,
};
use unsafe_utilities::broke_checker::AsReference;

use crate::{
    destroy::Destroy,
    window::{
        platform::{get_context, ActivityContext, WaitState},
        Event, utility::ExtendedEvent,
    },
};

use super::native_activity::{set_panic_handler, AndroidCallback};

#[derive(Debug, Default)]
pub struct Android<E: Event + ExtendedEvent> {
    _mark: std::marker::PhantomData<E>,
}

impl<E: Event + ExtendedEvent> Android<E> {}

#[allow(unused_variables)]
impl<E: Event + ExtendedEvent> AndroidCallback for Android<E> {
    unsafe extern "C" fn on_create(activity: *mut ANativeActivity, saved_state: *mut libc::c_void, saved_state_size: size_t) {
        log_verbose_!("Exposed", "Create");
        set_panic_handler();

        let context = ActivityContext::init::<E>(activity.to_ref()).unwrap();

        context.set_wait(WaitState::OnCreate);

        if sem_wait(&mut context.receiver_wait) == -1 {
            return Err(Error::last_os_error()).unwrap();
        }
    }

    unsafe extern "C" fn on_destroy(activity: *mut ANativeActivity) {
        log_verbose_!("Exposed", "Destroy");
        let context = get_context(activity);

        context.set_wait(WaitState::OnDestroy);
        context.destroy().unwrap();
    }

    unsafe extern "C" fn on_save_instance_state(
        activity: *mut ANativeActivity, out_size: *mut usize,
    ) -> *mut std::os::raw::c_void {
        log_verbose_!("Exposed", "Save");

        let context = get_context(activity);

        context.set_wait(WaitState::OnSaveInstanceState);
        context.receiver_wait().unwrap();

        *out_size = 0;
        0 as _
    }

    unsafe extern "C" fn on_window_focus_changed(activity: *mut ANativeActivity, has_focus: std::os::raw::c_int) {
        log_verbose_!("Exposed", "Focus");

        let context = get_context(activity);

        context.set_wait(WaitState::OnWindowFocusChanged);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_native_window_created(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        log_verbose_!("Exposed", "WindowCreate");

        let context = get_context(activity);

        context.native_window.store(window, Ordering::Release);

        context.set_wait(WaitState::OnNativeWindowCreated);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_native_window_resized(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        log_verbose_!("Exposed", "WindowResize");

        let context = get_context(activity);

        context.set_wait(WaitState::OnNativeWindowResized);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_native_window_redraw_needed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        log_verbose_!("Exposed", "Redraw");

        let context = get_context(activity);

        context.set_wait(WaitState::OnNativeWindowRedrawNeeded);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_native_window_destroyed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        log_verbose_!("Exposed", "WindowDestroyed");

        let context = get_context(activity);

        context.set_wait(WaitState::OnNativeWindowDestroyed);
        context.receiver_wait().unwrap();

        context.native_window.store(null_mut(), Ordering::Release);
    }

    unsafe extern "C" fn on_input_queue_created(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        log_verbose_!("Exposed", "InputQueueCreate");

        let context = get_context(activity);
        context.input_queue.store(queue, Ordering::Release);

        context.set_wait(WaitState::OnInputQueueCreated);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_input_queue_destroyed(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        log_verbose_!("Exposed", "InputQueueDestroy");

        let context = get_context(activity);

        context.set_wait(WaitState::OnInputQueueDestroyed);
        context.receiver_wait().unwrap();

        context.input_queue.store(null_mut(), Ordering::Release);
    }

    unsafe extern "C" fn on_content_rect_changed(activity: *mut ANativeActivity, rect: *const ARect) {
        log_verbose_!("Exposed", "Rect");

        let context = get_context(activity);

        context.set_wait(WaitState::OnContentRectChanged);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_configuration_changed(activity: *mut ANativeActivity) {
        log_verbose_!("Exposed", "Config");

        let context = get_context(activity);

        context.set_wait(WaitState::OnConfigurationChanged);
        context.receiver_wait().unwrap();
    }

    unsafe extern "C" fn on_low_memory(activity: *mut ANativeActivity) {
        log_verbose_!("Exposed", "LowMemory");

        let context = get_context(activity);

        context.set_wait(WaitState::OnLowMemory);
        context.receiver_wait().unwrap();
    }
}
