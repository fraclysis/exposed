use exposed::log::{log_error, log_verbose};
use exposed_gl::GlSurface;

const IS_ANDROID: bool = cfg!(target_os = "android");

#[allow(unused)]
pub struct App {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    program: gl::types::GLuint,
    context: exposed::destroy::Destroyable<exposed_gl::GlContext>,
    display: Option<exposed::destroy::Destroyable<exposed_gl::GlSurface>>,
    _window: exposed::destroy::Destroyable<exposed::window::WindowHandle>,
    running: bool,
}


impl exposed::window::Event for App {
    fn create(context: exposed::window::Context) -> Option<Self> {
        exposed_gl::load_lib_opengl().unwrap();

        let mut picker = exposed_gl::GlDefaultPicker::default();

        let (display, window) = exposed_gl::GlSurface::build_with::<Self>(
            &exposed::window::WindowBuilder::default(),
            context,
            &exposed_gl::surface_config!(),
            &mut picker,
        )
        .unwrap();

        let window = exposed::destroy::Destroyable(window);
        let display = exposed::destroy::Destroyable(display);

        #[rustfmt::skip]
        let context_config = [
            exposed_gl::tokens::CONTEXT_MAJOR_VERSION_ARB, 3,
            exposed_gl::tokens::CONTEXT_MINOR_VERSION_ARB, 0,
            exposed_gl::tokens::CONTEXT_FLAGS_ARB, exposed_gl::tokens::CONTEXT_DEBUG_BIT_ARB,
            exposed_gl::tokens::END,
        ];

        let context = display.create_context(&context_config, exposed_gl::GlContext::NO_CONTEXT).unwrap();

        let context = exposed::destroy::Destroyable(context);

        display.make_current(context.0).unwrap();

        gl::load_with(|symbol| match std::ffi::CString::new(symbol) {
            Ok(c_symbol) => unsafe { exposed_gl::get_proc_addr(c_symbol.as_ptr()) },
            Err(e) => {
                eprintln!("{e}");
                std::ptr::null()
            }
        });

        log_gl_info();

        unsafe { gl::ClearColor(0.0, 0.0, 0.0, 0.0) };
        unsafe { gl::Enable(gl::FRAMEBUFFER_SRGB) };

        let _vertex_source = r#"
            #version 330
        
            in vec2 vPosition;
            in vec3 vColor;
            out vec3 fColor;
        
            void main() {
                fColor = vColor;
                gl_Position =  vec4(vPosition, 0.0, 1.0);
            }
        "#;

        let vertex_source = r#"
            #version 100
            precision mediump float;

            attribute vec2 vPosition;
            attribute vec3 vColor;

            varying vec3 fColor;

            void main() {
                gl_Position = vec4(vPosition, 0.0, 1.0);
                fColor = vColor;
            }
        "#;

        let _fragment_source = r#"
            #version 330 core

            in vec3 fColor;
            out vec4 FragColor;

            void main() { 
                FragColor = vec4(fColor ,1.0);
            }
        "#;

        let fragment_source = r#"
            #version 100
            precision mediump float;

            varying vec3 fColor;

            void main() {
                gl_FragColor = vec4(fColor, 1.0);
            }
        "#;

        let program = create_program(vertex_source, fragment_source).unwrap();

        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        unsafe { gl::BindVertexArray(vao) };

        #[rustfmt::skip]
        let vertex_data: [f32; 15] = [
           -0.5, -0.5,  1.0,  0.0,  0.0,
            0.0,  0.5,  0.0,  1.0,  0.0,
            0.5, -0.5,  0.0,  0.0,  1.0,
        ];

        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo) };
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * vertex_data.len()) as _,
                vertex_data.as_ptr() as _,
                gl::STATIC_DRAW,
            )
        };

        let pos_attrib = unsafe { gl::GetAttribLocation(program, b"vPosition\0".as_ptr() as _) };
        let color_attrib = unsafe { gl::GetAttribLocation(program, b"vColor\0".as_ptr() as _) };

        unsafe {
            gl::VertexAttribPointer(
                pos_attrib as _,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            )
        };

        unsafe {
            gl::VertexAttribPointer(
                color_attrib as _,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                (2 * std::mem::size_of::<f32>()) as usize as _,
            )
        };

        unsafe { gl::EnableVertexAttribArray(pos_attrib as _) };
        unsafe { gl::EnableVertexAttribArray(color_attrib as _) };

        let mut running = true;
        if let Err(e) = window.show() {
            running = false;
            eprintln!("{e}");
        }

        Some(Self { vao, vbo, program, context, display: Some(display), _window: window, running })
    }

    fn render(&mut self, _: exposed::window::WindowHandle) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        if let Some(display) = &mut self.display {
            if let Err(e) = display.swap_buffers() {
                eprintln!("{e}");
            }
        }
    }

    fn resized(&mut self, _window: exposed::window::WindowHandle, width: i32, height: i32) {
        if (width == 0) | (height == 0) {
            return;
        }

        unsafe { gl::Viewport(0, 0, width, height) };
    }

    fn close_requested(&mut self, _: exposed::window::WindowHandle) {
        self.running = false;
    }

    fn show(&mut self, window: exposed::window::WindowHandle) {
        if IS_ANDROID {
            let display = exposed::destroy::Destroyable(GlSurface::build::<Self>(window, &exposed_gl::surface_config!(), &mut exposed_gl::GlDefaultPicker::default()).unwrap());

            display.make_current(self.context.0).unwrap();

            self.display = Some(display);
        }
    }

    fn minimized(&mut self, _: exposed::window::WindowHandle) {
        if IS_ANDROID {
            if let Some(display) = &mut self.display {
                if let Err(e) = display.make_current(exposed_gl::GlContext::NO_CONTEXT) {
                    log_error!("Exposed", "{e}");
                }
            }
            self.display = None;
        }
    }
}

impl exposed::window::utility::ExtendedEvent for App {
    fn is_running(&mut self) -> bool {
        self.running
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
        unsafe { gl::BindVertexArray(0) };

        unsafe { gl::DeleteBuffers(1, &self.vbo) };
        unsafe { gl::DeleteVertexArrays(1, &self.vao) };

        unsafe { gl::UseProgram(0) };
        unsafe { gl::DeleteProgram(self.program) };
    }
}

fn create_program(vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
    unsafe fn compile_shader(shader: u32, source: &str) -> Result<(), String> {
        gl::ShaderSource(shader, 1, [source.as_ptr()].as_ptr() as _, [source.len()].as_ptr() as _);
        gl::CompileShader(shader);

        let mut result = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
        if result as u32 == 0 {
            let mut length = 0;

            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);

            let mut message: Vec<u8> = Vec::with_capacity(length as usize);
            unsafe { message.set_len(length as usize) };
            gl::GetShaderInfoLog(shader, length, &mut length, message.as_mut_ptr() as _);

            gl::DeleteShader(shader);

            if let Ok(message) = String::from_utf8(message) {
                return Err(message);
            } else {
                eprintln!("Cannot read message!");
                todo!();
            }
        }

        Ok(())
    }

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    unsafe { compile_shader(vertex_shader, vertex_source).unwrap() };
    unsafe { compile_shader(fragment_shader, fragment_source).unwrap() };

    let program = unsafe { gl::CreateProgram() };

    unsafe { gl::AttachShader(program, vertex_shader) };
    unsafe { gl::AttachShader(program, fragment_shader) };

    unsafe { gl::LinkProgram(program) };
    unsafe { gl::UseProgram(program) };

    unsafe { gl::DeleteShader(vertex_shader) };
    unsafe { gl::DeleteShader(fragment_shader) };

    let mut result = 0;
    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut result) };
    if result as u32 == 0 {
        let mut length = 0;

        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length) };

        let mut message: Vec<u8> = Vec::with_capacity(length as usize);
        unsafe { message.set_len(length as usize) };
        unsafe { gl::GetProgramInfoLog(program, length, &mut length, message.as_mut_ptr() as _) };

        if let Ok(message) = String::from_utf8(message) {
            eprintln!("{message}");
        } else {
            eprintln!("Cannot read message!");
        }

        unsafe { gl::DeleteProgram(program) };
        todo!("Error return");
    }

    Ok(program)
}

exposed::window::android_on_create!(exposed::window::Android<App>);

fn log_gl_info() {
    fn gl_str<'a>(name: u32) -> &'a str {
        unsafe {
            let cstr = gl::GetString(name);

            if let Ok(s) = std::ffi::CStr::from_ptr(cstr as _).to_str() {
                s
            } else {
                "<Error>"
            }
        }
    }

    log_verbose!("Exposed", "Version: {}", gl_str(gl::VERSION));
}
