pub mod window;

pub mod destroy;

pub mod log {
    pub use exposed_macro::{
        cstr, log_debug, log_default, log_error, log_fatal, log_info, log_silent, log_unknown, log_verbose, log_warn,
    };

    pub trait LogResult {
        fn log_error(self);
    }

    impl<E: std::error::Error> LogResult for Result<(), E> {
        fn log_error(self) {
            if let Err(e) = self {
                log_error!("LogResult", "{e}");
            }
        }
    }
}

pub use unsafe_utilities;
