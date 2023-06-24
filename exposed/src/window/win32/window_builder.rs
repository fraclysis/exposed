use std::{ffi::c_int, io::Error, ptr::null};

use windows_sys::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{GetLastError, SetLastError, HWND},
        UI::WindowsAndMessaging::{
            CreateWindowExW, SetWindowLongPtrW, CW_USEDEFAULT, GWLP_USERDATA, HMENU,
            WINDOW_EX_STYLE, WINDOW_STYLE, WS_EX_ACCEPTFILES, WS_EX_OVERLAPPEDWINDOW,
            WS_OVERLAPPEDWINDOW,
        },
    },
};

use crate::window::{Event, EventHandler, WindowHandle};

use super::HINSTANCE;

const TITLE_BUFFER_LEN: usize = 256;

#[repr(C)]
#[allow(non_snake_case)]
pub struct WindowBuilder {
    pub defWindowName: PCWSTR,
    pub parent: HWND,
    pub menu: HMENU,
    pub exStyle: WINDOW_EX_STYLE,
    pub style: WINDOW_STYLE,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,

    pub utf8NameBuffer: [u16; 256],
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            exStyle: WS_EX_ACCEPTFILES | WS_EX_OVERLAPPEDWINDOW,
            defWindowName: w!("Fract"),
            style: WS_OVERLAPPEDWINDOW,
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            parent: 0,
            menu: 0,
            utf8NameBuffer: [0u16; TITLE_BUFFER_LEN],
        }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(&mut self, title: &str) -> &mut Self {
        for (i, c) in title.encode_utf16().enumerate() {
            if i >= TITLE_BUFFER_LEN {
                break;
            }
            self.utf8NameBuffer[i] = c;
        }

        self
    }

    pub fn with_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build<E: Event>(
        &self,
        event_handler: &mut EventHandler<E>,
    ) -> Result<WindowHandle, Error> {
        unsafe {
            let window_name = if self.utf8NameBuffer[0] == 0 {
                self.defWindowName
            } else {
                self.utf8NameBuffer.as_ptr()
            };

            let hwnd = CreateWindowExW(
                self.exStyle,
                event_handler.wndClassName,
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

            SetLastError(0);
            if SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                event_handler as *mut EventHandler<E> as isize as _,
            ) == 0
            {
                if GetLastError() != 0 {
                    return Err(Error::last_os_error());
                }
            }

            Ok(WindowHandle { windowHandle: hwnd })
        }
    }
}
