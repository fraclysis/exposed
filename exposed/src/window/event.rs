use crate::window::EventHandler;

use super::{Key, MouseButton, ScanCode, WindowHandle};

#[allow(unused)]
pub trait Event: Sized + 'static {
    unsafe fn low_create(this: *mut Self, event_handler: &'static mut EventHandler<Self>) -> bool {
        if let Some(success) = Self::create(event_handler) {
            this.write(success);
            true
        } else {
            false
        }
    }

    fn create(event_handler: &'static mut EventHandler<Self>) -> Option<Self>;

    fn destroy(&mut self);

    // ─── WINDOW RECT ────────────────────────────────────────────────────────────────
    // ────────────────────────────────────────────────────────────────────────────────

    fn render(&mut self) {}

    #[inline]
    #[cfg(target_os = "windows")]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        use windows_sys::Win32::Graphics::Gdi::{BeginPaint, EndPaint};

        unsafe {
            let mut paint = std::mem::zeroed();
            BeginPaint(window.windowHandle, &mut paint);

            self.render();

            EndPaint(window.windowHandle, &paint);
        }
    }

    #[inline]
    #[cfg(not(target_os = "windows"))]
    /// Use full for validating window in windows
    fn low_render(&mut self, window: WindowHandle) {
        self.render();
    }

    fn resized(&mut self, window: WindowHandle, width: i32, height: i32) {}

    fn focused(&mut self, window: WindowHandle, focused: bool) {}

    fn minimized(&mut self, window: WindowHandle) {}

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

    fn touch(&mut self, window: WindowHandle, touch: ()) {}

    fn axis_motion() {}

    fn scale_factor_changed() {}
}
