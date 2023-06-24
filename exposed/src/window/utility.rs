use std::{io::Error, mem::MaybeUninit};

use super::{Event, EventHandler, EventHandlerBuilder};

pub trait ExtendedEvent {
    #[inline]
    fn post_event(&mut self) {}

    #[inline]
    fn polled(&mut self) {}

    #[inline]
    fn is_animating(&mut self) -> bool {
        false
    }
}

pub fn run<T: Event + ExtendedEvent>(
    event_handler_builder: EventHandlerBuilder,
) -> Result<(), Error> {
    let mut event_handler_container: MaybeUninit<EventHandler<T>> = MaybeUninit::uninit();
    let mut app_container: MaybeUninit<T> = MaybeUninit::uninit();

    let event_handler = unsafe { &mut *event_handler_container.as_mut_ptr() };
    let app = unsafe { &mut *app_container.as_mut_ptr() };

    unsafe { event_handler_builder.build(event_handler, app) }?;

    while event_handler.running {
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

    unsafe { (event_handler as *mut EventHandler<T>).drop_in_place() };
    unsafe { (app as *mut T).drop_in_place() };

    Ok(())
}
