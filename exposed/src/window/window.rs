use std::io::Error;

use crate::destroy::Destroy;

use super::{platform, Context, Event, Rect, Size};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(pub platform::WindowHandle);

impl WindowHandle {
    pub fn show(self) -> Result<(), Error> {
        self.0.show()
    }

    pub fn update(self) -> Result<(), Error> {
        self.0.update()
    }

    pub fn redraw(self) -> Result<(), Error> {
        self.0.redraw()
    }

    pub fn window_title(self) -> Result<String, Error> {
        self.0.window_title()
    }

    pub fn set_window_title(self, title: &str) -> Result<(), Error> {
        self.0.set_window_title(title)
    }

    pub fn dpi(self) -> Result<u32, Error> {
        self.0.dpi()
    }

    pub fn set_capture(self) {
        self.0.set_capture()
    }

    pub fn release_capture(self) -> Result<(), Error> {
        self.0.release_capture()
    }

    pub fn client_size(self) -> Result<Size, Error> {
        self.0.client_size()
    }

    pub fn client_rect(self) -> Result<Rect, Error> {
        self.0.client_rect()
    }

    pub fn window_rect(self) -> Result<Rect, Error> {
        self.0.window_rect()
    }
}

impl Destroy for WindowHandle {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        self.0.destroy()
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowBuilder(pub platform::WindowBuilder);

impl WindowBuilder {
    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self.0.with_title(title);
        self
    }

    pub fn with_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.0.with_size(width, height);
        self
    }

    #[inline]
    pub fn build<E: Event>(&self, context: Context) -> Result<WindowHandle, Error> {
        Ok(self.0.build::<E>(context)?.into())
    }
}
