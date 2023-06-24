use std::{
    alloc::{alloc, dealloc, Layout},
    io::Error,
    ptr::null_mut,
};

use glutin_glx_sys::glx_extra::Glx;
use libc::{c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};

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
            let PFNglXGetProcAddressARB =
                dlsym(LIB_OPENGL, "glXGetProcAddressARB\0".as_ptr().cast());
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
                )
                    -> *const c_void = std::mem::transmute(PFNglXGetProcAddressARB);

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
    Err(Error::new(
        std::io::ErrorKind::Other,
        "glX is not loaded. Load glX with `exposed_gl::load_lib_opengl`.",
    ))
}
