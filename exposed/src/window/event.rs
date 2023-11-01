use super::{Context, Key, MouseButton, ScanCode, Touch, WindowHandle};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    #[default]
    Cancelled,
}

#[allow(unused)]
pub trait Event: Sized + 'static {
    fn create(context: Context) -> Option<Self> {
        None
    }

    fn on_create(context: Context) {}

    fn on_start(&mut self, context: Context) {}

    fn on_resume(&mut self, context: Context) {}

    fn on_pause(&mut self, context: Context) {}

    fn on_stop(&mut self, context: Context) {}

    // ─── WINDOW RECT ────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn render(&mut self, window: WindowHandle) {}

    #[inline]
    #[cfg(target_os = "windows")]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        use windows_sys::Win32::Graphics::Gdi::{BeginPaint, EndPaint};

        unsafe {
            let mut paint = std::mem::zeroed();
            BeginPaint(window.0 .0, &mut paint);

            self.render(window);

            EndPaint(window.0.0, &paint);
        }
    }

    #[inline]
    #[cfg(not(target_os = "windows"))]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        self.render(window);
    }

    fn resized(&mut self, window: WindowHandle, width: i32, height: i32) {}

    fn focused(&mut self, window: WindowHandle, focused: bool) {}

    fn minimized(&mut self, window: WindowHandle) {}

    fn show(&mut self, window: WindowHandle) {}

    fn maximized(&mut self, window: WindowHandle) {}

    fn moved(&mut self, window: WindowHandle, x: i32, y: i32) {}

    fn file_recived(&mut self, window: WindowHandle, path: String) {}

    fn close_requested(&mut self, window: WindowHandle) {}

    fn destroyed(&mut self, window: WindowHandle) {}

    // ─── Keyboard ───────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn key_down(&mut self, window: WindowHandle, key: Key, scancode: ScanCode) {}

    fn key_up(&mut self, window: WindowHandle, key: Key, scancode: ScanCode) {}

    fn received_character(&mut self, window: WindowHandle, character: char) {}

    // ─── Mouse ──────────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn mouse_wheel(&mut self, window: WindowHandle, delta_x: f32, delta_y: f32) {}

    fn mouse_button_down(&mut self, window: WindowHandle, button: MouseButton) {}

    fn mouse_button_release(&mut self, window: WindowHandle, button: MouseButton) {}

    fn cursor_moved(&mut self, window: WindowHandle, position_x: i32, position_y: i32) {}

    fn cursor_entered(&mut self, window: WindowHandle) {}

    fn cursor_left(&mut self, window: WindowHandle) {}

    fn raw_mouse_motion(&mut self, delta_x: i32, delta_y: i32) {}

    // ─── MOBILE LIKE ────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn touch(&mut self, window: WindowHandle, touch: Touch, pointer_count: usize) {}

    fn touch_end(&mut self, window: WindowHandle) {}

    fn axis_motion() {}

    fn scale_factor_changed() {}

    // ─── HELPER ─────────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    #[inline]
    #[cfg(target_os = "windows")]
    fn missed_events(
        hwnd: windows_sys::Win32::Foundation::HWND, msg: u32, wparam: windows_sys::Win32::Foundation::WPARAM,
        lparam: windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT {
        use windows_sys::Win32::UI::WindowsAndMessaging::DefWindowProcW;

        #[cfg(debug_assertions)]
        println!("Missed event hWnd: {hwnd} Msg: {msg} wParam: {wparam} lParam {lparam}");

        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }

    #[inline]
    #[cfg(target_os = "windows")]
    fn utf16_to_char_error(&mut self, e: std::char::DecodeUtf16Error) {
        eprintln!("{e}")
    }
}
