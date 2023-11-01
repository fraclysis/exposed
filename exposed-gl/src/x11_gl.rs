use glutin_glx_sys as glx;
use glx::{
    glx_extra::{
        types::{Display, GLXFBConfig},
        MAX_SWAP_INTERVAL_EXT, SWAP_INTERVAL_EXT,
    },
    AllocNone, InputOutput, StructureNotifyMask, XIMStatusNothing, XNClientWindow_0,
};
use std::{
    alloc::{alloc, dealloc, Layout},
    ffi::{c_char, c_ulong},
    io::{
        Error,
        ErrorKind::{self, Other},
    },
    ptr::{null, null_mut},
};

use x11::xlib::{
    ButtonMotionMask, ButtonPressMask, ButtonReleaseMask, CWBorderPixel, CWColormap, CWEventMask, EnterWindowMask, ExposureMask,
    FocusChangeMask, KeyPressMask, KeyReleaseMask, LeaveWindowMask, PointerMotionMask, True, XBlackPixel, XCreateColormap,
    XCreateIC, XCreateWindow, XFree, XIMPreeditNothing, XNInputStyle_0, XRootWindow, XSetICFocus, XSetWindowAttributes,
    XWhitePixel,
};

use glutin_glx_sys::glx_extra::Glx;
use libc::{c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};

use exposed::{
    destroy::Destroy,
    unsafe_utilities::broke_checker::AsReference,
    window::{
        platform::{WindowBuilder, WindowHandle},
        Context, Event,
    },
};

pub use glutin_glx_sys::glx_extra as pgl_extra;

use crate::GlConfigPicker;

macro_rules! error {
    ($($arg:tt)*) => {
        Err(Error::new(ErrorKind::Other, format!($($arg)*)))
    };
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlContext {
    pub context: usize,
}

impl Destroy for GlContext {
    fn destroy(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl GlContext {
    pub const NO_CONTEXT: Self = Self { context: 0 };
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlSurface {
    pub display: *mut Display,
    pub window: c_ulong,
    pub config: GLXFBConfig,
}

impl GlSurface {
    pub fn build_with<E: Event, P: GlConfigPicker>(
        window_builder: &WindowBuilder, context: Context, config: &[u32], picker: &mut P,
    ) -> Result<(GlSurface, WindowHandle), Error> {
        let c = unsafe { context.0.to_ref() };
        let glx = get_glx()?;

        let display = c.display.cast();

        let mut major = 0;
        let mut minor = 0;

        if unsafe { glx.QueryVersion(display, &mut major, &mut minor) } == 0 {
            return error!("glxQueryVersion returned an error");
        }

        let mut config_size = 0;
        let fb_config = unsafe { glx.ChooseFBConfig(display, c.screen_id, config.as_ptr().cast(), &mut config_size) };

        let mut picked_config = None;
        for i in 0..config_size as usize {
            if let Some(config) = picker.pick(GlPixelFormat { display, format: unsafe { *fb_config.add(i) } as usize }) {
                picked_config = Some(config);
            }
        }
        unsafe { XFree(fb_config.cast()) };

        if let Some(config) = picked_config {
            let visual = unsafe { glx.GetVisualFromFBConfig(display, config as _) };
            if visual.is_null() {
                todo!()
            }

            if c.screen_id != unsafe { visual.to_ref().screen } {
                todo!()
            }

            let root_window = unsafe { XRootWindow(c.display, c.screen_id) };

            let mut window_attrib = unsafe {
                XSetWindowAttributes {
                    border_pixel: XBlackPixel(c.display, c.screen_id),
                    background_pixel: XWhitePixel(c.display, c.screen_id),
                    override_redirect: True,
                    colormap: XCreateColormap(c.display, root_window, visual.to_ref().visual.cast(), AllocNone),
                    event_mask: KeyPressMask
                        | KeyReleaseMask
                        | FocusChangeMask
                        | PointerMotionMask
                        | ButtonMotionMask
                        | ButtonPressMask
                        | ButtonReleaseMask
                        | EnterWindowMask
                        | LeaveWindowMask
                        | ExposureMask
                        | StructureNotifyMask,
                    ..std::mem::zeroed()
                }
            };

            let window = unsafe {
                XCreateWindow(
                    c.display,
                    root_window,
                    window_builder.x,
                    window_builder.y,
                    window_builder.width as _,
                    window_builder.height as _,
                    0,
                    visual.to_ref().depth,
                    InputOutput as u32,
                    visual.to_ref().visual.cast(),
                    CWColormap | CWBorderPixel | CWEventMask,
                    &mut window_attrib,
                )
            };

            let ic = unsafe {
                XCreateIC(
                    c.im,
                    XNInputStyle_0.as_ptr(),
                    XIMPreeditNothing | XIMStatusNothing,
                    XNClientWindow_0.as_ptr(),
                    window,
                    0usize,
                )
            };

            c.window_map.insert(window, ic);

            unsafe { XSetICFocus(ic) };

            Ok((GlSurface { display, window, config: config as _ }, WindowHandle(window)))
        } else {
            todo!()
        }
    }

    pub fn build<E: Event, P: GlConfigPicker>(
        _window: WindowHandle, _config: &[u32], _picker: &mut P,
    ) -> Result<GlSurface, Error> {
        Err(ErrorKind::Unsupported.into())
    }

    pub fn set_swap_interval(self, interval: i32) -> Result<(), Error> {
        let glx = get_glx()?;

        let drawable = unsafe { glx.GetCurrentDrawable() };
        let mut swap = 0;
        let mut max_swap = 0;

        if drawable != 0 {
            unsafe {
                glx.QueryDrawable(self.display, drawable, SWAP_INTERVAL_EXT as _, &mut swap);
                glx.QueryDrawable(self.display, drawable, MAX_SWAP_INTERVAL_EXT as _, &mut max_swap);

                dbg!(swap);
                dbg!(max_swap);
            }
        }

        Ok(())
    }

    pub fn make_current(self, context: GlContext) -> Result<(), Error> {
        if unsafe { GLX.is_null() } {
            return lib_not_loaded_err();
        }
        let glx = unsafe { &*GLX };

        if unsafe { glx.MakeCurrent(self.display, self.window, context.context as _) } == 0 {
            return Err(Error::new(Other, "Failed to make current."));
        }

        Ok(())
    }

    pub fn create_context(&self, config: &[u32], share_context: GlContext) -> Result<GlContext, Error> {
        let glx = get_glx()?;

        let context = unsafe {
            glx.CreateContextAttribsARB(self.display, self.config, share_context.context as _, 1, config.as_ptr().cast())
        };

        if context.is_null() {
            todo!()
        }

        Ok(GlContext { context: context as _ })
    }

    pub fn swap_buffers(self) -> Result<(), Error> {
        if unsafe { GLX.is_null() } {
            return lib_not_loaded_err();
        }
        let glx = unsafe { &*GLX };

        unsafe { glx.SwapBuffers(self.display, self.window) };

        Ok(())
    }
}

impl Destroy for GlSurface {
    fn destroy(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub static mut GLX: *mut Glx = null_mut();
pub static mut LIB_OPENGL: *mut c_void = null_mut();

const LAYOUT_GLX: Layout = Layout::new::<Glx>();

pub fn load_lib_opengl() -> Result<(), Error> {
    let paths = ["libGL.so.1\0", "libGL.so\0"];

    unsafe {
        if LIB_OPENGL.is_null() {
            for path in paths {
                let path = path.as_ptr() as _;
                LIB_OPENGL = dlopen(path, RTLD_LAZY | RTLD_LOCAL) as _;
                if !LIB_OPENGL.is_null() {
                    // TODO info log
                    break;
                }
            }

            if LIB_OPENGL.is_null() {
                todo!("Error")
            }
        } else {
            // TODO Log warning
        }

        if GLX.is_null() {
            GLX = alloc(LAYOUT_GLX).cast();

            #[allow(non_snake_case)]
            let PFNglXGetProcAddressARB = dlsym(LIB_OPENGL, "glXGetProcAddressARB\0".as_ptr().cast());
            if PFNglXGetProcAddressARB.is_null() {
                todo!()
            }

            GLX.write(Glx::load_with(|symbol| {
                let mut c_symbol = symbol.to_string();
                c_symbol.push('\0');

                let dl_symbol = dlsym(LIB_OPENGL, c_symbol.as_ptr().cast());
                if !dl_symbol.is_null() {
                    return dl_symbol;
                }

                #[allow(non_snake_case)]
                let glXGetProcAddressARB: extern "C" fn(
                    proc_name: *const glutin_glx_sys::glx::types::GLubyte,
                ) -> *const c_void = std::mem::transmute(PFNglXGetProcAddressARB);

                glXGetProcAddressARB(c_symbol.as_ptr().cast())
            }));
        } else {
            // TODO
        }

        Ok(())
    }
}

pub fn free_lib_opengl() -> Result<(), Error> {
    unsafe {
        if GLX.is_null() {
        } else {
            dealloc(GLX.cast(), LAYOUT_GLX);
            GLX = null_mut();
        }

        if LIB_OPENGL.is_null() {
            // TODO
        } else {
            // TODO Errors
            dlclose(LIB_OPENGL);
            LIB_OPENGL = null_mut();
        }
    }
    Ok(())
}

#[test]
fn glx_load_test() {
    load_lib_opengl().unwrap();
    unsafe {
        assert!(!GLX.is_null());
        let glx = &*GLX;
        assert!(glx.CreateContextAttribsARB.is_loaded());
        assert!(glx.ChooseVisual.is_loaded());
        assert!(glx.CreateWindow.is_loaded());
        assert!(glx.GetProcAddress.is_loaded());
    }
    free_lib_opengl().unwrap();
}

pub fn lib_not_loaded_err<T>() -> Result<T, Error> {
    Err(Error::new(std::io::ErrorKind::Other, "glX is not loaded. Load glX with `exposed_gl::load_lib_opengl`."))
}

pub unsafe fn get_proc_addr(symbol: *const c_char) -> *const c_void {
    if GLX.is_null() {
        null()
    } else {
        let glx = &*GLX;
        glx.GetProcAddress(symbol.cast()).cast()
    }
}

pub struct GlPixelFormat {
    pub display: *mut Display,
    pub format: usize,
}

impl GlPixelFormat {
    pub fn get<const C: usize>(&self, attributes: &[u32; C], values: &mut [i32; C]) -> Result<(), Error> {
        let glx = get_glx()?;

        for (i, a) in attributes.iter().enumerate() {
            unsafe { glx.GetFBConfigAttrib(self.display, self.format as _, *a as i32, values.get_unchecked_mut(i)) };
        }

        Ok(())
    }
}

pub fn get_glx() -> Result<&'static mut Glx, Error> {
    unsafe {
        if !GLX.is_null() {
            Ok(&mut *GLX)
        } else {
            Err(Error::new(ErrorKind::NotFound, "First initialize GLX with load_lib_opengl."))
        }
    }
}
