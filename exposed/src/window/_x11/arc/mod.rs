mod event_handler;
mod event_handler_builder;
mod window;
mod window_builder;

pub use event_handler::*;
pub use event_handler_builder::*;
pub use window::*;
pub use window_builder::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context {
    pub display: *mut x11::xlib::Display,
}

impl Context {
    pub fn event_handler<E: super::Event>(self) -> &mut EventHandler<E> {
        todo!()
    }

}

pub struct Android<E: super::Event> {
    _mark: std::marker::PhantomData<E>,
}
