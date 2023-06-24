fn main() -> Result<(), std::io::Error> {
    exposed::window::utility::run::<App>(Default::default())
}

struct App {
    window: exposed::window::WindowHandle,
    event_handler: &'static mut exposed::window::EventHandler<Self>,
}

impl exposed::window::Event for App {
    fn create(event_handler: &'static mut exposed::window::EventHandler<Self>) -> Option<Self> {
        let window = exposed::window::WindowBuilder::default()
            .with_size(300, 200)
            .build(event_handler)
            .unwrap();

        if let Err(e) = window.show() {
            event_handler.running = false;
            eprintln!("{e}");
        }

        Some(Self {
            window,
            event_handler,
        })
    }

    fn destroy(&mut self) {
        if let Err(e) = self.window.destroy() {
            eprintln!("{e}");
        };
    }

    fn close_requested(&mut self, _: exposed::window::WindowHandle) {
        self.event_handler.running = false;
    }

    fn key_down(
        &mut self,
        _window: exposed::window::WindowHandle,
        key: exposed::window::Key,
        _scancode: exposed::window::ScanCode,
    ) {
        println!("Pressed:  {key:?}");
    }

    fn key_up(
        &mut self,
        _window: exposed::window::WindowHandle,
        key: exposed::window::Key,
        _scancode: exposed::window::ScanCode,
    ) {
        println!("Released: {key:?}");
    }

    fn received_character(&mut self, _window: exposed::window::WindowHandle, character: char) {
        println!("Char:     {character:?}");
    }
}

impl exposed::window::utility::ExtendedEvent for App {}
