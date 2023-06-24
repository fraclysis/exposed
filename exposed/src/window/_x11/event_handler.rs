use std::{
    ffi::{c_int, c_ulong},
    io::Error,
    ptr::null_mut,
};

use exposed_macro::c_str;
use libc::{c_long, printf};
use x11::xlib::{
    self, Button4, Button5, Screen, XBufferOverflow, XCheckIfEvent, XEvent, XLookupBoth,
    XLookupKeySym, XLookupString, XNextEvent, Xutf8LookupString, _XDisplay, _XIC, _XIM,
};

use crate::window::{Event, Key, MouseButton};

use super::WindowHandle;

#[repr(C)]
pub struct EventHandler<E: Event> {
    pub user_data: *mut E,
    pub display: *mut _XDisplay,
    pub screen_id: c_int,
    pub im: *mut _XIM,
    pub screen: *mut Screen,
    pub wm_delete: c_ulong,
    pub window_data: intmap::IntMap<*mut _XIC>,
    pub key_event_buffer: [i8; 25],
    pub event: XEvent,
    pub running: bool,
}

impl<E: Event> EventHandler<E> {
    pub fn destroy(&self) -> Result<(), Error> {
        Ok(())
    }

    /// If no messages are available, the return value false.
    #[inline]
    pub fn poll(&mut self) -> i32 {
        extern "C" fn predicate(
            _display: *mut _XDisplay,
            _event: *mut XEvent,
            _arg: *mut i8,
        ) -> i32 {
            1
        }

        unsafe { XCheckIfEvent(self.display, &mut self.event, Some(predicate), null_mut()) }
    }

    #[inline]
    pub fn wait(&mut self) {
        unsafe { XNextEvent(self.display, &mut self.event) };
    }

    #[inline]
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
                            display: self.display,
                        })
                    }
                }

                // xlib::ResizeRequest => {
                //     XClearArea(self.display, event.expose.window, 0, 0, 0, 0, True);

                //     app.resized(
                //         WindowHandle {
                //             windowHandle: event.resize_request.window,
                //             display: self.display,
                //         },
                //         event.resize_request.width,
                //         event.resize_request.height,
                //     );
                // }
                xlib::ConfigureNotify => {
                    app.resized(
                        WindowHandle {
                            windowHandle: event.configure.window,
                            display: event.configure.display,
                        },
                        event.configure.width,
                        event.configure.height,
                    );

                    app.moved(
                        WindowHandle {
                            windowHandle: event.configure.window,
                            display: event.configure.display,
                        },
                        event.configure.x,
                        event.configure.y,
                    );
                }

                xlib::FocusIn => app.focused(
                    WindowHandle {
                        windowHandle: event.focus_change.window,
                        display: self.display,
                    },
                    true,
                ),

                xlib::FocusOut => app.focused(
                    WindowHandle {
                        windowHandle: event.focus_change.window,
                        display: self.display,
                    },
                    false,
                ),

                xlib::KeyPress => {
                    let key = 0;
                    let character = '1';

                    let ic = *self.window_data.get(event.key.window).unwrap_unchecked();

                    let mut keysym = std::mem::zeroed();
                    let mut status = std::mem::zeroed();

                    // pub const XBufferOverflow: i32 = -1;
                    // pub const XLookupNone: i32 = 1;
                    // pub const XLookupChars: i32 = 2;
                    // pub const XLookupKeySym: i32 = 3;
                    // pub const XLookupBoth: i32 = 4;

                    let key_event = &mut event.key;
                    let count = Xutf8LookupString(
                        ic,
                        key_event,
                        self.key_event_buffer.as_mut_ptr(),
                        24,
                        &mut keysym,
                        &mut status,
                    );

                    if status == XBufferOverflow {
                        eprintln!(
                            "[{}:{}] Xutf8LookupString returned XBufferOverflow!",
                            file!(),
                            column!()
                        );
                    }

                    if count > 0 {
                        printf(c_str!(
                            "buffer: %.*s\n",
                            count,
                            self.key_event_char_buffer.as_ptr(),
                        ));
                    }

                    if status == XLookupKeySym || status == XLookupBoth {
                        println!("[{}:{}] Status: {}", file!(), column!(), status);
                    }

                    match status {
                        xlib::XLookupChars => {}
                        xlib::XLookupKeySym => {}

                        _ => (),
                    }

                    app.received_character(
                        WindowHandle {
                            windowHandle: event.key.window,
                            display: self.display,
                        },
                        character,
                    );
                    app.key_down(
                        WindowHandle {
                            windowHandle: event.key.window,
                            display: self.display,
                        },
                        Key(0),
                        0,
                    );
                }

                xlib::KeyRelease => {
                    let mut str_buffer = [0u8; 25];
                    let info = &mut self.event.key;
                    let mut key_sym = std::mem::zeroed();
                    let len = XLookupString(
                        info,
                        str_buffer.as_mut_ptr().cast(),
                        25,
                        &mut key_sym,
                        null_mut(),
                    );

                    println!("{}, {}", len, std::str::from_utf8_unchecked(&str_buffer));

                    // Xutf8LookupString(6, 5, 4, 3, 2, 1);

                    app.key_up(
                        WindowHandle {
                            windowHandle: info.window,
                            display: self.display,
                        },
                        Key(0),
                        0,
                    );
                }

                xlib::ButtonPress => {
                    let info = &mut self.event.button;
                    let app = &mut *self.user_data;
                    let window_handle = WindowHandle {
                        windowHandle: info.window,
                        display: self.display,
                    };

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
                            WindowHandle {
                                windowHandle: info.window,
                                display: self.display,
                            },
                            MouseButton(info.button),
                        )
                    }
                }

                xlib::MotionNotify => {
                    let info = &mut self.event.motion;
                    app.cursor_moved(
                        WindowHandle {
                            windowHandle: info.window,
                            display: self.display,
                        },
                        info.x_root,
                        info.y_root,
                    )
                }

                xlib::EnterNotify => {
                    let info = &mut self.event.crossing;
                    app.cursor_entered(WindowHandle {
                        windowHandle: info.window,
                        display: self.display,
                    })
                }

                xlib::LeaveNotify => {
                    let info = &mut self.event.crossing;
                    app.cursor_left(WindowHandle {
                        windowHandle: info.window,
                        display: self.display,
                    })
                }

                xlib::MapNotify => {
                    let info = &mut self.event.map;
                }

                xlib::UnmapNotify => {
                    let info = &mut self.event.map;
                }

                xlib::VisibilityNotify => {
                    let info = &mut self.event.visibility;

                    let _ = xlib::VisibilityFullyObscured
                        | xlib::VisibilityPartiallyObscured
                        | xlib::VisibilityUnobscured;
                }

                xlib::ClientMessage => {
                    if event.client_message.format == 32 {
                        if *event.client_message.data.as_longs().get_unchecked(0)
                            == self.wm_delete as c_long
                        {
                            app.close_requested(WindowHandle {
                                windowHandle: event.client_message.window,
                                display: self.display,
                            })
                        }
                    }
                }

                _ => (),
            }
        }
    }
}
