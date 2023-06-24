use std::{
    io::Error,
    mem::{transmute, zeroed},
    ptr::null,
};

use windows_sys::Win32::{
    Foundation::{ERROR_INVALID_HANDLE, HWND, RECT},
    Graphics::Gdi::{InvalidateRect, UpdateWindow},
    UI::{
        HiDpi::GetDpiForWindow,
        WindowsAndMessaging::{
            DestroyWindow, GetClientRect, GetWindowRect, GetWindowTextW, SetWindowTextW,
            ShowWindowAsync, SW_SHOWDEFAULT,
        },
    },
};

use crate::window::Size;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle {
    pub windowHandle: HWND,
}

impl WindowHandle {
    #[inline]
    pub fn destroy(self) -> Result<(), Error> {
        if unsafe { DestroyWindow(self.windowHandle) } == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn show(self) -> Result<(), Error> {
        if unsafe { ShowWindowAsync(self.windowHandle, SW_SHOWDEFAULT) } == 0 {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to show window. Window might be not valid.",
            ))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn update(self) {
        unsafe { UpdateWindow(self.windowHandle) };
    }

    #[inline]
    pub fn redraw(self) -> Result<(), Error> {
        if unsafe { InvalidateRect(self.windowHandle, null(), 0) } == 0 {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to invalidate the window.",
            ))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn get_window_title(self) -> Result<String, Error> {
        let mut buffer = [0u16; 255];

        let len = unsafe { GetWindowTextW(self.windowHandle, buffer.as_mut_ptr() as _, 255) };
        if len == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(String::from_utf16_lossy(&buffer[0..len as usize]))
        }
    }

    #[inline]
    pub fn set_window_title(hwnd: HWND, title: String) -> Result<(), Error> {
        let title_u16 = utf8_to_utf16_null(&title);

        if unsafe { SetWindowTextW(hwnd, title_u16.as_ptr()) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn get_window_rect(self) -> Result<RECT, Error> {
        unsafe {
            let mut rect = zeroed();
            if GetWindowRect(self.windowHandle, &mut rect) == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(rect)
            }
        }
    }

    pub fn get_client_rect(self) -> Result<RECT, Error> {
        unsafe {
            let mut rect = zeroed();
            if GetClientRect(self.windowHandle, &mut rect) == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(rect)
            }
        }
    }

    pub fn get_dpi(self) -> Result<u32, Error> {
        let dpi = unsafe { GetDpiForWindow(self.windowHandle) };
        if dpi == 0 {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow
            // "An invalid hwnd value will result in a return value of 0."
            Err(Error::from_raw_os_error(unsafe {
                transmute(ERROR_INVALID_HANDLE)
            }))
        } else {
            Ok(dpi)
        }
    }

    pub fn get_client_size(&self) -> Result<Size, Error> {
        let rect = self.get_client_rect()?;

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok(Size { width, height })
    }

    pub fn get_window_size(&self) -> Result<Size, Error> {
        let rect = self.get_window_rect()?;

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok(Size { width, height })
    }
}

pub fn utf8_to_utf16_null(text: &str) -> Vec<u16> {
    let mut utf16: Vec<u16> = text.encode_utf16().collect();
    utf16.push(0);
    utf16
}
