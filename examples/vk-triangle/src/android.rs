use exposed::{
    log::{cstr, log_info_},
    window::{glue::AndroidCallback, Size},
};
use libc::size_t;
use ndk_sys::{
    AInputEvent_getType, AInputQueue_attachLooper, AInputQueue_finishEvent, AInputQueue_getEvent,
    AInputQueue_preDispatchEvent, AKeyEvent_getKeyCode, ALooper_pollAll, ALooper_prepare,
    ANativeActivity, ANativeWindow_getHeight, ANativeWindow_getWidth, ALOOPER_POLL_ERROR,
    ALOOPER_POLL_TIMEOUT, ALOOPER_PREPARE_ALLOW_NON_CALLBACKS,
};
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, RawDisplayHandle, RawWindowHandle,
};

use crate::triangle::Triangle;

#[no_mangle]
pub extern "C" fn hello() -> i32 {
    143 + 1
}

pub struct Callbacks;

#[allow(unused)]
impl AndroidCallback for Callbacks {
    unsafe extern "C" fn on_create(
        activity: *mut ANativeActivity, saved_state: *mut std::ffi::c_void,
        saved_state_size: size_t,
    ) {
    }

    unsafe extern "C" fn on_input_queue_created(
        activity: *mut ANativeActivity, queue: *mut ndk_sys::AInputQueue,
    ) {
        let q = queue as usize;

        std::thread::spawn(move || {
            exposed::window::glue::set_panic_handler();

            let queue = q as _;

            let looper = ALooper_prepare(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as _);

            let mut fd = 0;
            let mut events = 0;
            let mut data = 0 as _;

            AInputQueue_attachLooper(queue, looper, 21, None, 10 as _);

            loop {
                let pool = ALooper_pollAll(0, &mut fd, &mut events, &mut data);

                if pool == ALOOPER_POLL_ERROR {
                    panic!()
                } else if pool != ALOOPER_POLL_TIMEOUT {
                    let mut event = 0 as _;
                    if AInputQueue_getEvent(queue, &mut event) >= 0 {
                        let event_type = AInputEvent_getType(event);
                        if AInputQueue_preDispatchEvent(queue, event) != 0 {
                            continue;
                        }

                        match event_type as u32 {
                            ndk_sys::AINPUT_EVENT_TYPE_CAPTURE => {}
                            ndk_sys::AINPUT_EVENT_TYPE_DRAG => {}
                            ndk_sys::AINPUT_EVENT_TYPE_FOCUS => {}
                            ndk_sys::AINPUT_EVENT_TYPE_KEY => {
                                let key = exposed::window::Key(AKeyEvent_getKeyCode(event) as _);
                                let key = format!("{key:?}\0");

                                log_info_!("VkTriangleKey", "Key %s", key.as_ptr());
                            }
                            ndk_sys::AINPUT_EVENT_TYPE_MOTION => {}
                            ndk_sys::AINPUT_EVENT_TYPE_TOUCH_MODE => {}
                            _ => (),
                        }

                        AInputQueue_finishEvent(queue, event, 0);
                    }

                    log_info_!(
                        "VkTriangleKey",
                        "    fd: %d \n    events : %d data: %p",
                        fd,
                        events,
                        data
                    );
                }
            }
        });
    }
}

const TAG: *const std::ffi::c_char = cstr!("TAG");

pub struct Callbacks2;

impl AndroidCallback for Callbacks2 {
    unsafe extern "C" fn on_create(
        activity: *mut ANativeActivity, saved_state: *mut libc::c_void, saved_state_size: size_t,
    ) {
        log_info_!(TAG, "OnCreate %p %p %zu", activity, saved_state, saved_state_size);
    }

    unsafe extern "C" fn on_start(activity: *mut ANativeActivity) {
        log_info_!(TAG, "OnStart %p", activity);
    }

    unsafe extern "C" fn on_resume(activity: *mut ANativeActivity) {
        log_info_!(TAG, "OnResume %p", activity);
    }

    unsafe extern "C" fn on_pause(activity: *mut ANativeActivity) {
        log_info_!(TAG, "OnPause %p", activity);
    }

    unsafe extern "C" fn on_stop(activity: *mut ANativeActivity) {
        log_info_!(TAG, "OnStop %p", activity);
    }

    unsafe extern "C" fn on_destroy(activity: *mut ANativeActivity) {
        log_info_!(TAG, "OnDestroy %p", activity);
    }

    unsafe extern "C" fn on_save_instance_state(
        activity: *mut ANativeActivity, out_size: *mut usize,
    ) -> *mut std::os::raw::c_void {
        log_info_!(TAG, "OnSave %p", activity);

        *out_size = 0;
        0 as _
    }

    unsafe extern "C" fn on_window_focus_changed(
        activity: *mut ANativeActivity, has_focus: std::os::raw::c_int,
    ) {
        exposed::window::glue::set_panic_handler();
        log_info_!(TAG, "FocusChanged %p %d", activity, has_focus);
    }

    unsafe extern "C" fn on_native_window_created(
        activity: *mut ANativeActivity, window: *mut ndk_sys::ANativeWindow,
    ) {
        log_info_!(TAG, "WindowCreate %p, %p", activity, window);
    }

    unsafe extern "C" fn on_native_window_resized(
        activity: *mut ANativeActivity, window: *mut ndk_sys::ANativeWindow,
    ) {
        log_info_!(TAG, "WindowResize %p, %p", activity, window);
    }

    unsafe extern "C" fn on_native_window_redraw_needed(
        activity: *mut ANativeActivity, window: *mut ndk_sys::ANativeWindow,
    ) {
        log_info_!(TAG, "RedrawNeeded %p, %p", activity, window);

        let d = AndroidDisplayHandle::empty();
        let mut w = AndroidNdkWindowHandle::empty();
        w.a_native_window = window as _;

        let size =
            Size { width: ANativeWindow_getWidth(window), height: ANativeWindow_getHeight(window) };

        let mut t =
            Triangle::new(RawWindowHandle::AndroidNdk(w), RawDisplayHandle::Android(d), size)
                .unwrap();

        t.render();
    }

    unsafe extern "C" fn on_native_window_destroyed(
        activity: *mut ANativeActivity, window: *mut ndk_sys::ANativeWindow,
    ) {
        log_info_!(TAG, "WindowDestroy %p, %p", activity, window);
    }

    unsafe extern "C" fn on_input_queue_created(
        activity: *mut ANativeActivity, queue: *mut ndk_sys::AInputQueue,
    ) {
        log_info_!(TAG, "InputQueueCreate %p, %p", activity, queue);
    }

    unsafe extern "C" fn on_input_queue_destroyed(
        activity: *mut ANativeActivity, queue: *mut ndk_sys::AInputQueue,
    ) {
        log_info_!(TAG, "InputQueueDestroyed %p, %p", activity, queue);
    }

    unsafe extern "C" fn on_content_rect_changed(
        activity: *mut ANativeActivity, rect: *const ndk_sys::ARect,
    ) {
        log_info_!(TAG, "InputQueueRect %p, %p", activity, rect);
    }

    unsafe extern "C" fn on_configuration_changed(activity: *mut ANativeActivity) {
        log_info_!(TAG, "ConfigChange %p", activity);
    }

    unsafe extern "C" fn on_low_memory(activity: *mut ANativeActivity) {
        log_info_!(TAG, "LowMemory %p", activity);
    }
}
