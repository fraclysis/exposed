use crate::triangle::Triangle;

use exposed::{
    destroy::Destroyable,
    window::{utility::ExtendedEvent, Context, Event, WindowBuilder, WindowHandle},
};

pub struct App {
    _triangle: Triangle,
    _window: Destroyable<WindowHandle>,
    running: bool,
}

impl Event for App {
    fn create(context: Context) -> Option<Self> {
        let window = WindowBuilder::default().build::<Self>(context).unwrap();

        if let Err(e) = window.show() {
            eprintln!("{e}");
        }

        #[cfg(target_os = "windows")]
        let w = {
            let mut w = raw_window_handle::Win32WindowHandle::empty();
            w.hwnd = window.0 .0 as _;
            w.hinstance = unsafe { exposed::window::win32::HINSTANCE } as _;
            raw_window_handle::RawWindowHandle::Win32(w)
        };
        #[cfg(target_os = "linux")]
        let w = {
            let mut w = raw_window_handle::XlibWindowHandle::empty();
            w.window = window.0 .0;
            raw_window_handle::RawWindowHandle::Xlib(w)
        };

        #[cfg(target_os = "linux")]
        let d = {
            let mut d = raw_window_handle::XlibDisplayHandle::empty();
            let c = unsafe { exposed::window::_x11::ThreadContext::current_thread() };
            d.display = c.display as _;
            d.screen = c.screen_id;
            raw_window_handle::RawDisplayHandle::Xlib(d)
        };

        #[cfg(target_os = "windows")]
        let d = { raw_window_handle::RawDisplayHandle::Windows(raw_window_handle::WindowsDisplayHandle::empty()) };

        #[cfg(target_os = "android")]
        let w = {
            let mut w = raw_window_handle::AndroidNdkWindowHandle::empty();
            w.a_native_window = window.0.native_handle() as _;
            raw_window_handle::RawWindowHandle::AndroidNdk(w)
        };
        #[cfg(target_os = "android")]
        let d = { raw_window_handle::RawDisplayHandle::Android(raw_window_handle::AndroidDisplayHandle::empty()) };

        let s = window.client_size().unwrap();

        let triangle = unsafe { Triangle::new(w, d, s).unwrap() };

        Some(Self { _triangle: triangle, _window: Destroyable::new(window), running: true })
    }

    fn close_requested(&mut self, _: WindowHandle) {
        self.running = false;
    }

    fn render(&mut self, _: WindowHandle) {
        unsafe { self._triangle.render() };
    }
}

impl ExtendedEvent for App {
    #[inline]
    fn is_running(&mut self) -> bool {
        self.running
    }
}
