pub struct App {
    _window: exposed::destroy::Destroyable<exposed::window::WindowHandle>,
    running: bool,
}

impl exposed::window::Event for App {
    fn create(context: exposed::window::Context) -> Option<Self> {
        let window = exposed::destroy::Destroyable(exposed::window::WindowBuilder::default().build::<Self>(context).unwrap());

        window.show().unwrap();

        if let Err(e) = window.show() {
            eprintln!("{e}");
            return None;
        }

        Some(Self { _window: window, running: true })
    }

    fn close_requested(&mut self, _: exposed::window::WindowHandle) {
        self.running = false;
    }
}

impl exposed::window::utility::ExtendedEvent for App {
    #[inline]
    fn is_running(&mut self) -> bool {
        self.running
    }
}

exposed::window::android_on_create!(exposed::window::Android<App>);
