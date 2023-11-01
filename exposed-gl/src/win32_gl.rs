#![allow(non_snake_case)]

use std::{
    alloc::{dealloc, handle_alloc_error, Layout},
    ffi::{c_char, c_void},
    io::{Error, ErrorKind},
    mem::zeroed,
    ptr::{null, null_mut},
};

use glutin_wgl_sys::wgl_extra::{self, Wgl};
use std::alloc::alloc;
use windows_sys::{
    w,
    Win32::{
        Foundation::{
            GetLastError, ERROR_DC_NOT_FOUND, ERROR_INVALID_OPERATION, ERROR_INVALID_PARAMETER, ERROR_INVALID_PIXEL_FORMAT,
            ERROR_NO_SYSTEM_RESOURCES, HMODULE,
        },
        Graphics::{
            Gdi::{GetDC, ReleaseDC, HDC},
            OpenGL::{
                wglCreateContext, wglDeleteContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat, DescribePixelFormat,
                SetPixelFormat, SwapBuffers, HGLRC, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL,
                PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
            },
        },
        System::LibraryLoader::{FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryW},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, UnregisterClassW, CS_OWNDC, CW_USEDEFAULT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW,
        },
    },
};

use exposed::{
    destroy::{Destroy, Destroyable},
    window::{
        platform::{WindowBuilder, WindowHandle},
        Context, Event,
    },
};

use crate::GlConfigPicker;

pub use glutin_wgl_sys as pgl;

pub use pgl::wgl_extra as pgl_extra;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlSurfaceBuilder {
    pub pixelFormatAttribs: *const i32,
}

impl Default for GlSurfaceBuilder {
    fn default() -> Self {
        Self {
            pixelFormatAttribs: {
                const GL_TRUE: u32 = 1;

                #[rustfmt::skip]
                static PIXEL_FORMAT_ATTRIBS: [u32; 19] = [
                    wgl_extra::DRAW_TO_WINDOW_ARB,     GL_TRUE,
                    wgl_extra::SUPPORT_OPENGL_ARB,     GL_TRUE,
                    wgl_extra::DOUBLE_BUFFER_ARB,      GL_TRUE,
                    wgl_extra::ACCELERATION_ARB,       wgl_extra::FULL_ACCELERATION_ARB,
                    wgl_extra::PIXEL_TYPE_ARB,         wgl_extra::TYPE_RGBA_ARB,
                    wgl_extra::COLOR_BITS_ARB,         32,
                    wgl_extra::DEPTH_BITS_ARB,         24,
                    wgl_extra::STENCIL_BITS_ARB,       8,
                    wgl_extra::SAMPLES_ARB,            4,
                    0
                ];

                PIXEL_FORMAT_ATTRIBS.as_ptr() as *const i32
            },
        }
    }
}

impl GlSurfaceBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_with<E: Event>(
        &self, context: Context, window_builder: &WindowBuilder,
    ) -> Result<(GlSurface, WindowHandle), Error> {
        let window = window_builder.build::<E>(context)?;
        let display = self.build::<E>(window)?;

        Ok((display, window))
    }

    pub fn build<E: Event>(&self, window: WindowHandle) -> Result<GlSurface, Error> {
        if unsafe { WGL }.is_null() {
            return Err(Error::new(std::io::ErrorKind::Other, "Todo wgl load error."));
        }

        let wgl = unsafe { &*WGL };

        let real_dc = unsafe { GetDC(window.0) };
        if real_dc == 0 {
            return Err(Error::new(std::io::ErrorKind::Other, "GetDc failed to get DC for HWND."));
        }

        if !wgl.ChoosePixelFormatARB.is_loaded() {
            return Err(Error::new(std::io::ErrorKind::Other, "Could not found ChoosePixelFormatARB!"));
        }

        if !wgl.CreateContextAttribsARB.is_loaded() {
            return Err(Error::new(std::io::ErrorKind::Other, "Could not found CreateContextAttribsARB!"));
        }

        let mut pixel_format: i32 = 0;
        let mut num_formats: u32 = 0;

        unsafe {
            wgl.ChoosePixelFormatARB(real_dc as _, self.pixelFormatAttribs, null(), 1, &mut pixel_format, &mut num_formats)
        };

        if num_formats == 0 {
            return Err(Error::last_os_error());
        }

        let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
        if unsafe { DescribePixelFormat(real_dc, pixel_format, std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _, &mut pfd) } == 0
        {
            return Err(Error::last_os_error());
        }

        if unsafe { SetPixelFormat(real_dc, pixel_format, &pfd) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(GlSurface { dc: real_dc })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlContextBuilder {
    pub debug: u32,
    pub major: u32,
    pub minor: u32,
}

impl Default for GlContextBuilder {
    fn default() -> Self {
        Self { debug: wgl_extra::CONTEXT_DEBUG_BIT_ARB, major: 4, minor: 3 }
    }
}

impl GlContextBuilder {
    // !! WARN could cause problems when editing
    pub fn with_version(&mut self, major: u32, minor: u32) -> &mut Self {
        self.major = major;
        self.minor = minor;
        self
    }

    pub fn with_debug(&mut self) -> &mut Self {
        self.debug |= wgl_extra::CONTEXT_DEBUG_BIT_ARB;
        self
    }

    pub fn build(&self, display: GlSurface, share_context: GlContext) -> Result<GlContext, Error> {
        unsafe {
            debug_assert!(!WGL.is_null());
            let wgl = &*WGL;

            let pixel_context_attribs = [
                wgl_extra::CONTEXT_MAJOR_VERSION_ARB,
                self.major,
                wgl_extra::CONTEXT_MINOR_VERSION_ARB,
                self.minor,
                wgl_extra::CONTEXT_PROFILE_MASK_ARB,
                wgl_extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                wgl_extra::CONTEXT_FLAGS_ARB,
                self.debug,
                0,
            ];

            let gl_context =
                wgl.CreateContextAttribsARB(display.dc as _, share_context.context as _, pixel_context_attribs.as_ptr().cast())
                    as isize;

            if gl_context == 0 {
                println!("{}", GetLastError());
                match GetLastError() {
                    ERROR_INVALID_OPERATION => return Err(Error::new(std::io::ErrorKind::Other, "ERROR_INVALID_OPERATION")),
                    ERROR_DC_NOT_FOUND => return Err(Error::new(std::io::ErrorKind::Other, "ERROR_DC_NOT_FOUND")),
                    ERROR_INVALID_PIXEL_FORMAT => {
                        return Err(Error::new(std::io::ErrorKind::Other, "ERROR_INVALID_PIXEL_FORMAT"))
                    }
                    ERROR_NO_SYSTEM_RESOURCES => return Err(Error::new(std::io::ErrorKind::Other, "ERROR_NO_SYSTEM_RESOURCES")),
                    ERROR_INVALID_PARAMETER => return Err(Error::new(std::io::ErrorKind::Other, "ERROR_INVALID_PARAMETER")),

                    e => return Err(Error::new(std::io::ErrorKind::Other, format!("Unknown error {e}, {e:#X}"))),
                }
            }

            Ok(GlContext { context: gl_context })
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlSurface {
    pub dc: HDC,
}

impl GlSurface {
    pub fn build_with<E: Event, P: GlConfigPicker>(
        window_builder: &WindowBuilder, context: Context, min_config: &[u32], picker: &mut P,
    ) -> Result<(GlSurface, WindowHandle), Error> {
        let window = Destroyable(window_builder.build::<E>(context)?);
        let surface = Self::build::<E, P>(window.0, min_config, picker)?;

        Ok((surface, unsafe { window.into_inner() }))
    }

    pub fn build<E: Event, P: GlConfigPicker>(window: WindowHandle, config: &[u32], picker: &mut P) -> Result<GlSurface, Error> {
        let wgl = get_wgl()?;

        let dc = unsafe { GetDC(window.0) };
        if dc == 0 {
            return Err(ErrorKind::Other.into());
        }

        let mut num_pixel_format = 0;
        let attr = wgl_extra::NUMBER_PIXEL_FORMATS_ARB as i32;

        if unsafe { wgl.GetPixelFormatAttribivARB(dc as _, 0, 0, 1, &attr, &mut num_pixel_format) } == 0 {
            return Err(Error::last_os_error());
        }

        let mut pixel_formats: Vec<i32> = Vec::with_capacity(num_pixel_format as _);

        let conf = if config.len() == 0 { null() } else { config.as_ptr() };

        let mut pixel_formats_len = 0;
        if unsafe {
            wgl.ChoosePixelFormatARB(
                dc as _,
                conf.cast(),
                null(),
                num_pixel_format as _,
                pixel_formats.as_mut_ptr(),
                &mut pixel_formats_len,
            )
        } == 0
        {
            return Err(Error::last_os_error());
        }

        unsafe { pixel_formats.set_len(pixel_formats_len as _) };

        let mut picked_format = None;

        for pixel_format in pixel_formats {
            let pixel_format = GlPixelFormat { hdc: dc, format: pixel_format as _ };
            if let Some(c) = picker.pick(pixel_format) {
                picked_format = Some(c);
            }
        }

        if let Some(picked_format) = picked_format {
            let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
            if unsafe { DescribePixelFormat(dc, picked_format as _, std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _, &mut pfd) }
                == 0
            {
                return Err(Error::last_os_error());
            }

            if unsafe { SetPixelFormat(dc, picked_format as _, &pfd) } == 0 {
                return Err(Error::last_os_error());
            }

            Ok(GlSurface { dc })
        } else {
            todo!()
        }
    }

    pub fn create_context(&self, config: &[u32], share_context: GlContext) -> Result<GlContext, Error> {
        let wgl = get_wgl()?;

        let context = unsafe { wgl.CreateContextAttribsARB(self.dc as _, share_context.context as _, config.as_ptr().cast()) };
        if context.is_null() {
            return Err(Error::last_os_error());
        }

        Ok(GlContext { context: context as _ })
    }

    pub fn swap_buffers(self) -> Result<(), Error> {
        if unsafe { SwapBuffers(self.dc) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn set_swap_interval(self, interval: i32) -> Result<(), Error> {
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
                Err(Error::new(std::io::ErrorKind::Other, "wglSwapIntervalEXT is not loaded."))
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
}

impl Destroy for GlSurface {
    fn destroy(&mut self) -> Result<(), Error> {
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
}

impl Destroy for GlContext {
    fn destroy(&mut self) -> Result<(), Error> {
        if unsafe { wglDeleteContext(self.context) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
pub struct GlPixelFormat {
    pub hdc: HDC,
    pub format: usize,
}

impl GlPixelFormat {
    pub fn get<const C: usize>(&self, attributes: &[u32; C], values: &mut [i32; C]) -> Result<(), Error> {
        let wgl = get_wgl()?;

        if unsafe {
            wgl.GetPixelFormatAttribivARB(
                self.hdc as _,
                self.format as _,
                0,
                C as _,
                attributes.as_ptr().cast(),
                values.as_mut_ptr(),
            )
        } == 0
        {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}

pub fn get_wgl() -> Result<&'static mut Wgl, Error> {
    unsafe {
        if !WGL.is_null() {
            Ok(&mut *WGL)
        } else {
            Err(Error::new(ErrorKind::NotFound, "First initialize WGL with load_lib_opengl."))
        }
    }
}

const DUMMY_CLASS_NAME: *const u16 = w!("Dumb Dumb");
const DUMMY_WINDOW_NAME: *const u16 = w!("Dumb Dumb win");
pub static mut OPENGL_DLL_NAME: *const u16 = w!("opengl32.dll");

const WGL_LAYOUT: Layout = Layout::new::<Wgl>();

pub static mut WGL: *mut Wgl = null_mut();

pub static mut LIB_OPENGL: HMODULE = 0;

pub fn load_lib_opengl() -> Result<(), Error> {
    unsafe {
        if LIB_OPENGL == 0 {
            LIB_OPENGL = LoadLibraryW(OPENGL_DLL_NAME);
            if LIB_OPENGL == 0 {
                return Err(Error::last_os_error());
            }
        } else {
            eprintln!("LIB_OPENGL is already loaded.")
        }
    }

    if unsafe { WGL.is_null() } {
        let h_instance = unsafe { GetModuleHandleW(null()) };

        let wc: WNDCLASSW = WNDCLASSW {
            style: CS_OWNDC,
            lpfnWndProc: Some(DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: 0,
            hCursor: 0,
            hbrBackground: 0,
            lpszMenuName: null(),
            lpszClassName: DUMMY_CLASS_NAME,
        };

        if unsafe { RegisterClassW(&wc) } == 0 {
            return Err(Error::last_os_error());
        }

        let dummy_window = unsafe {
            CreateWindowExW(
                0,
                DUMMY_CLASS_NAME,
                DUMMY_WINDOW_NAME,
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                h_instance,
                null(),
            )
        };

        if dummy_window == 0 {
            return Err(Error::last_os_error());
        }

        let dummy_dc = unsafe { GetDC(dummy_window) };

        if dummy_dc == 0 {
            return Err(Error::new(std::io::ErrorKind::Other, format!("[{}:{}]Failed to get dummy DC", file!(), line!())));
        }

        let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { zeroed() };
        pfd.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _;
        pfd.nVersion = 1;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.cColorBits = 32;
        pfd.iLayerType = PFD_MAIN_PLANE;
        pfd.cDepthBits = 24;
        pfd.cStencilBits = 8;

        let pixel_format = unsafe { ChoosePixelFormat(dummy_dc, &pfd) };
        if pixel_format == 0 {
            return Err(Error::last_os_error());
        }

        if unsafe { SetPixelFormat(dummy_dc, pixel_format, &pfd) } == 0 {
            return Err(Error::new(std::io::ErrorKind::Other, format!("[{}:{}] Failed to set pixel format.", file!(), line!())));
        }

        let dummy_context = unsafe { wglCreateContext(dummy_dc) };

        if dummy_context == 0 {
            return Err(Error::last_os_error());
        }

        if unsafe { wglMakeCurrent(dummy_dc, dummy_context) } == 0 {
            return Err(Error::last_os_error());
        }

        unsafe {
            WGL = alloc(WGL_LAYOUT).cast();
            if WGL.is_null() {
                handle_alloc_error(WGL_LAYOUT)
            }
        }

        unsafe { assert!(LIB_OPENGL != 0) };

        // TODO: remove null symbol pushing
        unsafe {
            *WGL = Wgl::load_with(|mut symbol| {
                if symbol == "wglUseFontBitmaps" {
                    symbol = "wglUseFontBitmapsW"
                }
                if symbol == "wglUseFontOutlines" {
                    symbol = "wglUseFontOutlinesW"
                }

                let mut c_symbol = symbol.to_string();
                c_symbol.push('\0');

                if let Some(pfn) = GetProcAddress(LIB_OPENGL, c_symbol.as_ptr()) {
                    return pfn as _;
                }

                if let Some(pfn) = wglGetProcAddress(c_symbol.as_ptr()) {
                    return pfn as _;
                }

                null()
            });
        }

        // TODO fix early returning before the cleanup

        if unsafe { wglMakeCurrent(dummy_dc, 0) } == 0 {
            return Err(Error::last_os_error());
        }

        if unsafe { wglDeleteContext(dummy_context) } == 0 {
            return Err(Error::last_os_error());
        }
        if unsafe { ReleaseDC(dummy_window, dummy_dc) } == 0 {
            return Err(Error::last_os_error());
        }
        if unsafe { DestroyWindow(dummy_window) } == 0 {
            return Err(Error::last_os_error());
        }
        if unsafe { UnregisterClassW(DUMMY_CLASS_NAME, h_instance) } == 0 {
            return Err(Error::last_os_error());
        }
    } else {
        eprintln!("WGL is already loaded.")
    }

    Ok(())
}

pub fn free_lib_opengl() -> Result<(), Error> {
    if unsafe { WGL.is_null() } {
        eprintln!("Static Egl library is already deallocated.");
    } else {
        unsafe { dealloc(WGL as *mut u8, WGL_LAYOUT) };
        unsafe { WGL = null_mut() };
    }

    if unsafe { LIB_OPENGL } == 0 {
        eprintln!("Static Gl library is already deallocated.");
    } else {
        if unsafe { FreeLibrary(LIB_OPENGL) } == 0 {
            return Err(Error::last_os_error());
        }
        unsafe { LIB_OPENGL = 0 };
    }
    Ok(())
}

pub unsafe fn get_proc_addr(symbol: *const c_char) -> *const c_void {
    debug_assert!(!WGL.is_null());
    debug_assert!(LIB_OPENGL != 0);

    if let Some(pfn) = GetProcAddress(LIB_OPENGL, symbol as *const u8) {
        return pfn as _;
    }

    let wgl = &*WGL;
    wgl.GetProcAddress(symbol).cast()
}
