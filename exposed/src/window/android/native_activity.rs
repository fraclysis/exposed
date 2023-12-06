use exposed_macro::log_fatal;
use libc::{c_void, size_t};
use ndk_sys::{AInputQueue, ANativeActivity, ANativeWindow, ARect};
use unsafe_utilities::to_ref::ToReference;

#[allow(unused_variables)]
pub trait AndroidCallback {
    unsafe extern "C" fn on_create(activity: *mut ANativeActivity, saved_state: *mut c_void, saved_state_size: size_t) {}
    unsafe extern "C" fn on_start(activity: *mut ANativeActivity) {}
    unsafe extern "C" fn on_resume(activity: *mut ANativeActivity) {}
    unsafe extern "C" fn on_pause(activity: *mut ANativeActivity) {}
    unsafe extern "C" fn on_stop(activity: *mut ANativeActivity) {}
    unsafe extern "C" fn on_destroy(activity: *mut ANativeActivity) {}

    unsafe extern "C" fn on_save_instance_state(
        activity: *mut ANativeActivity, out_size: *mut usize,
    ) -> *mut ::std::os::raw::c_void {
        *out_size = 0;
        0 as _
    }

    unsafe extern "C" fn on_window_focus_changed(activity: *mut ANativeActivity, has_focus: ::std::os::raw::c_int) {}

    unsafe extern "C" fn on_native_window_created(activity: *mut ANativeActivity, window: *mut ANativeWindow) {}

    unsafe extern "C" fn on_native_window_resized(activity: *mut ANativeActivity, window: *mut ANativeWindow) {}

    unsafe extern "C" fn on_native_window_redraw_needed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {}

    unsafe extern "C" fn on_native_window_destroyed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {}

    unsafe extern "C" fn on_input_queue_created(activity: *mut ANativeActivity, queue: *mut AInputQueue) {}

    unsafe extern "C" fn on_input_queue_destroyed(activity: *mut ANativeActivity, queue: *mut AInputQueue) {}

    unsafe extern "C" fn on_content_rect_changed(activity: *mut ANativeActivity, rect: *const ARect) {}

    unsafe extern "C" fn on_configuration_changed(activity: *mut ANativeActivity) {}

    unsafe extern "C" fn on_low_memory(activity: *mut ANativeActivity) {}
}

#[allow(unused_variables)]
pub unsafe fn init_callbacks<C: AndroidCallback>(activity: usize, saved_state: usize, saved_state_size: usize) {
    let activity: *mut ANativeActivity = activity as _;
    let saved_state: *mut c_void = saved_state as _;
    let saved_state_size: size_t = saved_state_size as _;

    let callbacks = activity.to_ref().callbacks.to_ref();

    callbacks.onStart = Some(C::on_start);
    callbacks.onResume = Some(C::on_resume);
    callbacks.onSaveInstanceState = Some(C::on_save_instance_state);
    callbacks.onPause = Some(C::on_pause);
    callbacks.onStop = Some(C::on_stop);
    callbacks.onDestroy = Some(C::on_destroy);

    callbacks.onWindowFocusChanged = Some(C::on_window_focus_changed);
    callbacks.onNativeWindowCreated = Some(C::on_native_window_created);
    callbacks.onNativeWindowResized = Some(C::on_native_window_resized);
    callbacks.onNativeWindowRedrawNeeded = Some(C::on_native_window_redraw_needed);
    callbacks.onNativeWindowDestroyed = Some(C::on_native_window_destroyed);

    callbacks.onInputQueueCreated = Some(C::on_input_queue_created);
    callbacks.onInputQueueDestroyed = Some(C::on_input_queue_destroyed);

    callbacks.onContentRectChanged = Some(C::on_content_rect_changed);
    callbacks.onConfigurationChanged = Some(C::on_configuration_changed);
    callbacks.onLowMemory = Some(C::on_low_memory);

    C::on_create(activity, saved_state, saved_state_size);
}

pub fn set_panic_handler() {
    std::panic::set_hook(Box::new(move |p| {
        log_fatal!("Exposed", "{}", p);
    }))
}
