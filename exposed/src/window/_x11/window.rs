use std::{ffi::c_ulong, io::Error};

use x11::xlib::{XMapWindow, _XDisplay};

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle {
    pub windowHandle: c_ulong,
    pub display: *mut _XDisplay,
}

impl WindowHandle {
    pub fn destroy(self) -> Result<(), Error> {
        todo!()
    }

    pub fn show(self) -> Result<(), Error> {
        unsafe { XMapWindow(self.display, self.windowHandle) };
        Ok(())
    }

    pub fn update(self) {}

    pub fn redraw(self) -> Result<(), Error> {
        todo!()
    }

    pub fn get_window_rect(self) -> Result<u32, Error> {
        todo!()
    }

    pub fn get_dpi(self) -> Result<u32, Error> {
        todo!()
    }
}
