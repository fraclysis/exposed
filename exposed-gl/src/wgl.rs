use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    io::Error,
    mem::zeroed,
    ptr::{null, null_mut}, ffi::{c_char, c_void},
};

use glutin_wgl_sys::wgl_extra::Wgl;
use windows_sys::{
    w,
    Win32::{
        Foundation::HMODULE,
        Graphics::{
            Gdi::{GetDC, ReleaseDC},
            OpenGL::{
                wglCreateContext, wglDeleteContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat,
                PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
            },
        },
        System::LibraryLoader::{FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryW},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, UnregisterClassW, CS_OWNDC, CW_USEDEFAULT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW,
        },
    },
};
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

