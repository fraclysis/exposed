use std::{
    io::{Error, ErrorKind::Other},
    os::raw::c_ulong,
    ptr::null,
};

use exposed::window::{Event, EventHandler, WindowBuilder, WindowHandle};
use glutin_glx_sys as glx;
use glx::{
    glx_extra::{
        self,
        types::{Display, GLXFBConfig},
    },
    AllocNone, InputOutput, StructureNotifyMask, XIMStatusNothing, XNClientWindow_0, _XDisplay,
};
use libc::{c_char, c_void};
use x11::xlib::{
    ButtonMotionMask, ButtonPressMask, ButtonReleaseMask, CWBackPixel, CWBorderPixel, CWColormap,
    CWEventMask, EnterWindowMask, ExposureMask, False, FocusChangeMask, KeyPressMask,
    KeyReleaseMask, LeaveWindowMask, PointerMotionMask, ResizeRedirectMask, True, XBlackPixel,
    XCreateColormap, XCreateIC, XCreateWindow, XFree, XIMPreeditNothing, XNInputStyle_0,
    XRootWindow, XSetICFocus, XSetWindowAttributes, XSync, XWhitePixel, _XIC,
};

use crate::{lib_not_loaded_err, GLX};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlContext {
    pub context: usize,
}

impl GlContext {
    pub const NO_CONTEXT: Self = Self { context: 0 };

    pub fn destroy(self) -> Result<(), Error> {
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlDisplay {
    pub display: *mut Display,
    pub window: c_ulong,
    pub config: GLXFBConfig,
}

impl GlDisplay {
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

    pub fn swap_buffers(self) -> Result<(), Error> {
        if unsafe { GLX.is_null() } {
            return lib_not_loaded_err();
        }
        let glx = unsafe { &*GLX };

        unsafe { glx.SwapBuffers(self.display, self.window) };

        Ok(())
    }

    pub fn destroy(self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct GlContextBuilder {}

impl Default for GlContextBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl GlContextBuilder {
    pub fn build(&self, display: GlDisplay, share_context: GlContext) -> Result<GlContext, Error> {
        if unsafe { GLX.is_null() } {
            return lib_not_loaded_err();
        }
        let glx = unsafe { &*GLX };

        #[rustfmt::skip]
        let context_attribs = [
            glx_extra::CONTEXT_MAJOR_VERSION_ARB, 3,
            glx_extra::CONTEXT_MINOR_VERSION_ARB, 0,
            glx_extra::NONE,
        ];

        println!("DD");
        let context = unsafe {
            glx.CreateContextAttribsARB(
                display.display,
                display.config,
                share_context.context as _,
                True,
                null(),
            )
        };
        unsafe { XSync(display.display.cast(), False) };

        println!("FF");

        Ok(GlContext {
            context: context as _,
        })
    }
}

pub struct GlDisplayBuilder {}

impl Default for GlDisplayBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl GlDisplayBuilder {
    pub fn build_with<E: Event>(
        &self,
        window_builder: WindowBuilder,
        event_handler: &mut EventHandler<E>,
    ) -> Result<(GlDisplay, WindowHandle), Error> {
        let display = event_handler.display.cast();
        let screen_id = event_handler.screen_id;

        let mut glx_major = 0;
        let mut glx_minor = 0;

        unsafe {
            if GLX.is_null() {
                return lib_not_loaded_err();
            }
        }

        let glx = unsafe { &*GLX };

        if unsafe { glx.QueryVersion(display, &mut glx_major, &mut glx_minor) } == 0 {
            return Err(Error::new(Other, "glXQueryVersion returned an error."));
        }

        #[rustfmt::skip]
        let glx_attribs = [
            glx_extra::X_RENDERABLE    , glx::True as u32,
		    glx_extra::DRAWABLE_TYPE   , glx_extra::WINDOW_BIT,
		    glx_extra::RENDER_TYPE     , glx_extra::RGBA_BIT,
		    glx_extra::X_VISUAL_TYPE   , glx_extra::TRUE_COLOR,
		    glx_extra::RED_SIZE        , 8,
		    glx_extra::GREEN_SIZE      , 8,
		    glx_extra::BLUE_SIZE       , 8,
		    glx_extra::ALPHA_SIZE      , 8,
		    glx_extra::DEPTH_SIZE      , 24,
		    glx_extra::STENCIL_SIZE    , 8,
		    glx_extra::DOUBLEBUFFER    , glx::True as u32,
		    glx_extra::NONE
        ];

        let mut fbcount = 0;
        let fbc: *mut GLXFBConfig = unsafe {
            glx.ChooseFBConfig(
                display.cast(),
                screen_id,
                glx_attribs.as_ptr().cast(),
                &mut fbcount,
            )
        };

        if fbc.is_null() {
            return Err(Error::new(Other, "Failed to retrieve framebuffer."));
        }

        let fbc_slice: &[GLXFBConfig] =
            unsafe { std::slice::from_raw_parts(fbc, fbcount as usize) };

        // Pick the FB config/visual with the most samples per pixel
        let mut best_fbc = -1;
        let mut worst_fbc = -1;
        let mut best_num_samp = -1;
        let mut worst_num_samp = 999;

        for (i, f) in fbc_slice.iter().enumerate() {
            let f: GLXFBConfig = *f;
            let vi = unsafe { glx.GetVisualFromFBConfig(display, f) };

            if !vi.is_null() {
                let mut samp_buf = 0;
                let mut samples = 0;

                unsafe {
                    glx.GetFBConfigAttrib(
                        display,
                        f,
                        glx::glx_extra::SAMPLE_BUFFERS as i32,
                        &mut samp_buf,
                    );
                    glx.GetFBConfigAttrib(display, f, glx::glx_extra::SAMPLES as i32, &mut samples)
                };

                if (best_fbc < 0) || (samp_buf != 0 && samples > best_num_samp) {
                    best_fbc = i as i32;
                    best_num_samp = samples;
                }

                if (worst_fbc < 0) || samp_buf == 0 || samples < worst_num_samp {
                    worst_fbc = i as i32;
                }
                println!("S:{samples}, B:{samp_buf}");
                worst_num_samp = samples;
            }

            unsafe { XFree(vi.cast()) };
        }

        let best_fbcc: GLXFBConfig = fbc_slice[best_fbc as usize];

        {
            let mut samp_buf = 0;
            let mut samples = 0;

            unsafe {
                glx.GetFBConfigAttrib(
                    display,
                    best_fbcc,
                    glx::glx_extra::SAMPLE_BUFFERS as i32,
                    &mut samp_buf,
                );
                glx.GetFBConfigAttrib(
                    display,
                    best_fbcc,
                    glx::glx_extra::SAMPLES as i32,
                    &mut samples,
                )
            };

            println!("Picked Config with {samples} samples and sample buffers {samp_buf}.");
        }

        drop(fbc_slice);
        unsafe { XFree(fbc.cast()) };

        let visual = unsafe { glx.GetVisualFromFBConfig(display, best_fbcc) };
        if visual.is_null() {
            return Err(Error::new(Other, "Could not create correct visual info."));
        }

        if screen_id != unsafe { &*visual }.screen {
            return Err(Error::new(
                Other,
                "Visual.screen and screen_id do not match.",
            ));
        }

        let mut window_attribs: XSetWindowAttributes = unsafe { std::mem::zeroed() };
        window_attribs.border_pixel = unsafe { XBlackPixel(display.cast(), screen_id) };
        window_attribs.background_pixel = unsafe { XWhitePixel(display.cast(), screen_id) };
        window_attribs.override_redirect = True;
        window_attribs.colormap = unsafe {
            XCreateColormap(
                display.cast(),
                XRootWindow(display.cast(), screen_id),
                (&*visual).visual.cast(),
                AllocNone,
            )
        };

        window_attribs.event_mask = KeyPressMask
            | KeyReleaseMask
            | FocusChangeMask
            // | ResizeRedirectMask
            | PointerMotionMask
            | ButtonMotionMask
            | ButtonPressMask
            | ButtonReleaseMask
            | EnterWindowMask
            | LeaveWindowMask
            | ExposureMask
            | StructureNotifyMask;

        let window = unsafe {
            XCreateWindow(
                display.cast(),
                XRootWindow(display.cast(), screen_id),
                window_builder.x,
                window_builder.y,
                window_builder.width as u32,
                window_builder.height as u32,
                0,
                (&*visual).depth,
                InputOutput as u32,
                (&*visual).visual.cast(),
                /* CWBackPixel | */ CWColormap | CWBorderPixel | CWEventMask,
                &mut window_attribs,
            )
        };

        println!("WW");

        unsafe {
            let ic: *mut _XIC = XCreateIC(
                event_handler.im,
                XNInputStyle_0.as_ptr(),
                XIMPreeditNothing | XIMStatusNothing,
                XNClientWindow_0.as_ptr(),
                window,
                0usize,
            );
            XSetICFocus(ic);

            event_handler.window_data.insert(window, ic);
        }

        Ok((
            GlDisplay {
                display,
                window,
                config: best_fbcc,
            },
            WindowHandle {
                windowHandle: window,
                display: event_handler.display,
            },
        ))
    }

    #[deprecated]
    pub fn build<E: Event>(
        &self,
        window: WindowHandle,
        event_handler: &mut EventHandler<E>,
    ) -> Result<GlDisplay, Error> {
        if unsafe { GLX }.is_null() {
            return Err(Error::new(Other, "Load glx with load_lib_opengl."));
        }

        // let display = unsafe { x11::xlib::XOpenDisplay(null()) };
        // let screen_id = unsafe { x11::xlib::XDefaultScreen(display) };

        let display = event_handler.display;
        let screen_id = event_handler.screen_id;

        dbg!(display);
        dbg!(screen_id);

        let glx = unsafe { &*GLX };

        let mut major = 0;
        let mut minor = 0;

        if unsafe { glx.QueryVersion(display.cast(), &mut major, &mut minor) } == glx::False {
            return Err(Error::new(Other, "Failed to query GLX version."));
        }

        if (major <= 1) && (minor <= 2) {
            return Err(Error::new(Other, "GLX 1.2 or greater is required."));
        }

        println!("Glx: {major}.{minor}");

        #[rustfmt::skip]
        let glx_attribs = [
            glx_extra::X_RENDERABLE    , glx::True as u32,
		    glx_extra::DRAWABLE_TYPE   , glx_extra::WINDOW_BIT,
		    glx_extra::RENDER_TYPE     , glx_extra::RGBA_BIT,
		    glx_extra::X_VISUAL_TYPE   , glx_extra::TRUE_COLOR,
		    glx_extra::RED_SIZE        , 8,
		    glx_extra::GREEN_SIZE      , 8,
		    glx_extra::BLUE_SIZE       , 8,
		    glx_extra::ALPHA_SIZE      , 8,
		    glx_extra::DEPTH_SIZE      , 24,
		    glx_extra::STENCIL_SIZE    , 8,
		    glx_extra::DOUBLEBUFFER    , glx::True as u32,
		    glx_extra::NONE
        ];

        let mut fbcount = 0;
        let fbc: *mut GLXFBConfig = unsafe {
            glx.ChooseFBConfig(
                display.cast(),
                screen_id,
                glx_attribs.as_ptr().cast(),
                &mut fbcount,
            )
        };

        if fbc.is_null() {
            return Err(Error::new(Other, "Failed to retrieve framebuffer."));
        }

        println!("Found {fbcount} matching framebuffers.");

        // TODO XFree( fbc ); // Make sure to free this!

        // !! This fails because of fbccast
        let b_fbc = unsafe { *(fbc.add(0)) };
        let visual = unsafe { glx.GetVisualFromFBConfig(display.cast(), b_fbc) }; // fbc is a array so for now we are using it as fbc[0] by casting

        println!("VVSEEE");

        if visual.is_null() {
            return Err(Error::new(Other, "Could not create visual window."));
        }

        println!("CCVVSEEE");

        if event_handler.screen_id != unsafe { &*visual }.screen {
            // TODO
            return Err(Error::new(Other, "No match."));
        }

        println!("EEE");
        todo!("Retval--------------")
    }
}

pub unsafe fn get_proc_addr(symbol: *const c_char) -> *const c_void {
    if GLX.is_null() {
        // TODO warn and retrun null
        null()
    } else {
        let glx = &*GLX;
        glx.GetProcAddress(symbol.cast()).cast()
    }
}
