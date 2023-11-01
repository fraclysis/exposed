use std::{
    alloc::Layout,
    ffi::{c_int, c_ulong},
    io::Error,
    ptr::null_mut,
};

use libc::{c_long, c_void};
use x11::xlib::{
    self, Button4, Button5, Display, Screen, XBufferOverflow, XCheckIfEvent, XCloseDisplay, XEvent, XKeycodeToKeysym,
    XLookupBoth, XLookupChars, XLookupKeySym, XNextEvent, Xutf8LookupString, _XDisplay, XIM, _XIC,
};

use crate::{
    destroy::Destroy,
    window::{Event, Key, MouseButton},
};

use super::{Context, WindowHandle};

const XKEY_RELEASE_LOOKUP_FROM_UTF8: bool = false;

pub use intmap;

#[repr(C)]
pub struct EventHandler<E: Event> {
    pub user_data: *mut E,
    pub display: *mut Display,
    pub screen_id: c_int,
    pub im: XIM,
    pub screen: *mut Screen,
    pub wm_delete: c_ulong,
    pub window_data: *mut c_void, /* intmap::IntMap<*mut _XIC> */
    pub key_event_buffer: [u8; 25],
    pub event: XEvent,
    pub running: bool,
}

impl<E: Event> Destroy for EventHandler<E> {
    fn destroy(&mut self) -> Result<(), Error> {
        if !self.window_data.is_null() {
            unsafe {
                let window_data = self.window_data as *mut intmap::IntMap<*mut _XIC>;
                self.window_data.drop_in_place();

                std::alloc::dealloc(window_data.cast(), Layout::new::<intmap::IntMap<*mut _XIC>>())
            };

            self.window_data = null_mut();
        }

        unsafe { XCloseDisplay(self.display) };
        Ok(())
    }
}

impl<E: Event> EventHandler<E> {
    pub fn poll(&mut self) -> i32 {
        extern "C" fn predicate(_display: *mut _XDisplay, _event: *mut XEvent, _arg: *mut i8) -> i32 {
            1
        }

        unsafe { XCheckIfEvent(self.display, &mut self.event, Some(predicate), null_mut()) }
    }

    pub fn wait(&mut self) {
        unsafe { XNextEvent(self.display, &mut self.event) };
    }

    pub fn dispatch(&mut self) {
        unsafe {
            let app = &mut *self.user_data;
            let event = &mut self.event;

            match event.type_ {
                xlib::Expose => {
                    println!("Resize {}", event.expose.count);
                    if event.expose.count == 0 {
                        app.low_render(WindowHandle {
                            windowHandle: event.expose.window,
                            context: Context { display: self.display },
                        })
                    }
                }

                xlib::ConfigureNotify => {
                    app.resized(
                        WindowHandle { windowHandle: event.configure.window, context: Context { display: self.display }},
                        event.configure.width,
                        event.configure.height,
                    );

                    app.moved(
                        WindowHandle { windowHandle: event.configure.window, context: Context { display: self.display } },
                        event.configure.x,
                        event.configure.y,
                    );
                }

                xlib::FocusIn => app.focused(
                    WindowHandle { windowHandle: event.focus_change.window, context: Context { display: self.display } },
                    true,
                ),

                xlib::FocusOut => app.focused(
                    WindowHandle { windowHandle: event.focus_change.window, context: Context { display: self.display } },
                    false,
                ),

                // TODO
                xlib::KeyPress => {
                    let ic =
                        *(&mut *(self.window_data as *mut intmap::IntMap<*mut _XIC>)).get(event.key.window).unwrap_unchecked();

                    let mut keysym = 0;
                    let mut status = 0;

                    // pub const XBufferOverflow: i32 = -1;
                    // pub const XLookupNone: i32 = 1;
                    // pub const XLookupChars: i32 = 2;
                    // pub const XLookupKeySym: i32 = 3;
                    // pub const XLookupBoth: i32 = 4;

                    let key_event = &mut event.key;
                    let count =
                        Xutf8LookupString(ic, key_event, self.key_event_buffer.as_mut_ptr().cast(), 24, &mut keysym, &mut status);

                    if status == XBufferOverflow {
                        // TODO report for buffer size increase
                        eprintln!("[{}:{}] Xutf8LookupString returned XBufferOverflow!", file!(), column!());

                        return;
                    }

                    match status {
                        xlib::XLookupChars => {}
                        xlib::XLookupKeySym => {}
                        _ => (),
                    }

                    if (status == XLookupBoth) | (status == XLookupKeySym) {
                        // TODO send send key sym
                        app.key_down(
                            WindowHandle { windowHandle: event.key.window, context: Context { display: self.display } },
                            Key(keysym as _), // TODO keysym is 64bit wide key is only 32bit
                            event.key.keycode,
                        );
                    }

                    if (status == XLookupBoth) | (status == XLookupChars) {
                        for c in std::str::from_utf8_unchecked(&self.key_event_buffer[..count as usize]).chars() {
                            app.received_character(
                                WindowHandle { windowHandle: event.key.window, context: Context { display: self.display } },
                                c,
                            );
                        }
                    }
                }

                xlib::KeyRelease => {
                    let keysym = if XKEY_RELEASE_LOOKUP_FROM_UTF8 {
                        // Xutf8LookupString(6, 5, 4, 3, 2, 1);
                        0
                    } else {
                        XKeycodeToKeysym(self.display, event.key.keycode as u8, 0)
                    };

                    app.key_up(
                        WindowHandle { windowHandle: event.key.window, context: Context { display: self.display } },
                        Key(keysym as _), // TODO keysym is 64bit wide key is only 32bit
                        event.key.keycode,
                    );
                }

                xlib::ButtonPress => {
                    let info = &mut self.event.button;
                    let app = &mut *self.user_data;
                    let window_handle = WindowHandle { windowHandle: info.window, context: Context { display: self.display } };

                    if info.button == Button4 {
                        app.mouse_wheel(window_handle, 1.0, 0.0)
                    } else if info.button == Button5 {
                        app.mouse_wheel(window_handle, -1.0, 0.0)
                    } else {
                        app.mouse_button_down(window_handle, MouseButton(info.button))
                    }
                }

                xlib::ButtonRelease => {
                    let info = &mut self.event.button;

                    if !(info.button == Button4 || info.button == Button5) {
                        app.mouse_button_release(
                            WindowHandle { windowHandle: info.window, context: Context { display: self.display } },
                            MouseButton(info.button),
                        )
                    }
                }

                xlib::MotionNotify => {
                    let info = &mut self.event.motion;
                    app.cursor_moved(
                        WindowHandle { windowHandle: info.window, context: Context { display: self.display } },
                        info.x_root,
                        info.y_root,
                    )
                }

                xlib::EnterNotify => {
                    let info = &mut self.event.crossing;
                    app.cursor_entered(WindowHandle { windowHandle: info.window, context: Context { display: self.display } })
                }

                xlib::LeaveNotify => {
                    let info = &mut self.event.crossing;
                    app.cursor_left(WindowHandle { windowHandle: info.window, context: Context { display: self.display } })
                }

                xlib::MapNotify => {}

                xlib::UnmapNotify => {}

                xlib::VisibilityNotify => {}

                xlib::ClientMessage => {
                    if event.client_message.format == 32 {
                        if *event.client_message.data.as_longs().get_unchecked(0) == self.wm_delete as c_long {
                            app.close_requested(WindowHandle {
                                windowHandle: event.client_message.window,
                                context: Context { display: self.display },
                            })
                        }
                    }
                }

                _ => (),
            }
        }
    }
}
