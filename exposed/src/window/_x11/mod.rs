mod window;
pub use window::*;

mod context;
pub use context::*;

mod event_handler;
pub use event_handler::*;

pub struct Android<E: super::Event>(pub std::marker::PhantomData<E>);
