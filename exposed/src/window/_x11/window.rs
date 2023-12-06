use std::{
    io::{Error, ErrorKind},
    mem::zeroed,
};

use unsafe_utilities::to_ref::ToReference;

use x11::xlib::{
    self, ButtonMotionMask, ButtonPressMask, ButtonReleaseMask, EnterWindowMask, Expose, ExposureMask, FocusChangeMask,
    KeyPressMask, KeyReleaseMask, LeaveWindowMask, PointerMotionMask, ResizeRedirectMask, XBlackPixel, XClearWindow, XCreateIC,
    XCreateSimpleWindow, XDestroyIC, XDestroyWindow, XEvent, XGetWindowAttributes, XIMPreeditNothing, XIMStatusNothing,
    XMapWindow, XNClientWindow_0, XNInputStyle_0, XRootWindowOfScreen, XSelectInput, XSendEvent, XSetICFocus, XSetWMProtocols,
    XStoreName, XWhitePixel,
};

use crate::{
    destroy::Destroy,
    window::{Event, Rect, Size},
};

use super::{Context, ThreadContext};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(pub xlib::Window, pub *mut xlib::Display);

impl WindowHandle {
    pub fn show(self) -> Result<(), Error> {
        unsafe { XMapWindow(ThreadContext::current_thread().display, self.0) };
        Ok(())
    }

    pub fn update(self) -> Result<(), Error> {
        Ok(())
    }

    pub fn redraw(self) -> Result<(), Error> {
        unsafe {
            let mut e: XEvent = zeroed();
            e.type_ = Expose;
            e.expose.window = self.0;
            if XSendEvent(self.1, self.0, 0, ExposureMask, &mut e) == 0 {
                return Err(Error::new(ErrorKind::Other, "Failed at XSendEvent."));
            }
        }

        Ok(())
    }

    pub fn window_title(self) -> Result<String, Error> {
        todo!("https://tronche.com/gui/x/xlib/ICC/client-to-window-manager/XFetchName.html")
    }

    pub fn set_window_title(self, title: &str) -> Result<(), Error> {
        unsafe { XStoreName(self.1, self.0, format!("{title}\0").as_ptr().cast()) };
        Ok(())
    }

    pub fn dpi(self) -> Result<u32, Error> {
        todo!()
    }

    pub fn set_capture(self) {
        // TODO https://www.x.org/releases/current/doc/man/man3/XSelectInput.3.xhtml
    }

    pub fn release_capture(self) -> Result<(), Error> {
        // TODO https://www.x.org/releases/current/doc/man/man3/XSelectInput.3.xhtml
        Ok(())
    }

    pub fn client_size(self) -> Result<Size, Error> {
        unsafe {
            let mut attr = zeroed();
            XGetWindowAttributes(self.1, self.0, &mut attr);

            Ok(Size { width: attr.width, height: attr.height })
        }
    }

    pub fn client_rect(self) -> Result<Rect, Error> {
        todo!()
    }

    pub fn window_rect(self) -> Result<Rect, Error> {
        todo!()
    }
}

impl Into<crate::window::WindowHandle> for WindowHandle {
    fn into(self) -> crate::window::WindowHandle {
        crate::window::WindowHandle(self)
    }
}

impl Destroy for WindowHandle {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        unsafe {
            let c = ThreadContext::current_thread();

            if let Some(ic) = c.window_map.remove(&self.0) {
                XDestroyIC(ic);
            }

            XDestroyWindow(self.1, self.0);
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowBuilder {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), width: 480, height: 170 }
    }
}

impl WindowBuilder {
    pub fn with_title(&mut self, _title: &str) -> &mut Self {
        todo!()
    }

    pub fn with_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.width = width as _;
        self.height = height as _;
        self
    }

    #[inline]
    pub fn build<E: Event>(&self, context: Context) -> Result<WindowHandle, Error> {
        unsafe {
            let c = context.0.to_ref();

            let window = XCreateSimpleWindow(
                c.display,
                XRootWindowOfScreen(c.screen),
                self.x,
                self.y,
                self.width,
                self.height,
                1,
                XBlackPixel(c.display, c.screen_id),
                XWhitePixel(c.display, c.screen_id),
            );

            // TODO:(fraclysis) Check for window error

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

            XSelectInput(c.display, window, event_masks);

            XClearWindow(c.display, window);

            XSetWMProtocols(c.display, window, &mut c.wm_delete, 1);

            let ic = XCreateIC(
                c.im,
                XNInputStyle_0.as_ptr(),
                XIMPreeditNothing | XIMStatusNothing,
                XNClientWindow_0.as_ptr(),
                window,
                0usize,
            );

            XSetICFocus(ic);

            c.window_map.insert(window, ic);

            Ok(WindowHandle(window, c.display))
        }
    }
}
