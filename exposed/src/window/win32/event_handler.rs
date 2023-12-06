use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
    mem::size_of,
    ptr::{null, null_mut},
};

use windows_sys::Win32::{
    Foundation::HMODULE,
    System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentThreadId},
    UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, LoadCursorW, PeekMessageW, RegisterClassExW, TranslateMessage, UnregisterClassW,
        CS_HREDRAW, CS_OWNDC, CS_VREDRAW, HCURSOR, HICON, IDC_ARROW, MSG, PM_REMOVE, WNDCLASSEXW,
    },
};

use crate::{
    destroy::{Destroy, Destroyable},
    window::{platform, win32::win_proc, Context, Event},
};

use super::ThreadContext;

#[repr(C)]
pub struct EventHandler<E: Event> {
    pub window_class: Vec<u16>,
    pub msg: MSG,
    pub _mark: std::marker::PhantomData<E>,
}

impl<E: Event> Debug for EventHandler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler").finish()
    }
}

impl<E: Event> Into<crate::window::EventHandler<E>> for EventHandler<E> {
    fn into(self) -> crate::window::EventHandler<E> {
        crate::window::EventHandler(self)
    }
}

impl<E: Event> Destroy for EventHandler<E> {
    fn destroy(&mut self) -> Result<(), Error> {
        let context = unsafe { ThreadContext::get_ref() };

        context.window_class = null();
        let result = unsafe { UnregisterClassW(self.window_class.as_ptr(), 0) };
        context.user_data = null_mut();

        if result == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}

impl<E: Event> EventHandler<E> {
    pub fn poll(&mut self) -> i32 {
        unsafe { PeekMessageW(&mut self.msg, 0, 0, 0, PM_REMOVE) }
    }

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

pub static mut HINSTANCE: HMODULE = 0;

#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerBuilder {
    pub class_style: u32,
    pub icon: HICON,
    pub cursor: HCURSOR,
}

impl Default for EventHandlerBuilder {
    fn default() -> Self {
        Self { class_style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW, icon: 0, cursor: unsafe { LoadCursorW(0, IDC_ARROW) } }
    }
}

impl EventHandlerBuilder {
    pub unsafe fn build<E: Event>(&mut self, user_data: *mut E) -> Result<platform::EventHandler<E>, Error> {
        if unsafe { HINSTANCE } == 0 {
            unsafe { HINSTANCE = GetModuleHandleW(null()) };
        }

        if !ThreadContext::get().window_class.is_null() {
            return Err(Error::new(ErrorKind::Other, "Single EventHandler is allowed per thread."));
        }

        let thread_id = GetCurrentThreadId();

        let window_class: Vec<u16> = format!("ExposedClass{thread_id}\0").encode_utf16().collect();

        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: self.class_style,
            lpfnWndProc: Some(win_proc::<E>),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: unsafe { HINSTANCE },
            hIcon: self.icon,
            hCursor: self.cursor,
            hbrBackground: 0,
            lpszMenuName: null(),
            lpszClassName: window_class.as_ptr(),
            hIconSm: 0,
        };

        let class_atom = unsafe { RegisterClassExW(&wc) };
        if class_atom == 0 {
            return Err(Error::last_os_error());
        }
        let event_handler = Destroyable(EventHandler { window_class, msg: std::mem::zeroed(), _mark: std::marker::PhantomData });

        let context = ThreadContext::get_ref();
        context.window_class = event_handler.window_class.as_ptr();

        if let Some(event) = E::create(Context(ThreadContext::get_ref())) {
            std::ptr::write(user_data, event);
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to create event."));
        }

        context.user_data = user_data as _;

        Ok(event_handler.into_inner())
    }
}
