use std::{
    alloc::{alloc, dealloc, Layout},
    ffi::c_void,
    io::{Error, ErrorKind},
    mem::zeroed,
    ptr::{null, null_mut},
    sync::atomic::{AtomicPtr, Ordering},
};

use exposed::{
    destroy::{Destroy, Destroyable},
    log::{log_verbose, log_warn},
    window::{
        android::ndk::ANativeWindow_setBuffersGeometry,
        platform::{WindowBuilder, WindowHandle},
        Context, Event,
    },
};
use glutin_egl_sys::{
    egl::{
        self,
        types::{EGLConfig, EGLSurface},
        Egl,
    },
    EGLContext, EGLDisplay,
};
use libc::{c_char, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};

pub use glutin_egl_sys;

use crate::GlConfigPicker;

pub static mut EGL: *mut Egl = null_mut();
pub static mut LIB_OPENGL: *mut c_void = null_mut();

const LAYOUT_GLX: Layout = Layout::new::<Egl>();

pub static mut EGL_DISPLAY: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

fn egl_error(egl: &mut Egl) -> i32 {
    unsafe { egl.GetError() }
}

macro_rules! egl_error {
    ($egl:expr, $m:tt) => {
        Err(Error::new(ErrorKind::Other, format!($m, egl_error($egl))))
    };
}

pub fn load_lib_opengl() -> Result<(), Error> {
    let paths = ["libEGL.so.1\0", "libEGL.so\0"];

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
            todo!()
        }

        if EGL.is_null() {
            EGL = alloc(LAYOUT_GLX).cast();

            #[allow(non_snake_case)]
            let PFN_eglGetProcAddress = dlsym(LIB_OPENGL, "eglGetProcAddress\0".as_ptr().cast());
            if PFN_eglGetProcAddress.is_null() {
                todo!()
            }

            EGL.write(Egl::load_with(|symbol| {
                let mut c_symbol = symbol.to_string();
                c_symbol.push('\0');

                let dl_symbol = dlsym(LIB_OPENGL, c_symbol.as_ptr().cast());
                if !dl_symbol.is_null() {
                    return dl_symbol;
                }

                #[allow(non_snake_case)]
                let eglGetProcAddress: extern "C" fn(proc_name: *const c_char) -> *const c_void =
                    std::mem::transmute(PFN_eglGetProcAddress);

                eglGetProcAddress(c_symbol.as_ptr().cast())
            }));
        } else {
            // TODO
            todo!()
        }

        let egl = get_egl()?;

        let display = egl.GetDisplay(egl::DEFAULT_DISPLAY);
        if display.is_null() {
            return egl_error!(egl, "{}");
        }

        let mut major = 0;
        let mut minor = 0;

        if egl.Initialize(display, &mut major, &mut minor) == 0 {
            return egl_error!(egl, "Failed to initialize EGLDisplay. (EGL error: {})");
        }

        EGL_DISPLAY.store(display.cast_mut(), Ordering::Release);

        log_verbose!("Exposed", "EGL version: {major}.{minor}");

        Ok(())
    }
}

pub fn free_lib_opengl() -> Result<(), Error> {
    unsafe {
        let display = EGL_DISPLAY.load(Ordering::Acquire);

        if !display.is_null() {
            if let Ok(egl) = get_egl() {
                egl.Terminate(display);
            } else {
                log_warn!("Exposed", "Failed to terminate display. EGL was not present.");
            }
        }

        if !EGL.is_null() {
            dealloc(EGL.cast(), LAYOUT_GLX);
            EGL = null_mut();
        }

        if !LIB_OPENGL.is_null() {
            dlclose(LIB_OPENGL);
            LIB_OPENGL = null_mut();
        }
    }
    Ok(())
}

pub fn get_egl() -> Result<&'static mut Egl, Error> {
    unsafe {
        if !EGL.is_null() {
            Ok(&mut *EGL)
        } else {
            Err(Error::new(ErrorKind::NotFound, "First initialize EGL with load_lib_opengl."))
        }
    }
}

pub fn get_display() -> Result<EGLDisplay, Error> {
    let display = unsafe { EGL_DISPLAY.load(Ordering::Acquire) };

    if display.is_null() {
        return Err(Error::new(ErrorKind::NotFound, "EGL_DISPLAY is null. Initialize with load_lib_opengl."));
    }

    Ok(display)
}

pub unsafe fn get_proc_addr(symbol: *const c_char) -> *const c_void {
    debug_assert!(!EGL.is_null());
    debug_assert!(LIB_OPENGL != 0 as _);

    let egl = &mut *EGL;

    egl.GetProcAddress(symbol as *const c_char) as _
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct GlSurfaceBuilder {
    alpha_size: u32,
    blue_size: u32,
    red_size: u32,
    green_size: u32,
    depth_size: u32,
    samples: u32,
    stencil_size: u32,
}

impl GlSurfaceBuilder {
    pub fn new() -> Self {
        Self { red_size: 8, blue_size: 8, green_size: 8, alpha_size: 8, ..Default::default() }
    }
    pub fn build<E: Event>(&self, window: WindowHandle) -> Result<GlSurface, Error> {
        unsafe {
            let egl = get_egl()?;
            let display = get_display()?;

            #[rustfmt::skip]
            let attrib = [
                egl::SURFACE_TYPE, egl::WINDOW_BIT,
                egl::BLUE_SIZE, self.blue_size,
                egl::GREEN_SIZE, self.green_size,
                egl::RED_SIZE, self.red_size,
                egl::ALPHA_SIZE, self.alpha_size,
                // egl::SAMPLES, self.samples,
                // egl::DEPTH_SIZE, self.depth_size,
                // egl::STENCIL_SIZE, self.stencil_size,
                egl::NONE
            ];

            let mut config = zeroed();
            let mut num_configs = 0;

            if egl.ChooseConfig(display, attrib.as_ptr().cast(), &mut config, 1, &mut num_configs) == 0 {
                return egl_error!(egl, "Failed to choose a suitable config. (EGL error: {})");
            }

            let mut format = 0;
            if egl.GetConfigAttrib(display, config, egl::NATIVE_VISUAL_ID as _, &mut format) == 0 {
                return egl_error!(egl, "Failed to get config attrib. (EGL error: {})");
            }

            ANativeWindow_setBuffersGeometry(window.native_handle(), 0, 0, format);

            let surface = egl.CreateWindowSurface(display, config, window.native_handle() as _, 0 as _);

            if surface.is_null() {
                return egl_error!(egl, "Failed to create surface. (EGL error: {})");
            }

            Ok(GlSurface { surface, config })
        }
    }

    pub fn build_with<E: Event>(
        &self, context: Context, window_builder: &WindowBuilder,
    ) -> Result<(GlSurface, WindowHandle), Error> {
        unsafe {
            let window = Destroyable(window_builder.build::<E>(context)?);

            let surface = self.build::<E>(window.0)?;

            Ok((surface, window.into_inner()))
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlSurface {
    pub surface: EGLSurface,
    pub config: EGLConfig,
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
        let egl = get_egl()?;
        let display = get_display()?;

        let mut configs_capacity = 0;
        if unsafe { egl.GetConfigs(display, null_mut(), 0, &mut configs_capacity) } == 0 {
            return egl_error!(egl, "Failed at eglGetConfigs. (EGL error: {})");
        }

        let mut configs = Vec::with_capacity(configs_capacity as usize);

        let mut configs_len = 0;
        if unsafe { egl.ChooseConfig(display, config.as_ptr().cast(), configs.as_mut_ptr(), configs_capacity, &mut configs_len) }
            == 0
        {
            return egl_error!(egl, "Failed at eglGetConfigs. (EGL error: {})");
        }
        unsafe { configs.set_len(configs_len as usize) };

        let mut picked_config = None;
        for config in configs {
            if let Some(c) = picker.pick(GlPixelFormat { format: config as _ }) {
                picked_config = Some(c);
            }
        }

        if let Some(picked_config) = picked_config {
            let mut format = 0;
            if unsafe { egl.GetConfigAttrib(display, picked_config as _, egl::NATIVE_VISUAL_ID as _, &mut format) } == 0 {
                return egl_error!(egl, "Failed to get config attrib. (EGL error: {})");
            }

            unsafe { ANativeWindow_setBuffersGeometry(window.native_handle(), 0, 0, format) };

            let surface = unsafe { egl.CreateWindowSurface(display, picked_config as _, window.native_handle() as _, 0 as _) };

            if surface.is_null() {
                return egl_error!(egl, "Failed to create surface. (EGL error: {})");
            }

            Ok(GlSurface { surface, config: picked_config as _ })
        } else {
            todo!()
        }
    }

    pub fn create_context(&self, config: &[u32], share_context: GlContext) -> Result<GlContext, Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        let context: EGLContext = unsafe { egl.CreateContext(display, self.config, share_context.0, config.as_ptr().cast()) };

        if context.is_null() {
            return egl_error!(egl, "Failed at eglCreateContext. (EGL error: {})");
        }

        Ok(GlContext(context))
    }

    pub fn swap_buffers(self) -> Result<(), Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        if unsafe { egl.SwapBuffers(display, self.surface) } == 0 {
            return egl_error!(egl, "Failed to swap buffers. (EGL error: {}).");
        }

        Ok(())
    }

    pub fn set_swap_interval(self, interval: i32) -> Result<(), Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        if unsafe { egl.SwapInterval(display, interval) } == 0 {
            return egl_error!(egl, "Failed to set swap interval to {interval}. (EGL error: {}).");
        }

        Ok(())
    }

    pub fn make_current(self, context: GlContext) -> Result<(), Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        let surface;
        if context.0.is_null() {
            surface = null();
        } else {
            surface = self.surface;
        }

        if unsafe { egl.MakeCurrent(display, surface, surface, context.0) } == 0 {
            return egl_error!(egl, "Failed to make context current. (EGL error: {})");
        }

        Ok(())
    }

    pub fn make_no_context(self) -> Result<(), Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        if unsafe { egl.MakeCurrent(display, null(), null(), null()) } == 0 {
            return egl_error!(egl, "Failed to make context current. (EGL error: {})");
        }

        Ok(())
    }
}

impl Destroy for GlSurface {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        if unsafe { egl.DestroySurface(display, self.surface) } == 0 {
            return egl_error!(egl, "Failed to destroy EGLsurface. (EGL error: {})");
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlContext(pub EGLContext);

impl Destroy for GlContext {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        if unsafe { egl.DestroyContext(display, self.0) } == 0 {
            return egl_error!(egl, "Failed at eglDestroyContext. (EGL error: {})");
        }

        Ok(())
    }
}

impl GlContext {
    pub const NO_CONTEXT: Self = Self(null());
}

#[repr(C)]
pub struct GlPixelFormat {
    pub format: usize,
}

impl GlPixelFormat {
    pub fn get<const C: usize>(&self, attributes: &[u32; C], values: &mut [i32; C]) -> Result<(), Error> {
        let egl = get_egl()?;
        let display = get_display()?;

        for (i, a) in attributes.iter().enumerate() {
            if unsafe { egl.GetConfigAttrib(display, self.format as _, *a as i32, values.get_unchecked_mut(i)) } == 0 {
                return egl_error!(egl, "Failed at eglGetConfigAttrib. (EGL error: {})");
            }
        }

        Ok(())
    }
}
