use std::io::Error;

use crate::destroy::Destroy;

use super::{platform, Event};

#[derive(Debug)]
/// Provides a way to control event loop in a platform compatible way.
pub struct EventHandler<E: Event>(pub platform::EventHandler<E>);

impl<E: Event> EventHandler<E> {
    /// Returns 0 if not message is available.
    /// If return value is bigger than 0 `EventHandler::dispatch` must be called.
    ///
    /// Represents:
    /// - `PeekMessageW` in Windows
    /// - `XCheckIfEvent` in X11
    /// - `ALooper_poolAll` with timeout 0 in Android
    pub fn poll(&mut self) -> i32 {
        self.0.poll()
    }

    /// Return value is ignored.
    ///
    /// Represents:
    /// - `GetMessageW` in Windows
    /// - `XNextEvent` in X11
    /// - `ALooper_poolAll` with timeout negative in Android
    pub fn wait(&mut self) -> i32 {
        self.0.wait()
    }

    pub fn dispatch(&mut self) {
        self.0.dispatch()
    }
}

impl<E: Event> Destroy for EventHandler<E> {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        self.0.destroy()
    }
}

#[derive(Debug, Default)]
/// Provides a way to create `EventHandler` in a platform compatible way.
pub struct EventHandlerBuilder(pub platform::EventHandlerBuilder);

impl EventHandlerBuilder {
    #[inline]
    pub unsafe fn build<E: Event>(&mut self, user_data: *mut E) -> Result<EventHandler<E>, Error> {
        Ok(self.0.build(user_data)?.into())
    }
}
