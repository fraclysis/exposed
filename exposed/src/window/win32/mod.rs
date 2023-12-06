mod context;
mod event_handler;
mod win_proc;
mod window;

pub use context::*;
pub use event_handler::*;
pub use win_proc::*;
pub use window::*;

pub use windows_sys;

pub struct Android<T> {
    _mark: std::marker::PhantomData<T>,
}
