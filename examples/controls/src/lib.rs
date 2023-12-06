use exposed::{log::log_info, window::Context};

pub struct App {
    _window: exposed::destroy::Destroyable<exposed::window::WindowHandle>,
    running: bool,
}

impl exposed::window::Event for App {
    fn create(context: Context) -> Option<Self> {
        let window = exposed::destroy::Destroyable(
            exposed::window::WindowBuilder::default().with_size(300, 200).build::<Self>(context).unwrap(),
        );

        if let Err(e) = window.show() {
            eprintln!("{e}");
            return None;
        }

        Some(Self { _window: window, running: true })
    }

    fn close_requested(&mut self, _: exposed::window::WindowHandle) {
        self.running = false;
    }

    fn key_down(
        &mut self, _window: exposed::window::WindowHandle, key: exposed::window::Key, _scancode: exposed::window::ScanCode,
    ) {
        log_info!("Exposed Example", "Pressed:  {key:?}");
    }

    fn key_up(
        &mut self, _window: exposed::window::WindowHandle, key: exposed::window::Key, _scancode: exposed::window::ScanCode,
    ) {
        log_info!("Exposed Example", "Released: {key:?}");
    }

    fn received_character(&mut self, _window: exposed::window::WindowHandle, character: char) {
        log_info!("Exposed Example", "Char:     {character:?}");
    }

    fn mouse_button_down(&mut self, _: exposed::window::WindowHandle, button: exposed::window::MouseButton) {
        log_info!("Exposed Example", "Pressed:  {button:?}");
    }

    fn mouse_button_release(&mut self, _: exposed::window::WindowHandle, button: exposed::window::MouseButton) {
        log_info!("Exposed Example", "Released: {button:?}");
    }
}

impl exposed::window::utility::ExtendedEvent for App {
    fn is_running(&mut self) -> bool {
        self.running
    }
}

exposed::window::android_on_create!(exposed::window::Android<App>);
