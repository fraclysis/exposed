#[cfg(target_os = "windows")]
pub mod win32_gl;
#[cfg(target_os = "windows")]
pub use win32_gl as platform;

#[cfg(target_os = "linux")]
pub mod x11_gl;
#[cfg(target_os = "linux")]
pub use x11_gl as platform;

#[cfg(target_os = "android")]
pub mod gles;
#[cfg(target_os = "android")]
pub use gles as platform;

mod picker;
pub use picker::*;

pub mod tokens;

pub use platform::{free_lib_opengl, get_proc_addr, load_lib_opengl};

use std::io::Error;

use exposed::{
    destroy::Destroy,
    window::{Context, Event, WindowBuilder, WindowHandle},
};

use platform::GlPixelFormat;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlSurface(pub platform::GlSurface);

impl GlSurface {
    pub fn swap_buffers(self) -> Result<(), Error> {
        self.0.swap_buffers()
    }

    pub fn make_current(self, context: GlContext) -> Result<(), Error> {
        self.0.make_current(context.0)
    }

    pub fn set_swap_interval(self, interval: i32) -> Result<(), Error> {
        self.0.set_swap_interval(interval)
    }

    pub fn build_with<E: Event>(
        window_builder: &WindowBuilder, context: Context, min_config: &[u32], picker: &mut impl GlConfigPicker,
    ) -> Result<(GlSurface, WindowHandle), Error> {
        let (surface, window) = platform::GlSurface::build_with::<E, _>(&window_builder.0, context, min_config, picker)?;
        Ok((surface.into(), window.into()))
    }

    pub fn build<E: Event>(window: WindowHandle, config: &[u32], picker: &mut impl GlConfigPicker) -> Result<GlSurface, Error> {
        Ok(platform::GlSurface::build::<E, _>(window.0, config, picker)?.into())
    }

    pub fn create_context(&self, config: &[u32], share_context: GlContext) -> Result<GlContext, Error> {
        Ok(self.0.create_context(config, share_context.0)?.into())
    }
}

impl Destroy for GlSurface {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        self.0.destroy()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlContext(pub platform::GlContext);

impl GlContext {
    pub const NO_CONTEXT: Self = Self(platform::GlContext::NO_CONTEXT);
}

impl Destroy for GlContext {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        self.0.destroy()
    }
}

impl From<platform::GlContext> for GlContext {
    fn from(value: platform::GlContext) -> Self {
        Self(value)
    }
}

impl From<platform::GlSurface> for GlSurface {
    fn from(value: platform::GlSurface) -> Self {
        Self(value)
    }
}

pub trait GlConfigPicker {
    fn pick(&mut self, pixel_format: GlPixelFormat) -> Option<usize>;
}
