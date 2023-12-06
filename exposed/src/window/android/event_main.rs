use std::{io::Error, mem::zeroed, ptr::null_mut, sync::atomic::Ordering};

use exposed_macro::{cstr, log_error, log_info, log_verbose};
use jni_sys::JNINativeInterface_;
use libc::c_void;
use ndk_sys::{
    AInputEvent_getType, AInputQueue_finishEvent, AInputQueue_getEvent, AInputQueue_preDispatchEvent, AKeyEvent_getKeyCode,
    ALooper_pollOnce, ALooper_prepare, AMotionEvent_getAction, AMotionEvent_getPointerCount, AMotionEvent_getPointerId,
    AMotionEvent_getX, AMotionEvent_getY, ANativeActivity, AINPUT_EVENT_TYPE_KEY, AINPUT_EVENT_TYPE_MOTION,
    ALOOPER_POLL_CALLBACK, ALOOPER_POLL_ERROR, ALOOPER_POLL_TIMEOUT, ALOOPER_POLL_WAKE, ALOOPER_PREPARE_ALLOW_NON_CALLBACKS,
    AMOTION_EVENT_ACTION_CANCEL, AMOTION_EVENT_ACTION_DOWN, AMOTION_EVENT_ACTION_MASK, AMOTION_EVENT_ACTION_MOVE,
    AMOTION_EVENT_ACTION_POINTER_DOWN, AMOTION_EVENT_ACTION_POINTER_UP, AMOTION_EVENT_ACTION_UP,
};
use unsafe_utilities::to_ref::ToReference;

use crate::window::{platform::WindowHandle, Event, Key, Touch, TouchPhase};

use super::{ActivityContext, Context, WaitState};

pub extern "C" fn main<E: Event>(data: *mut c_void) -> *mut c_void {
    unsafe { _main::<E>((data as *mut ActivityContext).to_ref()) }
}

unsafe fn _main<E: Event>(data: &mut ActivityContext) -> *mut c_void {
    let context = Context(data);

    let looper = ALooper_prepare(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as _);

    data.looper.store(looper, Ordering::Release);

    if let Some(mut _e) = E::create(context) {
        use WaitState::*;

        log_info!("Exposed", "Ok");

        let context = context.get();

        if !context.show_called {
            // todo!("Exit the application.");
        }

        let act = data;
        let vm = act.android_activity.vm;
        let mut env = zeroed();

        (vm.to_ref().to_ref().AttachCurrentThread.unwrap())(vm, &mut env, null_mut());

        pub type JNIEnv = *const JNINativeInterface_;

        let env = env as *mut JNIEnv;

        let version = (env.to_ref().to_ref().v1_1.GetVersion)(env);

        log_error!("Exposed", "jni {version}");

        let class = (env.to_ref().to_ref().v1_1.GetObjectClass)(env, act.android_activity.clazz as _);
        log_error!("Exposed", "jni Class{}" class as usize);

        let _method_id = (env.to_ref().to_ref().v1_1.GetMethodID)(env, class, cstr!("showSoftKeyboard"), cstr!("()V"));

        // TODO:(fraclysis) release the application for running
        match context.waits_at() {
            None => unreachable!(),
            Running => {
                todo!()
            }
            OnCreate => {
                todo!()
            }
            OnDestroy => {
                todo!()
            }
            OnNativeWindowCreated => {
                context.post_receiver().unwrap();
            }
            OnSaveInstanceState => {}
            OnWindowFocusChanged => {}
            OnNativeWindowResized => {}
            OnNativeWindowRedrawNeeded => {}
            OnNativeWindowDestroyed => {}
            OnInputQueueCreated => {}
            OnInputQueueDestroyed => {}
            OnContentRectChanged => {}
            OnConfigurationChanged => {}
            OnLowMemory => {}
        }

        let window = WindowHandle { context: Context(context) }.into();

        loop {
            let waits_at = context.waits_at();
            let waits = waits_at;

            match waits {
                None => unreachable!(),

                Running => {
                    let mut fd = zeroed();
                    let mut events = zeroed();
                    let mut out_data = zeroed();
                    let l = ALooper_pollOnce(-1, &mut fd, &mut events, &mut out_data);
                    match l {
                        ALOOPER_POLL_WAKE => {
                            _e.render(window);
                        }
                        ALOOPER_POLL_CALLBACK => {}
                        ALOOPER_POLL_TIMEOUT => {}
                        ALOOPER_POLL_ERROR => {}
                        ActivityContext::INPUT_QUEUE_IDENT => {
                            let queue = context.input_queue.load(Ordering::Acquire);
                            let mut event = zeroed();

                            if AInputQueue_getEvent(queue, &mut event) >= 0 {
                                if AInputQueue_preDispatchEvent(queue, event) != 0 {
                                    continue;
                                }

                                let event_type = AInputEvent_getType(event);

                                match event_type as u32 {
                                    AINPUT_EVENT_TYPE_MOTION => {
                                        let pointer_size = AMotionEvent_getPointerCount(event);

                                        for i in 0..pointer_size {
                                            let pointer_id = AMotionEvent_getPointerId(event, i);

                                            let action = AMotionEvent_getAction(event) as u32;

                                            let action = (action & AMOTION_EVENT_ACTION_MASK) as i32;

                                            let mut phase: Option<TouchPhase> = Option::None;

                                            // TODO:(fraclysis) Make touch phase type as keys type
                                            match action as u32 {
                                                AMOTION_EVENT_ACTION_DOWN => {
                                                    phase = Some(TouchPhase::Started);
                                                }
                                                AMOTION_EVENT_ACTION_UP => {
                                                    phase = Some(TouchPhase::Ended);
                                                }
                                                AMOTION_EVENT_ACTION_MOVE => {
                                                    phase = Some(TouchPhase::Moved);
                                                }
                                                AMOTION_EVENT_ACTION_CANCEL => {
                                                    phase = Some(TouchPhase::Cancelled);
                                                }
                                                AMOTION_EVENT_ACTION_POINTER_DOWN => {
                                                    phase = Some(TouchPhase::Started);
                                                }
                                                AMOTION_EVENT_ACTION_POINTER_UP => {
                                                    phase = Some(TouchPhase::Ended);
                                                }

                                                _ => {}
                                            }

                                            if let Some(phase) = phase {
                                                let x = AMotionEvent_getX(event, i);
                                                let y = AMotionEvent_getY(event, i);

                                                let touch = Touch {
                                                    phase,
                                                    location: (x, y),
                                                    id: pointer_id as _,
                                                    os_data: event as _,
                                                    pointer_index: i,
                                                };

                                                _e.touch(window, touch, pointer_size);
                                            }
                                        }

                                        _e.touch_end(window);
                                    }

                                    AINPUT_EVENT_TYPE_KEY => {
                                        let _key = Key(AKeyEvent_getKeyCode(event) as _);
                                    }
                                    _ => {}
                                }

                                // TODO:(fraclysis) Handle events on here

                                AInputQueue_finishEvent(queue, event, 0);
                            } else {
                                log_error!("Exposed", "Failure reading next input event: {}", Error::last_os_error());
                            }
                        }
                        _ => (),
                    }
                }

                OnCreate => {
                    // Window is not created by the application so we exit.
                    context.post_receiver().unwrap();
                    todo!("Exit the application.")
                }

                OnSaveInstanceState => {
                    context.post_receiver().unwrap();
                }
                OnWindowFocusChanged => {
                    context.post_receiver().unwrap();
                }

                OnNativeWindowCreated => {
                    log_verbose!("Exposed", "Window create");
                    _e.show(window);
                    context.post_receiver().unwrap();
                }
                OnNativeWindowResized => {
                    if let Ok(s) = window.client_size() {
                        _e.resized(window, s.width, s.height);
                    } else {
                        _e.resized(window, 0, 0);
                    }
                    context.post_receiver().unwrap();
                }
                OnNativeWindowRedrawNeeded => {
                    _e.render(window);
                    context.post_receiver().unwrap();
                }
                OnNativeWindowDestroyed => {
                    _e.minimized(window);

                    context.post_receiver().unwrap();
                }

                OnInputQueueCreated => {
                    context.looper_attach();
                    context.post_receiver().unwrap();
                }
                OnInputQueueDestroyed => {
                    context.looper_detach();
                    context.post_receiver().unwrap();
                }

                OnContentRectChanged => {
                    context.post_receiver().unwrap();
                }
                OnConfigurationChanged => {
                    context.post_receiver().unwrap();
                }
                OnLowMemory => {
                    context.post_receiver().unwrap();
                }

                OnDestroy => break,
            }
        }

        drop(_e)
    } else {
        log_info!("Exposed", "Fail");
    }

    null_mut()
}

pub fn panic_last_error() -> ! {
    panic!("{}", Error::last_os_error())
}

pub unsafe fn get_context(activity: *mut ANativeActivity) -> &'static mut ActivityContext {
    let a = activity.to_ref().instance as *mut ActivityContext;
    a.to_ref()
}
