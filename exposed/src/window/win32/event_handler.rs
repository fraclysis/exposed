use std::io::Error;

use windows_sys::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, PeekMessageW, TranslateMessage, UnregisterClassW, MSG,
            PM_REMOVE,
        },
    },
};

use crate::window::Event;

#[repr(C)]
#[allow(non_snake_case)]
pub struct EventHandler<E: Event> {
    pub userData: *mut E,
    pub wndClassName: PCWSTR,
    pub msg: MSG,
    pub running: bool,
    pub isUserDataValid: bool,
    pub last_hwnd: HWND,
    pub last_msg: u32,
    pub last_wparam: WPARAM,
    pub last_lparam: LPARAM,
}

impl<E: Event> EventHandler<E> {
    pub fn destroy(&self) -> Result<(), Error> {
        unsafe { &mut *self.userData }.destroy();

        if unsafe { UnregisterClassW(self.wndClassName, 0) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    /// If no messages are available, the return value false.
    #[inline]
    pub fn poll(&mut self) -> i32 {
        unsafe { PeekMessageW(&mut self.msg, 0, 0, 0, PM_REMOVE) }
    }

    /// # Retuns
    /// If it recives WM_QUIT or an error occurs returns false otherwise retuns true
    #[inline]
    pub fn wait(&mut self) -> i32 {
        unsafe { GetMessageW(&mut self.msg, 0, 0, 0) }
    }

    #[inline]
    pub fn dispatch(&mut self) {
        unsafe {
            TranslateMessage(&self.msg);
            DispatchMessageW(&self.msg);
        }
    }
}
