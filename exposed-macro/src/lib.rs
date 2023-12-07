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

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_debug_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_debug {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_DEBUG.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_DEBUG.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_debug {
  ($tag:expr, $($arg:expr),*) => {
    println!("DEBUG: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("DEBUG: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_default_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_default {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_DEFAULT.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_default {
  ($tag:expr, $($arg:expr),*) => {
    println!("DEFAULT: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("DEFAULT: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_error_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_ERROR.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_ERROR.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_ERROR.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_ERROR.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_error {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_ERROR.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_ERROR.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_error {
  ($tag:expr, $($arg:expr),*) => {
    println!("ERROR: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("ERROR: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_fatal_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_FATAL.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_FATAL.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_FATAL.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_FATAL.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_fatal {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_FATAL.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_FATAL.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_fatal {
  ($tag:expr, $($arg:expr),*) => {
    println!("FATAL: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("FATAL: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_info_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_INFO.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_INFO.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_INFO.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_INFO.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_info {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_INFO.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_INFO.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_info {
  ($tag:expr, $($arg:expr),*) => {
    println!("INFO: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("INFO: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_silent_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_SILENT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_SILENT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_SILENT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_SILENT.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_silent {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_SILENT.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_SILENT.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_silent {
  ($tag:expr, $($arg:expr),*) => {
    println!("SILENT: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("SILENT: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_unknown_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_unknown {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_UNKNOWN.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_unknown {
  ($tag:expr, $($arg:expr),*) => {
    println!("UNKNOWN: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("UNKNOWN: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_verbose_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_verbose {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_VERBOSE.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_verbose {
  ($tag:expr, $($arg:expr),*) => {
    println!("VERBOSE: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("VERBOSE: {} {}", $tag,format!($($arg),*));
  };
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_warn_ {
  ($tag:literal, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_WARN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:literal, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_WARN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
  ($tag:expr, $fmt:literal) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_WARN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt))
  };
  ($tag:expr, $fmt:literal, $($args:expr),*) => {
    $crate::__android_log_print($crate::android_LogPriority::ANDROID_LOG_WARN.0 as _, $crate::cstr!($tag), $crate::cstr!($fmt), $($args),*)
  };
}
#[macro_export]
#[cfg(target_os = "android")]
macro_rules! log_warn {
  ($tag:expr, $($arg:expr),*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_WARN.0, $crate::cstr!($tag),format!($($arg),*))
  };
  ($tag:literal, $($arg:expr)*) => {
    $crate::log_string($crate::android_LogPriority::ANDROID_LOG_WARN.0, $crate::cstr!($tag),format!($($arg),*))
  };
}
#[macro_export]
#[cfg(not(target_os = "android"))]
macro_rules! log_warn {
  ($tag:expr, $($arg:expr),*) => {
    println!("WARN: {} {}", $tag,format!($($arg),*));
  };
  ($tag:literal, $($arg:expr)*) => {
    println!("WARN: {} {}", $tag,format!($($arg),*));
  };
}

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
