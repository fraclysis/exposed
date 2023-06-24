#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{
    error::Error,
    ffi::{c_char, c_void},
    fmt::Write,
};

use super::{Event, EventHandlerBuilder, Rect, WindowBuilder, WindowHandle};

static mut ERROR_STRING_WITHOUT_NULL: String = String::new();

pub type EventHandler = crate::window::EventHandler<EventCallback>;

type CreateCallback = extern "C" fn(userData: *mut c_void, eventHandler: *mut EventHandler);

type DestroyCallback = extern "C" fn(userData: *mut c_void);

type NotifyCallback = extern "C" fn(userData: *mut c_void, window: WindowHandle);

type FocusedCallback = extern "C" fn(userData: *mut c_void, window: WindowHandle, focused: bool);

type ResizeCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, width: i32, height: i32);

type CharRecivedCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, character: char);

type KeyCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, key: u32, scancode: u32);

type MovedCallback = extern "C" fn(userData: *mut c_void, window: WindowHandle, x: i32, y: i32);

type MouseWheelCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, delta_x: f32, delta_y: f32);

type MouseButtonCallback = extern "C" fn(userData: *mut c_void, window: WindowHandle, button: i32);

type CursorMovedCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, position_x: i32, position_y: i32);

type FileRecivedCallback =
    extern "C" fn(userData: *mut c_void, window: WindowHandle, path: *const c_char);

#[repr(C)]
pub struct EventCallback {
    userData: *mut c_void,
    create: Option<CreateCallback>,
    destroy: Option<DestroyCallback>,
    render: Option<NotifyCallback>,
    resized: Option<ResizeCallback>,
    focused: Option<FocusedCallback>,
    minimized: Option<NotifyCallback>,
    maximized: Option<NotifyCallback>,
    moved: Option<MovedCallback>,
    closeRequested: Option<NotifyCallback>,
    destroyed: Option<NotifyCallback>,
    keyPressed: Option<KeyCallback>,
    keyReleased: Option<KeyCallback>,
    charRecived: Option<CharRecivedCallback>,
    mouseWheel: Option<MouseWheelCallback>,
    mouseButtonPressed: Option<MouseButtonCallback>,
    mouseButtonReleased: Option<MouseButtonCallback>,
    cursorEntered: Option<NotifyCallback>,
    cursorLeft: Option<NotifyCallback>,
    cursorMoved: Option<CursorMovedCallback>,
    fileRecived: Option<FileRecivedCallback>,
}

pub unsafe fn set_error_string<E: Error>(error: E) {
    ERROR_STRING_WITHOUT_NULL.as_mut_vec().set_len(0);
    let _ = writeln!(&mut ERROR_STRING_WITHOUT_NULL, "{error}");
}

pub unsafe fn set_error_string_display<E: std::fmt::Display>(error: E) {
    ERROR_STRING_WITHOUT_NULL.as_mut_vec().set_len(0);
    let _ = writeln!(&mut ERROR_STRING_WITHOUT_NULL, "{error}");
}

#[repr(C)]
pub enum MessageLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub static mut MESSAGE_PROC: Option<
    extern "C" fn(message: *const c_char, message_len: i32, message_level: MessageLevel),
> = None;

#[no_mangle]
pub unsafe extern "C" fn Fract_set_message_proc(
    proc: Option<
        extern "C" fn(message: *const c_char, message_len: i32, message_level: MessageLevel),
    >,
) {
    MESSAGE_PROC = proc;
}

#[no_mangle]
pub unsafe extern "C" fn Fract_get_error_string(
    error_message: *mut *const c_char,
    error_message_size: *mut i32,
) {
    error_message.write(ERROR_STRING_WITHOUT_NULL.as_ptr() as *mut c_char);
    error_message_size.write(ERROR_STRING_WITHOUT_NULL.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn EventHandlerBuilder_default(
    event_handler_builder: *mut EventHandlerBuilder,
) {
    event_handler_builder.write(EventHandlerBuilder::default());
}

#[no_mangle]
pub unsafe extern "C" fn EventHandlerBuilder_build(
    event_handler_builder: *mut EventHandlerBuilder,
    event_handler: *mut EventHandler,
    event_callback: *mut EventCallback,
) -> i32 {
    let res = if event_handler_builder.is_null() {
        EventHandlerBuilder::default().build(event_handler, event_callback)
    } else {
        (&mut *event_handler_builder).build(event_handler, event_callback)
    };

    if let Err(e) = res {
        set_error_string(e);
        return 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn EventHandler_destroy(event_handler: *mut EventHandler) -> i32 {
    if let Err(e) = (&mut *event_handler).destroy() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn EventHandler_wait(event_handler: *mut EventHandler) {
    (&mut *event_handler).wait();
}

#[no_mangle]
pub unsafe extern "C" fn EventHandler_poll(event_handler: *mut EventHandler) -> i32 {
    (&mut *event_handler).poll()
}

#[no_mangle]
pub unsafe extern "C" fn EventHandler_dispatch(event_handler: *mut EventHandler) {
    (&mut *event_handler).dispatch();
}

#[no_mangle]
pub unsafe extern "C" fn WindowBuilder_default(window_builder: *mut WindowBuilder) {
    window_builder.write(WindowBuilder::default())
}

#[no_mangle]
pub unsafe extern "C" fn WindowBuilder_with_title(
    window_builder: *mut WindowBuilder,
    title: *const i8,
    title_size: i32,
) {
    let title_slice = std::slice::from_raw_parts(title as *const u8, title_size as usize);
    let title_str = std::str::from_utf8_unchecked(title_slice);

    (&mut *window_builder).with_title(title_str);
}

#[no_mangle]
pub unsafe extern "C" fn WindowBuilder_build(
    window_builder: *mut WindowBuilder,
    event_handler: *mut EventHandler,
    window: *mut WindowHandle,
) -> i32 {
    let res = if window_builder.is_null() {
        WindowBuilder::default().build(&mut *event_handler)
    } else {
        (&mut *window_builder).build(&mut *event_handler)
    };

    match res {
        Ok(w) => {
            window.write(w);
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn WindowHandle_destroy(window: WindowHandle) -> i32 {
    if let Err(e) = window.destroy() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn WindowHandle_show(window: WindowHandle) -> i32 {
    if let Err(e) = window.show() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn WindowHandle_update(window: WindowHandle) {
    window.update();
}

#[no_mangle]
pub unsafe extern "C" fn WindowHandle_redraw(window: WindowHandle) -> i32 {
    if let Err(e) = window.redraw() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn WindowHandle_get_dpi(window: WindowHandle, dpi: *mut u32) -> i32 {
    match window.get_dpi() {
        Ok(d) => {
            dpi.write(d as u32);
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

// !! TODO:(fractlysis) Make proper rect type
#[no_mangle]
pub unsafe extern "C" fn WindowHandle_get_window_rect(
    window: WindowHandle,
    rect: *mut Rect,
) -> i32 {
    match window.get_window_rect() {
        Ok(_r) => {
            rect.write(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            });
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

// !! TODO:(fractlysis) Make proper rect type
#[no_mangle]
pub unsafe extern "C" fn WindowHandle_get_client_rect(
    window: WindowHandle,
    rect: *mut Rect,
) -> i32 {
    match window.get_window_rect() {
        Ok(_r) => {
            rect.write(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            });
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

macro_rules!pfn_checked_call {
    ($pfn:expr, $($arg:expr),*) => {
        if let Some(pfn) = $pfn {
            (pfn)($($arg),*)
        }
    };
}

impl Event for EventCallback {
    #[allow(invalid_value)]
    #[inline]
    unsafe fn low_create(this: *mut Self, event_handler: &'static mut EventHandler) -> bool {
        assert!(!this.is_null());

        let this = &mut *this;
        pfn_checked_call!(this.create, this.userData, event_handler);

        true
    }

    #[inline]
    fn destroy(&mut self) {
        pfn_checked_call!(self.destroy, self.userData);
    }

    #[inline]
    #[cfg(target_os = "windows")]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        use windows_sys::Win32::Graphics::Gdi::{BeginPaint, EndPaint};

        unsafe {
            let mut paint = std::mem::zeroed();
            BeginPaint(window.windowHandle, &mut paint);

            pfn_checked_call!(self.render, self.userData, window);

            EndPaint(window.windowHandle, &paint);
        }
    }

    #[inline]
    #[cfg(not(target_os = "windows"))]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.render, self.userData, window);
    }

    #[inline]
    fn resized(&mut self, window: WindowHandle, width: i32, height: i32) {
        pfn_checked_call!(self.resized, self.userData, window, width, height);
    }

    #[inline]
    fn focused(&mut self, window: WindowHandle, focused: bool) {
        pfn_checked_call!(self.focused, self.userData, window, focused);
    }

    #[inline]
    fn minimized(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.minimized, self.userData, window);
    }

    #[inline]
    fn maximized(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.maximized, self.userData, window);
    }

    #[inline]
    fn moved(&mut self, window: WindowHandle, x: i32, y: i32) {
        pfn_checked_call!(self.moved, self.userData, window, x, y)
    }

    #[inline]
    fn close_requested(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.closeRequested, self.userData, window);
    }

    #[inline]
    fn destroyed(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.destroyed, self.userData, window);
    }

    #[inline]
    fn key_down(&mut self, window: WindowHandle, key: super::Key, scancode: super::ScanCode) {
        pfn_checked_call!(self.keyPressed, self.userData, window, key.0, scancode)
    }

    #[inline]
    fn key_up(&mut self, window: WindowHandle, key: super::Key, scancode: super::ScanCode) {
        pfn_checked_call!(self.keyReleased, self.userData, window, key.0, scancode)
    }

    #[inline]
    fn received_character(&mut self, window: WindowHandle, character: char) {
        pfn_checked_call!(self.charRecived, self.userData, window, character);
    }

    #[inline]
    fn mouse_wheel(&mut self, window: WindowHandle, delta_x: f32, delta_y: f32) {
        pfn_checked_call!(self.mouseWheel, self.userData, window, delta_x, delta_y);
    }

    #[inline]
    fn mouse_button_down(&mut self, window: WindowHandle, button: super::MouseButton) {
        pfn_checked_call!(
            self.mouseButtonPressed,
            self.userData,
            window,
            button.0 as i32
        );
    }

    #[inline]
    fn mouse_button_release(&mut self, window: WindowHandle, button: super::MouseButton) {
        pfn_checked_call!(
            self.mouseButtonReleased,
            self.userData,
            window,
            button.0 as i32
        );
    }

    #[inline]
    fn cursor_moved(&mut self, window: WindowHandle, position_x: i32, position_y: i32) {
        pfn_checked_call!(
            self.cursorMoved,
            self.userData,
            window,
            position_x,
            position_y
        );
    }

    #[inline]
    fn cursor_entered(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.cursorEntered, self.userData, window);
    }

    #[inline]
    fn cursor_left(&mut self, window: WindowHandle) {
        pfn_checked_call!(self.cursorLeft, self.userData, window);
    }

    #[inline]
    fn file_recived(&mut self, window: WindowHandle, mut path: String) {
        path.push('\0');
        pfn_checked_call!(
            self.fileRecived,
            self.userData,
            window,
            path.as_ptr().cast()
        );
    }

    // ─── UNIMPLEMENTED ──────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn create(_: &'static mut EventHandler) -> Option<Self> {
        unreachable!()
    }

    fn render(&mut self) {}
}
