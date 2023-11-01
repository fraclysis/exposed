use std::io::Error;

use crate::destroy::Destroy;

use super::{platform, Event};

#[derive(Debug)]
pub struct EventHandler<E: Event>(pub platform::EventHandler<E>);

impl<E: Event> EventHandler<E> {
    pub fn poll(&mut self) -> i32 {
        self.0.poll()
    }

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
pub struct EventHandlerBuilder(pub platform::EventHandlerBuilder);

impl EventHandlerBuilder {
    #[inline]
    pub unsafe fn build<E: Event>(&mut self, user_data: *mut E) -> Result<EventHandler<E>, Error> {
        Ok(self.0.build(user_data)?.into())
    }
}
