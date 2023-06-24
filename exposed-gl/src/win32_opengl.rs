#![allow(non_snake_case)]

use std::{
    ffi::{c_char, c_void},
    io::Error,
    ptr::null,
};

use glutin_wgl_sys::wgl_extra;
use windows_sys::Win32::{
    Graphics::{
        Gdi::{GetDC, HDC},
        OpenGL::{
            wglDeleteContext, wglMakeCurrent, DescribePixelFormat, SetPixelFormat, SwapBuffers,
            HGLRC, PIXELFORMATDESCRIPTOR,
        },
    },
    System::LibraryLoader::GetProcAddress,
};

use exposed::window::{Event, EventHandler, WindowBuilder, WindowHandle};

use crate::wgl::{LIB_OPENGL, WGL};

#[repr(C)]
pub struct GlDisplayBuilder {
    pub pixelFormatAttribs: *const i32,
}

impl Default for GlDisplayBuilder {
    fn default() -> Self {
        Self {
            pixelFormatAttribs: {
                const GL_TRUE: u32 = 1;

                #[rustfmt::skip]
                static PIXEL_FORMAT_ATTRIBS: [u32; 17] = [
                    wgl_extra::DRAW_TO_WINDOW_ARB,     GL_TRUE,
                    wgl_extra::SUPPORT_OPENGL_ARB,     GL_TRUE,
                    wgl_extra::DOUBLE_BUFFER_ARB,      GL_TRUE,
                    wgl_extra::ACCELERATION_ARB,       wgl_extra::FULL_ACCELERATION_ARB,
                    wgl_extra::PIXEL_TYPE_ARB,         wgl_extra::TYPE_RGBA_ARB,
                    wgl_extra::COLOR_BITS_ARB,         32,
                    wgl_extra::DEPTH_BITS_ARB,         24,
                    wgl_extra::STENCIL_BITS_ARB,       8,
                    0
                ];

                PIXEL_FORMAT_ATTRIBS.as_ptr() as *const i32
            },
        }
    }
}

impl GlDisplayBuilder {
    pub fn build_with<E: Event>(
        &self,
        window_builder: WindowBuilder,
        event_handler: &mut EventHandler<E>,
    ) -> Result<(GlDisplay, WindowHandle), Error> {
        let window = window_builder.build(event_handler)?;
        let dispay = self.build(window, event_handler)?;

        Ok((dispay, window))
    }

    pub fn build<E: Event>(
        &self,
        window: WindowHandle,
        _event_handler: &mut EventHandler<E>,
    ) -> Result<GlDisplay, Error> {
        if unsafe { WGL }.is_null() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Todo wgl load error.",
            ));
        }

        let wgl = unsafe { &*WGL };

        let real_dc = unsafe { GetDC(window.windowHandle) };
        if real_dc == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "GetDc failed to get DC for HWND.",
            ));
        }

        if !wgl.ChoosePixelFormatARB.is_loaded() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Could not found ChoosePixelFormatARB!",
            ));
        }

        if !wgl.CreateContextAttribsARB.is_loaded() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Could not found CreateContextAttribsARB!",
            ));
        }

        let mut pixel_format: i32 = 0;
        let mut num_formats: u32 = 0;

        unsafe {
            wgl.ChoosePixelFormatARB(
                real_dc as _,
                self.pixelFormatAttribs,
                null(),
                1,
                &mut pixel_format,
                &mut num_formats,
            )
        };

        if num_formats == 0 {
            return Err(Error::last_os_error());
        }

        let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
        if unsafe {
            DescribePixelFormat(
                real_dc,
                pixel_format,
                std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
                &mut pfd,
            )
        } == 0
        {
            return Err(Error::last_os_error());
        }

        if unsafe { SetPixelFormat(real_dc, pixel_format, &pfd) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(GlDisplay { dc: real_dc })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GlContextBuilder {
    pub pixelContextAttribs: *const i32,
    pub defPixelContextAttribs: [u32; 9],
}

impl Default for GlContextBuilder {
    fn default() -> Self {
        Self {
            pixelContextAttribs: null(),
            defPixelContextAttribs: [
                wgl_extra::CONTEXT_MAJOR_VERSION_ARB,
                3,
                wgl_extra::CONTEXT_MINOR_VERSION_ARB,
                0,
                wgl_extra::CONTEXT_PROFILE_MASK_ARB,
                wgl_extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                wgl_extra::CONTEXT_FLAGS_ARB,
                0,
                0,
            ],
        }
    }
}

impl GlContextBuilder {
    // !! WARN could cause problems when editing
    pub fn with_version(&mut self, major: i32, minor: i32) {
        self.defPixelContextAttribs[1] = major as u32;
        self.defPixelContextAttribs[3] = minor as u32;
    }

    pub fn with_debug(&mut self) {
        self.defPixelContextAttribs[7] |= wgl_extra::CONTEXT_DEBUG_BIT_ARB;
    }

    pub fn build(&self, display: GlDisplay, share_context: GlContext) -> Result<GlContext, Error> {
        unsafe {
            debug_assert!(!WGL.is_null());
            let wgl = &*WGL;

            let pixel_context_attribs = if self.pixelContextAttribs.is_null() {
                self.defPixelContextAttribs.as_ptr() as *const i32
            } else {
                self.pixelContextAttribs
            };

            let gl_context = wgl.CreateContextAttribsARB(
                display.dc as _,
                share_context.context as _,
                pixel_context_attribs,
            ) as isize;

            if gl_context == 0 {
                return Err(Error::last_os_error());
            }

            Ok(GlContext {
                context: gl_context,
            })
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlDisplay {
    pub dc: HDC,
}

impl GlDisplay {
    pub fn swap_buffers(self) -> Result<(), Error> {
        if unsafe { SwapBuffers(self.dc) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub unsafe fn set_swap_interval(self, interval: i32) -> Result<(), Error> {
        unsafe {
            debug_assert!(!WGL.is_null());

            let wgl = &*WGL;
            if wgl.SwapIntervalEXT.is_loaded() {
                if wgl.SwapIntervalEXT(interval) == 0 {
                    Err(Error::last_os_error())
                } else {
                    Ok(())
                }
            } else {
                Err(Error::new(
                    std::io::ErrorKind::Other,
                    "wglSwapIntervalEXT is not loaded.",
                ))
            }
        }
    }

    pub fn make_current(self, context: GlContext) -> Result<(), Error> {
        if unsafe { wglMakeCurrent(self.dc, context.context) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn destroy(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlContext {
    pub context: HGLRC,
}

impl GlContext {
    pub const NO_CONTEXT: Self = Self { context: 0 };

    pub fn destroy(self) -> Result<(), Error> {
        if unsafe { wglDeleteContext(self.context) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// # Safety
/// Requires and OpenGl context to be active
pub unsafe fn get_proc_addr(symbol: *const c_char) -> *const c_void {
    debug_assert!(!WGL.is_null());
    debug_assert!(LIB_OPENGL != 0);

    if let Some(pfn) = GetProcAddress(LIB_OPENGL, symbol as *const u8) {
        return pfn as _;
    }

    let wgl = &*WGL;
    wgl.GetProcAddress(symbol).cast()
}
