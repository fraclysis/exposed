use std::{
    ffi::{c_int, c_long},
    io::{Error, ErrorKind},
    mem::zeroed,
    ptr::{null, null_mut},
    sync::atomic::{AtomicUsize, Ordering},
};

use exposed_macro::{cstr, log_warn};
use libc::{c_ulong, setlocale, LC_ALL};
use x11::xlib::{
    self, Display, Expose, Screen, True, XCheckIfEvent, XCheckTypedEvent, XCloseDisplay, XCloseIM, XConvertCase, XDefaultScreen,
    XDefaultScreenOfDisplay, XEvent, XInternAtom, XKeysymToKeycode, XLookupBoth, XLookupChars, XLookupKeySym, XNextEvent,
    XOpenDisplay, XOpenIM, XSetLocaleModifiers, XSupportsLocale, Xutf8LookupString, _XDisplay, _XIM,
};

use crate::{
    destroy::Destroy,
    window::{platform::ThreadContext, Context, Event, Key, MouseButton},
};

use super::WindowHandle;

#[derive(Debug)]
pub struct EventHandler<E: Event> {
    pub screen: *mut Screen,
    pub im: *mut _XIM,
    pub display: *mut Display,
    pub screen_id: c_int,
    pub wm_delete: c_ulong,
    pub user_data: *mut E,
    pub event: XEvent,
}

impl<E: Event> EventHandler<E> {
    pub fn poll(&mut self) -> i32 {
        unsafe {
            extern "C" fn predicate(_display: *mut _XDisplay, _event: *mut XEvent, _arg: *mut i8) -> i32 {
                1
            }

            XCheckIfEvent(self.display, &mut self.event, Some(predicate), null_mut())
        }
    }

    pub fn wait(&mut self) -> i32 {
        unsafe { XNextEvent(self.display, &mut self.event) }
    }

    pub fn dispatch(&mut self) {
        unsafe {
            let app = &mut *self.user_data;
            let event = &mut self.event;

            match event.type_ {
                xlib::Expose => {
                    if XCheckTypedEvent(self.display, Expose, event) != 0 {}

                    app.low_render(WindowHandle(event.expose.window, self.display).into());
                }

                xlib::ConfigureNotify => {
                    let window = WindowHandle(event.expose.window, self.display).into();
                    app.resized(window, event.configure.width, event.configure.height);
                    app.moved(window, event.configure.x, event.configure.y);
                }

                xlib::FocusIn => {
                    let window = WindowHandle(event.focus_change.window, self.display).into();
                    app.focused(window, true);
                }

                xlib::FocusOut => {
                    let window = WindowHandle(event.focus_change.window, self.display).into();
                    app.focused(window, false);
                }

                xlib::KeyPress => {
                    let ic = match ThreadContext::current_thread().window_map.get(&event.key.window) {
                        Some(ic) => *ic,
                        None => return,
                    };

                    let mut keysym = 0;
                    let mut status = 0;

                    let mut key_event_buffer = [0u8; 25];

                    let count =
                        Xutf8LookupString(ic, &mut event.key, key_event_buffer.as_mut_ptr().cast(), 24, &mut keysym, &mut status);

                    let window = WindowHandle(event.key.window, self.display).into();

                    if status == XLookupBoth || status == XLookupKeySym {
                        let mut lower = 0;
                        let mut upper = 0;
                        XConvertCase(keysym, &mut lower, &mut upper);

                        app.key_down(window, Key(lower as _), XKeysymToKeycode(self.display, keysym) as _);
                    }

                    if status == XLookupBoth || status == XLookupChars {
                        match std::str::from_utf8(&key_event_buffer[..count as usize]) {
                            Ok(c) => {
                                for c in c.chars() {
                                    app.received_character(window, c);
                                }
                            }
                            Err(e) => log_warn!("Exposed", "Failed to get char event {e}"),
                        }
                    }
                }

                xlib::KeyRelease => {
                    let ic = match ThreadContext::current_thread().window_map.get(&event.key.window) {
                        Some(ic) => *ic,
                        None => return,
                    };

                    let mut keysym = 0;
                    let mut status = 0;

                    let mut key_event_buffer = [0u8; 25];

                    let _count =
                        Xutf8LookupString(ic, &mut event.key, key_event_buffer.as_mut_ptr().cast(), 24, &mut keysym, &mut status);

                    let window = WindowHandle(event.key.window, self.display).into();

                    if status == XLookupBoth || status == XLookupKeySym {
                        let mut lower = 0;
                        let mut upper = 0;
                        XConvertCase(keysym, &mut lower, &mut upper);

                        app.key_up(window, Key(lower as _), XKeysymToKeycode(self.display, keysym) as _);
                    }
                }

                xlib::ButtonPress => {
                    let window = WindowHandle(event.button.window, self.display).into();
                    app.mouse_button_down(window, MouseButton(event.button.button));
                }

                xlib::ButtonRelease => {
                    let window = WindowHandle(event.button.window, self.display).into();
                    app.mouse_button_release(window, MouseButton(event.button.button));
                }

                xlib::MotionNotify => {
                    let window = WindowHandle(event.motion.window, self.display).into();
                    app.cursor_moved(window, event.motion.x_root, event.motion.y_root)
                }

                xlib::EnterNotify => {
                    let window = WindowHandle(event.crossing.window, self.display).into();
                    app.cursor_entered(window)
                }

                xlib::LeaveNotify => {
                    let window = WindowHandle(event.crossing.window, self.display).into();
                    app.cursor_left(window)
                }

                xlib::MapNotify => {
                    // TODO
                }

                xlib::UnmapNotify => {
                    // TODO
                }

                xlib::VisibilityNotify => {
                    // TODO
                }

                xlib::ClientMessage => {
                    if event.client_message.format == 32 {
                        if *event.client_message.data.as_longs().get_unchecked(0) == self.wm_delete as c_long {
                            let window = WindowHandle(event.focus_change.window, self.display).into();
                            app.close_requested(window);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

impl<E: Event> Into<crate::window::EventHandler<E>> for EventHandler<E> {
    fn into(self) -> crate::window::EventHandler<E> {
        crate::window::EventHandler(self)
    }
}

impl<E: Event> Destroy for EventHandler<E> {
    fn destroy(&mut self) -> Result<(), Error> {
        unsafe {
            XCloseIM(self.im);
            XCloseDisplay(self.display);
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub struct EventHandlerBuilder {}

impl EventHandlerBuilder {
    pub unsafe fn build<E: Event>(&mut self, user_data: *mut E) -> Result<EventHandler<E>, Error> {
        static mut ONCE: AtomicUsize = AtomicUsize::new(0);

        if ONCE.fetch_add(1, Ordering::SeqCst) == 0 {
            if setlocale(LC_ALL, cstr!("")).is_null() {
                return Err(ErrorKind::Other.into());
            }

            if XSupportsLocale() == 0 {
                return Err(ErrorKind::Other.into());
            }

            if XSetLocaleModifiers(cstr!("@im=none")).is_null() {
                return Err(ErrorKind::Other.into());
            }
        }

        let display = XOpenDisplay(null());
        if display.is_null() {
            return Err(ErrorKind::Other.into());
        }

        let screen = XDefaultScreenOfDisplay(display);
        if screen.is_null() {
            XCloseDisplay(display);
            return Err(ErrorKind::Other.into());
        }

        let screen_id = XDefaultScreen(display);

        let im = XOpenIM(display, null_mut(), null_mut(), null_mut());
        if im.is_null() {
            XCloseDisplay(display);
            return Err(ErrorKind::Other.into());
        }

        let wm_delete = XInternAtom(display, cstr!("WM_DELETE_WINDOW"), True);

        let thread_context = ThreadContext::current_thread();

        thread_context.display = display;
        thread_context.screen = screen;
        thread_context.screen_id = screen_id;
        thread_context.wm_delete = wm_delete;
        thread_context.im = im;

        if let Some(s) = E::create(Context(thread_context)) {
            user_data.write(s);
        } else {
            XCloseIM(im);
            XCloseDisplay(display);
            return Err(ErrorKind::Other.into());
        }

        let event_handler = EventHandler { user_data, wm_delete, screen, screen_id, im, display, event: zeroed() };

        Ok(event_handler)
    }
}
