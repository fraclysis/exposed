use std::{io::Error, marker::PhantomData};

use crate::{destroy::Destroy, window::Event};

#[derive(Debug)]
pub struct EventHandler<E: Event> {
    _mark: PhantomData<E>,
}

impl<E: Event> EventHandler<E> {
    #[inline]
    pub fn poll(&mut self) -> i32 {
        todo!()
    }

    #[inline]
    pub fn wait(&mut self) -> i32 {
        todo!()
    }

    #[inline]
    pub fn dispatch(&mut self) {
        todo!()
    }

    pub unsafe fn new_android() -> Result<(), Error> {
        todo!()
    }
}

impl<E: Event> Destroy for EventHandler<E> {
    fn destroy(&mut self) -> Result<(), Error> {
        todo!()
    }
}

impl<E: Event> Into<crate::window::EventHandler<E>> for EventHandler<E> {
    fn into(self) -> crate::window::EventHandler<E> {
        crate::window::EventHandler(self)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerBuilder {}

impl Default for EventHandlerBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl EventHandlerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub unsafe fn build<E: Event>(&self, _user_data: *mut E) -> Result<EventHandler<E>, Error> {
        todo!()
    }
}
