use std::io::{Error, ErrorKind::Other};

use x11::xlib::{
    ButtonMotionMask, ButtonPressMask, ButtonReleaseMask, EnterWindowMask, FocusChangeMask,
    KeyPressMask, KeyReleaseMask, LeaveWindowMask, PointerMotionMask, ResizeRedirectMask,
    XBlackPixel, XClearWindow, XCreateIC, XCreateSimpleWindow, XIMPreeditNothing, XIMStatusNothing,
    XMapRaised, XNClientWindow_0, XNInputStyle_0, XRootWindowOfScreen, XSelectInput, XSetICFocus,
    XSetWMProtocols, XWhitePixel, _XIC,
};

use crate::window::Event;

use super::{EventHandler, WindowHandle};

#[repr(C)]
#[allow(non_snake_case)]
pub struct WindowBuilder {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        }
    }
}

impl WindowBuilder {
    #[allow(unreachable_code, unused)]
    pub fn build<E: Event>(
        &self,
        event_handler: &mut EventHandler<E>,
    ) -> Result<WindowHandle, Error> {
        return Err(Error::new(
            Other,
            "Creating a x11 window with build is not supported for now.",
        ));
        unsafe {
            let window = XCreateSimpleWindow(
                event_handler.display,
                XRootWindowOfScreen(event_handler.screen),
                0,
                0,
                320,
                200,
                1,
                XBlackPixel(event_handler.display, event_handler.screen_id),
                XWhitePixel(event_handler.display, event_handler.screen_id),
            );

            let event_masks = KeyPressMask
                | KeyReleaseMask
                | FocusChangeMask
                | ResizeRedirectMask
                | PointerMotionMask
                | ButtonMotionMask
                | ButtonPressMask
                | ButtonReleaseMask
                | EnterWindowMask
                | LeaveWindowMask;

            XSelectInput(event_handler.display, window, event_masks);

            XClearWindow(event_handler.display, window);

            XSetWMProtocols(
                event_handler.display,
                window,
                &mut event_handler.wm_delete,
                1,
            );

            let ic: *mut _XIC = XCreateIC(
                event_handler.im,
                XNInputStyle_0.as_ptr(),
                XIMPreeditNothing | XIMStatusNothing,
                XNClientWindow_0.as_ptr(),
                window,
                0usize,
            );

            XSetICFocus(ic);

            XMapRaised(event_handler.display, window);

            event_handler.window_data.insert(window, ic);

            Ok(WindowHandle {
                windowHandle: window,
                display: event_handler.display,
            })
        }
    }

    #[allow(unused)]
    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self
    }

    #[allow(unused)]
    pub fn with_size(&mut self, width: i32, height: i32) -> &mut Self {
        self
    }
}
