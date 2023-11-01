use std::{
    io::Error,
    mem::{transmute, zeroed},
    ptr::null,
};

use windows_sys::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{ERROR_INVALID_HANDLE, HWND, RECT},
        Graphics::Gdi::InvalidateRect,
        UI::{
            HiDpi::GetDpiForWindow,
            Input::KeyboardAndMouse::{ReleaseCapture, SetCapture},
            WindowsAndMessaging::{
                CreateWindowExW, DestroyWindow, GetClientRect, GetWindowRect, GetWindowTextW, SetWindowTextW, ShowWindowAsync,
                CW_USEDEFAULT, HMENU, SW_SHOWDEFAULT, WS_EX_ACCEPTFILES, WS_EX_OVERLAPPEDWINDOW, WS_OVERLAPPEDWINDOW,
            },
        },
    },
};

use crate::{
    destroy::Destroy,
    window::{Rect, Size},
};

use std::ffi::c_int;

use crate::window::{win32::ThreadContext, Event};

use super::{Context, HINSTANCE};

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(pub HWND);

impl Destroy for WindowHandle {
    fn destroy(&mut self) -> Result<(), Error> {
        if unsafe { DestroyWindow(self.0) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Into<crate::window::WindowHandle> for WindowHandle {
    fn into(self) -> crate::window::WindowHandle {
        crate::window::WindowHandle(self)
    }
}

impl WindowHandle {
    pub fn show(self) -> Result<(), Error> {
        if unsafe { ShowWindowAsync(self.0, SW_SHOWDEFAULT) } == 0 {
            Err(Error::new(std::io::ErrorKind::Other, "Failed to show window. Window might be not valid."))
        } else {
            Ok(())
        }
    }

    pub fn update(self) -> Result<(), Error> {
        todo!()
    }

    pub fn redraw(self) -> Result<(), Error> {
        if unsafe { InvalidateRect(self.0, null(), 0) } == 0 {
            Err(Error::new(std::io::ErrorKind::Other, "Failed to invalidate the window."))
        } else {
            Ok(())
        }
    }

    pub fn window_title(self) -> Result<String, Error> {
        let mut buffer = [0u16; 255];

        let len = unsafe { GetWindowTextW(self.0, buffer.as_mut_ptr() as _, 255) };
        if len == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(String::from_utf16_lossy(&buffer[0..len as usize]))
        }
    }

    pub fn set_window_title(self, title: &str) -> Result<(), Error> {
        let title_u16 = utf8_to_utf16_null(&title);

        if unsafe { SetWindowTextW(self.0, title_u16.as_ptr()) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn dpi(self) -> Result<u32, Error> {
        let dpi = unsafe { GetDpiForWindow(self.0) };
        if dpi == 0 {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow
            // "An invalid hwnd value will result in a return value of 0."
            Err(Error::from_raw_os_error(unsafe { transmute(ERROR_INVALID_HANDLE) }))
        } else {
            Ok(dpi)
        }
    }

    pub fn set_capture(self) {
        unsafe { SetCapture(self.0) };
    }

    pub fn release_capture(self) -> Result<(), Error> {
        unsafe {
            if ReleaseCapture() == 0 {
                return Err(Error::last_os_error());
            }

            Ok(())
        }
    }

    pub fn client_size(self) -> Result<Size, Error> {
        let rect = self.client_rect()?;

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok(Size { width, height })
    }

    pub fn client_rect(self) -> Result<Rect, Error> {
        unsafe {
            let mut rect = zeroed();
            if GetClientRect(self.0, &mut rect) == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(win32_rect_to_rect(rect))
            }
        }
    }

    pub fn window_rect(self) -> Result<Rect, Error> {
        unsafe {
            let mut rect = zeroed();
            if GetWindowRect(self.0, &mut rect) == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(win32_rect_to_rect(rect))
            }
        }
    }
}

pub fn utf8_to_utf16_null(text: &str) -> Vec<u16> {
    let mut utf16: Vec<u16> = text.encode_utf16().collect();
    utf16.push(0);
    utf16
}

fn win32_rect_to_rect(rect: RECT) -> Rect {
    Rect { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom }
}

const TITLE_BUFFER_LEN: usize = 256;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowBuilder {
    pub def_window_name: PCWSTR,
    pub parent: HWND,
    pub menu: HMENU,
    pub ex_style: u32,
    pub style: u32,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,

    pub utf8_name_buffer: [u16; 256],
}

#[cfg(target_os = "windows")]
impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            ex_style: WS_EX_ACCEPTFILES | WS_EX_OVERLAPPEDWINDOW,
            def_window_name: w!("Exposed"),
            style: WS_OVERLAPPEDWINDOW,
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            parent: 0,
            menu: 0,
            utf8_name_buffer: [0u16; TITLE_BUFFER_LEN],
        }
    }
}

#[cfg(target_os = "windows")]
impl WindowBuilder {
    pub fn with_title(&mut self, title: &str) -> &mut Self {
        for (i, c) in title.encode_utf16().enumerate() {
            if i >= TITLE_BUFFER_LEN {
                break;
            }
            self.utf8_name_buffer[i] = c;
        }

        self
    }

    pub fn with_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build<E: Event>(&self, _context: Context) -> Result<WindowHandle, Error> {
        unsafe {
            let window_name = if self.utf8_name_buffer[0] == 0 { self.def_window_name } else { self.utf8_name_buffer.as_ptr() };

            let hwnd = CreateWindowExW(
                self.ex_style,
                ThreadContext::get().window_class,
                window_name,
                self.style,
                self.x,
                self.y,
                self.width,
                self.height,
                self.parent,
                self.menu,
                HINSTANCE,
                null(),
            );

            if hwnd == 0 {
                return Err(Error::last_os_error());
            }

            Ok(WindowHandle(hwnd))
        }
    }
}
