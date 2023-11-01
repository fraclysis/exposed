use std::{ffi::c_ulong, io::Error};

use x11::xlib::XMapWindow;

use crate::{
    destroy::Destroy,
    window::{Rect, Size, Window},
};

use super::Context;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle {
    pub windowHandle: c_ulong,
    pub context: Context,
}

impl WindowHandle {}

impl Destroy for WindowHandle {
    fn destroy(&mut self) -> Result<(), Error> {
        todo!()
    }
}

impl Window for WindowHandle {
    fn show(self) -> Result<(), Error> {
        unsafe { XMapWindow(self.context.display, self.windowHandle) };

        Ok(())
    }

    fn update(self) -> Result<(), Error> {
        todo!()
    }

    fn redraw(self) -> Result<(), Error> {
        todo!()
    }

    fn window_title(self) -> Result<String, Error> {
        todo!()
    }

    fn set_window_title(self, title: &str) -> Result<(), Error> {
        todo!()
    }

    fn dpi(self) -> Result<u32, Error> {
        todo!()
    }

    fn set_capture(self) {
        todo!()
    }

    fn release_capture(self) -> Result<(), Error> {
        todo!()
    }

    fn client_size(self) -> Result<Size, Error> {
        todo!()
    }

    fn client_rect(self) -> Result<Rect, Error> {
        todo!()
    }

    fn window_rect(self) -> Result<Rect, Error> {
        todo!()
    }
}
