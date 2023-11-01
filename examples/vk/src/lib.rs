#![allow(dead_code)]
#![allow(unused_variables)]

use exposed::{
    destroy::Destroyable,
    log::log_error,
    window::{android_on_create, utility, Android, Context, Event, WindowBuilder, WindowHandle},
};
use renderer::Renderer;

mod renderer;
mod shader;

const IS_ANDROID: bool = cfg!(target_os = "android");

pub struct App {
    renderer: Renderer,
    window: Destroyable<WindowHandle>,
    context: Context,
    running: bool,
}

impl App {
    fn create_(context: Context) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Destroyable(WindowBuilder::default().build::<Self>(context)?);

        window.show()?;

        let renderer = unsafe { Renderer::new(window.0).unwrap() };

        Ok(Self { renderer, window, context, running: true })
    }
}

impl Event for App {
    fn create(context: Context) -> Option<Self> {
        match Self::create_(context) {
            Ok(app) => Some(app),
            Err(e) => {
                log_error!("VkExample", "{e}");
                None
            }
        }
    }

    fn resized(&mut self, window: WindowHandle, width: i32, height: i32) {
        // Recreate swapchains
        log_error!("Example", "Resized.");

        if width * height != 0 {
            self.renderer.recreate_swapchains(width as _, height as _)
        }
    }

    fn show(&mut self, window: WindowHandle) {
        if IS_ANDROID {
            self.renderer.create_surface(window)
        }
    }

    fn minimized(&mut self, window: WindowHandle) {
        if IS_ANDROID {
            self.renderer.destroy_surface(window)
        }
    }

    fn close_requested(&mut self, window: WindowHandle) {
        self.running = false;
    }

    fn render(&mut self, window: WindowHandle) {
        self.renderer.render().unwrap();
    }

    fn key_up(
        &mut self, window: WindowHandle, key: exposed::window::Key,
        scancode: exposed::window::ScanCode,
    ) {
        if key == exposed::window::Key::KEY_T {
            let window = Destroyable(WindowBuilder::default().build::<Self>(self.context).unwrap());
            log_error!("Example", "New window created.");

            window.show().unwrap();

            self.renderer.destroy_surface(self.window.0);
            log_error!("Example", "Old window surface destroyed.");

            self.renderer.create_surface(window.0);
            log_error!("Example", "New window surface created.");

            self.window = window;
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {}
}

impl utility::ExtendedEvent for App {
    fn is_running(&mut self) -> bool {
        self.running
    }
}

android_on_create!(Android<App>);
