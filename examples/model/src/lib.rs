mod camera;
mod model;
mod renderer;

use std::{
    ffi::CString,
    io::{Error, ErrorKind},
};

use camera::Camera;
use exposed::{
    destroy::Destroyable,
    log::{log_fatal, LogResult},
    window::{
        android_on_create, utility::ExtendedEvent, Android, Context, Event, Key, MouseButton, ScanCode, Size, Touch, TouchPhase,
        WindowBuilder, WindowHandle,
    },
};
use exposed_gl::{get_proc_addr, load_lib_opengl, surface_config, tokens, GlContext, GlDefaultPicker, GlSurface};
use model::Pmx;
use nalgebra_glm::vec2;
use renderer::PmxRenderer;

const IS_ANDROID: bool = cfg!(target_os = "android");
const SENSITIVITY: f32 = 0.01;

const SURFACE_CONFIG: &[u32] = &surface_config!(tokens::DEPTH_BITS_ARB, 1,);

pub struct App {
    render: bool,
    clicked: bool,
    last_x: i32,
    last_y: i32,
    camera: Camera,
    touch_count: usize,
    touch_buffer: [Touch; 2],
    models: Vec<Pmx>,
    pmx_renderer: PmxRenderer,
    gl_context: Destroyable<GlContext>,
    surface: Option<Destroyable<GlSurface>>,
    window: Destroyable<WindowHandle>,
    running: bool,
}

impl App {
    fn new(context: Context) -> Result<Self, Error> {
        let env: Vec<String> = std::env::args().collect();

        if env.len() != 2 {
            return Err(Error::new(ErrorKind::Other, format!("Use {} <PMX_PATH>", env[0])));
        }

        let model_path = &env[1];

        load_lib_opengl()?;

        let (surface, window) =
            GlSurface::build_with::<Self>(&WindowBuilder::default(), context, SURFACE_CONFIG, &mut GlDefaultPicker::default())?;

        let window = Destroyable(window);
        let surface = Destroyable(surface);

        #[rustfmt::skip]
        let context_config = [
            tokens::CONTEXT_MAJOR_VERSION_ARB, 3,
            tokens::CONTEXT_MINOR_VERSION_ARB, 0,
            tokens::CONTEXT_FLAGS_ARB, tokens::CONTEXT_DEBUG_BIT_ARB,
            tokens::END,
        ];

        let gl_context = surface.create_context(&context_config, GlContext::NO_CONTEXT)?;

        let gl_context = Destroyable(gl_context);

        surface.make_current(gl_context.0)?;
        surface.set_swap_interval(1).log_error();

        gl::load_with(|symbol| match CString::new(symbol) {
            Ok(c_symbol) => unsafe { get_proc_addr(c_symbol.as_ptr()) },
            Err(e) => {
                eprintln!("{e}");
                std::ptr::null()
            }
        });

        #[cfg(target_os = "windows")]
        unsafe {
            extern "system" fn debug_callback(
                _source: gl::types::GLenum, _gltype: gl::types::GLenum, _id: gl::types::GLuint, _severity: gl::types::GLenum,
                _length: gl::types::GLsizei, message: *const gl::types::GLchar, _user_param: *mut std::ffi::c_void,
            ) {
                unsafe {
                    let m = std::ffi::CStr::from_ptr(message).to_str().unwrap();
                    println!("{m}")
                }
            }

            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null())
        };

        let pmx_renderer = PmxRenderer::new()?;

        window.show()?;

        let mut models = Vec::new();

        models.push(Pmx::new(model_path, &pmx_renderer).unwrap());

        let camera = Camera::new(window.client_size()?);

        Ok(Self {
            pmx_renderer,
            gl_context,
            surface: Some(surface),
            window,
            running: true,
            models,
            camera,
            last_x: 0,
            last_y: 0,
            clicked: false,
            render: true,
            touch_count: Default::default(),
            touch_buffer: Default::default(),
        })
    }

    fn redraw(&mut self) {
        if !self.is_animating() {
            self.window.redraw().log_error()
        }
    }
}

impl Event for App {
    fn show(&mut self, window: WindowHandle) {
        if IS_ANDROID {
            let surface = Destroyable(GlSurface::build::<Self>(window, SURFACE_CONFIG, &mut GlDefaultPicker::default()).unwrap());

            surface.make_current(self.gl_context.0).unwrap();

            self.surface = Some(surface);
        }
    }

    fn minimized(&mut self, _: WindowHandle) {
        if IS_ANDROID {
            if let Some(display) = &mut self.surface {
                display.make_current(exposed_gl::GlContext::NO_CONTEXT).log_error();
            }
            self.surface = None;
        }
    }

    fn resized(&mut self, _: WindowHandle, width: i32, height: i32) {
        if width * height == 0 {
            return;
        }

        self.camera.resize(Size { width, height });

        self.pmx_renderer.resize(width, height);
    }

    fn close_requested(&mut self, _: WindowHandle) {
        self.running = false;
    }

    fn render(&mut self, _: WindowHandle) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };

        if self.render {
            for m in &self.models {
                self.pmx_renderer.render(m, &self.camera);
            }
        }

        if let Some(surface) = &mut self.surface {
            surface.swap_buffers().log_error();
        }
    }

    fn mouse_wheel(&mut self, _: WindowHandle, _delta_x: f32, delta_y: f32) {
        self.camera.radius -= delta_y * 0.8;

        self.camera.update();

        self.redraw();
    }

    fn cursor_moved(&mut self, _: WindowHandle, position_x: i32, position_y: i32) {
        let dx = self.last_x - position_x;
        let dy = self.last_y - position_y;
        self.last_x = position_x;
        self.last_y = position_y;

        if self.clicked {
            let x = dx as f32 * SENSITIVITY;
            let y = dy as f32 * SENSITIVITY;

            self.camera.orbit(x, y);
            self.camera.update();

            self.redraw();
        }
    }

    fn mouse_button_down(&mut self, window: WindowHandle, button: MouseButton) {
        if button == MouseButton::LEFT {
            self.clicked = true;
            window.set_capture();
        }
    }

    fn mouse_button_release(&mut self, window: WindowHandle, button: MouseButton) {
        if button == MouseButton::LEFT {
            self.clicked = false;
            window.release_capture().log_error();
        }
    }

    fn key_up(&mut self, _: WindowHandle, key: Key, _: ScanCode) {
        if key == Key::SPACE {
            self.render = false;
        }

        self.redraw();
    }

    fn key_down(&mut self, _: WindowHandle, key: Key, _: ScanCode) {
        const STEP: f32 = 0.01745329252 * 2.0;

        match key {
            Key::UP_ARROW => {
                self.camera.orbit(STEP, 0.0);
            }
            Key::DOWN_ARROW => {
                self.camera.orbit(-STEP, 0.0);
            }
            Key::LEFT_ARROW => {
                self.camera.orbit(0.0, STEP);
            }
            Key::RIGHT_ARROW => {
                self.camera.orbit(0.0, -STEP);
            }
            Key::ESCAPE => {
                self.running = false;
            }
            Key::SPACE => {
                self.render = true;
            }
            _ => (),
        }

        self.camera.update();

        self.redraw();
    }

    fn touch(&mut self, _: WindowHandle, touch: Touch, _: usize) {
        // match touch.phase {
        //     TouchPhase::Moved | TouchPhase::Ended => {
        //         let dx = touch.location.0 - self.old_touch.location.0;
        //         let dy = touch.location.1 - self.old_touch.location.1;

        //         self.camera.orbit(dx * SENSITIVITY, -dy * SENSITIVITY);
        //         self.camera.update();
        //         self.redraw();
        //     }
        //     _ => {}
        // }

        // self.old_touch = touch;

        // We collect the touch events
        if self.touch_count < self.touch_buffer.len() {
            self.touch_buffer[self.touch_count] = touch;
        }
        self.touch_count += 1;
    }

    fn touch_end(&mut self, _: WindowHandle) {
        match self.touch_count {
            1 => {
                if self.touch_buffer[0].phase == TouchPhase::Moved {
                    if let Some((ox, oy)) = self.touch_buffer[0].historical() {
                        let (nx, ny) = self.touch_buffer[0].location;

                        let dx = nx - ox;
                        let dy = ny - oy;

                        self.camera.orbit(dx * SENSITIVITY, -dy * SENSITIVITY);
                        self.camera.update();
                        self.redraw();
                    }
                }
            }
            2 => {
                if self.touch_buffer[0].phase == TouchPhase::Moved && self.touch_buffer[1].phase == TouchPhase::Moved {
                    let old_len = {
                        let a0 = self.touch_buffer[0].historical().unwrap_or(self.touch_buffer[0].location);
                        let a1 = self.touch_buffer[1].historical().unwrap_or(self.touch_buffer[1].location);
                        let a0 = vec2(a0.0, a0.1);
                        let a1 = vec2(a1.0, a1.1);
                        a1.metric_distance(&a0)
                    };

                    let len = {
                        let a0 = self.touch_buffer[0].location;
                        let a1 = self.touch_buffer[1].location;

                        let a0 = vec2(a0.0, a0.1);
                        let a1 = vec2(a1.0, a1.1);
                        a1.metric_distance(&a0)
                    };

                    let dz = len - old_len;
                    self.camera.radius += SENSITIVITY * dz;
                    self.camera.update();
                    self.redraw();
                }
            }
            _ => {}
        }

        self.touch_count = 0;
    }

    fn create(context: Context) -> Option<Self> {
        match Self::new(context) {
            Ok(s) => Some(s),
            Err(e) => {
                log_fatal!("Model", "{e}");
                None
            }
        }
    }
}

impl ExtendedEvent for App {
    fn is_running(&mut self) -> bool {
        self.running
    }

    fn is_animating(&mut self) -> bool {
        false
    }

    fn polled(&mut self) {
        self.render(self.window.0);
    }
}

android_on_create!(Android<App>);
