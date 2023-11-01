use exposed_proc::log_impl;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        ::core::concat!($s, '\0').as_ptr() as *const std::ffi::c_char
    };
    ($s:expr) => {
        $s
    };
}

#[cfg(target_os = "android")]
pub use ndk_sys::{__android_log_print, android_LogPriority};

log_impl!(debug, $crate::android_LogPriority::ANDROID_LOG_DEBUG.0);

log_impl!(default, $crate::android_LogPriority::ANDROID_LOG_DEFAULT.0);

log_impl!(error, $crate::android_LogPriority::ANDROID_LOG_ERROR.0);

log_impl!(fatal, $crate::android_LogPriority::ANDROID_LOG_FATAL.0);

log_impl!(info, $crate::android_LogPriority::ANDROID_LOG_INFO.0);

log_impl!(silent, $crate::android_LogPriority::ANDROID_LOG_SILENT.0);

log_impl!(unknown, $crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0);

log_impl!(verbose, $crate::android_LogPriority::ANDROID_LOG_VERBOSE.0);

log_impl!(warn, $crate::android_LogPriority::ANDROID_LOG_WARN.0);

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! android_on_create {
    ($callbacks:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn ANativeActivity_onCreate(activity: usize, saved_state: usize, saved_state_size: usize) {
            exposed::window::android::init_callbacks::<$callbacks>(activity, saved_state, saved_state_size)
        }
    };
}

#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! android_on_create {
    ($callbacks:ty) => {
        type _AndroidOnCreatePlaceHolder = $callbacks;
    };
}

#[cfg(target_os = "android")]
pub fn log_string(level: std::ffi::c_uint, tag: *const std::ffi::c_char, mut message: String) {
    message.push('\0');
    unsafe { __android_log_print(level as _, tag, cstr!("%s"), message.as_ptr()) };
}
