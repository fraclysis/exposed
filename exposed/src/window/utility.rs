use std::{
    alloc::{alloc, Layout},
    io::Error,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use crate::{
    destroy::Destroyable,
    window::{Event, EventHandler},
};

use super::EventHandlerBuilder;

pub trait ExtendedEvent {
    #[inline]
    fn post_event(&mut self) {}

    #[inline]
    fn polled(&mut self) {}

    #[inline]
    fn is_animating(&mut self) -> bool {
        false
    }

    fn is_running(&mut self) -> bool;
}

pub fn run<T: Event + ExtendedEvent>(mut event_handler_builder: EventHandlerBuilder) -> Result<(), Error> {
    let mut app_container: MaybeUninit<T> = MaybeUninit::uninit();

    let app = unsafe { &mut *app_container.as_mut_ptr() };

    let mut event_handler = Destroyable(unsafe { event_handler_builder.build(app) }?);

    while app.is_running() {
        if app.is_animating() {
            while event_handler.poll() > 0 {
                event_handler.dispatch();
                app.post_event();
            }

            app.polled();
        } else {
            event_handler.wait();
            event_handler.dispatch();

            app.post_event();
        }
    }

    unsafe { (app as *mut T).drop_in_place() };

    Ok(())
}

pub struct HeapEventHandler<E: Event> {
    pub user_data: Box<E>,
    event_handler: Destroyable<EventHandler<E>>,
}

impl<E: Event> Deref for HeapEventHandler<E> {
    type Target = EventHandler<E>;

    fn deref(&self) -> &Self::Target {
        &self.event_handler
    }
}

impl<E: Event> DerefMut for HeapEventHandler<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event_handler
    }
}

impl<E: Event> HeapEventHandler<E> {
    pub fn new(mut builder: EventHandlerBuilder) -> Result<HeapEventHandler<E>, Error> {
        unsafe {
            let user_data = alloc(Layout::new::<E>()) as *mut E;
            if user_data.is_null() {
                std::alloc::handle_alloc_error(Layout::new::<E>())
            }

            let event_handler = Destroyable(builder.build(user_data)?);

            Ok(Self { user_data: Box::from_raw(user_data), event_handler })
        }
    }
}
