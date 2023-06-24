use std::{
    io::Error,
    mem::{size_of, zeroed},
    ptr::null,
};

use windows_sys::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            LoadCursorW, RegisterClassExW, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, HCURSOR, HICON,
            IDC_ARROW, WNDCLASSEXW, WNDCLASS_STYLES,
        },
    },
};

use crate::window::{win_proc, Event, EventHandler};

pub static mut HINSTANCE: HMODULE = 0;

#[repr(C)]
#[allow(non_snake_case)]
pub struct EventHandlerBuilder {
    pub wndClassName: PCWSTR,
    pub classStyle: WNDCLASS_STYLES,
    pub icon: HICON,
    pub cursor: HCURSOR,
}

impl Default for EventHandlerBuilder {
    fn default() -> Self {
        Self {
            wndClassName: w!("FracDefClassName"),
            classStyle: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            icon: 0,
            cursor: unsafe { LoadCursorW(0, IDC_ARROW) },
        }
    }
}

impl EventHandlerBuilder {
    #[inline]
    pub unsafe fn build<E: Event>(
        &self,
        event_handler: *mut EventHandler<E>,
        user_data: *mut E,
    ) -> Result<(), Error> {
        unsafe { &mut *event_handler }.isUserDataValid = false;

        if unsafe { HINSTANCE } == 0 {
            unsafe { HINSTANCE = GetModuleHandleW(null()) };
        }

        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: self.classStyle,
            lpfnWndProc: Some(win_proc::<E>),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: unsafe { HINSTANCE },
            hIcon: self.icon,
            hCursor: self.cursor,
            hbrBackground: 0,
            lpszMenuName: null(),
            lpszClassName: self.wndClassName,
            hIconSm: 0,
        };

        let window_class = unsafe { RegisterClassExW(&wc) };
        if window_class == 0 {
            return Err(Error::last_os_error());
        }

        unsafe {
            event_handler.write(EventHandler {
                userData: user_data,
                wndClassName: self.wndClassName,
                msg: zeroed(),
                running: true,
                isUserDataValid: false,
                last_hwnd: 0,
                last_msg: 0,
                last_wparam: 0,
                last_lparam: 0,
            })
        };

        // unsafe { user_data.write(E::create(&mut *event_handler)) };
        if unsafe { E::low_create(user_data, &mut *event_handler) } {
            unsafe { &mut *event_handler }.isUserDataValid = true;
        } else {
            unsafe { &mut *event_handler }.running = false;
        }

        Ok(())
    }
}
