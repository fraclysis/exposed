fn main() -> Result<(), std::io::Error> {
    exposed::window::utility::run::<App>(Default::default())
}

struct App {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    program: gl::types::GLuint,
    context: exposed_gl::GlContext,
    display: exposed_gl::GlDisplay,
    window: exposed::window::WindowHandle,
    event_handler: &'static mut exposed::window::EventHandler<Self>,
}

impl exposed::window::Event for App {
    fn create(event_handler: &'static mut exposed::window::EventHandler<Self>) -> Option<Self> {
        exposed_gl::load_lib_opengl().unwrap();

        let (display, window) = exposed_gl::GlDisplayBuilder::default()
            .build_with(exposed::window::WindowBuilder::default(), event_handler)
            .unwrap();

        let context = exposed_gl::GlContextBuilder::default()
            .build(display, exposed_gl::GlContext::NO_CONTEXT)
            .unwrap();

        display.make_current(context).unwrap();

        gl::load_with(|symbol| match std::ffi::CString::new(symbol) {
            Ok(c_symbol) => unsafe { exposed_gl::get_proc_addr(c_symbol.as_ptr()) },
            Err(e) => {
                eprintln!("{e}");
                std::ptr::null()
            }
        });

        unsafe { gl::ClearColor(1.0, 1.0, 0.0, 1.0) };

        let vertex_source = r#"
            #version 330
        
            in vec2 vPosition;
            in vec3 vColor;
            out vec3 fColor;
        
            void main() {
                fColor = vColor;
                gl_Position =  vec4(vPosition, 0.0, 1.0);
            }
        "#;

        let fragment_source = r#"
            #version 330 core

            in vec3 fColor;
            out vec4 FragColor;

            void main() { 
                FragColor = vec4(fColor ,1.0);
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

        if let Err(e) = window.show() {
            event_handler.running = false;
            eprintln!("{e}");
        }

        Some(Self {
            vao,
            vbo,
            program,
            context,
            display,
            window,
            event_handler,
        })
    }

    fn render(&mut self) {
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

        if let Err(e) = self.display.swap_buffers() {
            eprintln!("{e}");
        }
    }

    fn resized(&mut self, _window: exposed::window::WindowHandle, width: i32, height: i32) {
        if (width == 0) | (height == 0) {
            return;
        }

        unsafe { gl::Viewport(0, 0, width, height) };
    }

    fn close_requested(&mut self, _: exposed::window::WindowHandle) {
        self.event_handler.running = false;
    }

    fn destroy(&mut self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
        unsafe { gl::BindVertexArray(0) };

        unsafe { gl::DeleteBuffers(1, &self.vbo) };
        unsafe { gl::DeleteVertexArrays(1, &self.vao) };

        unsafe { gl::UseProgram(0) };
        unsafe { gl::DeleteProgram(self.program) };

        if let Err(e) = self.context.destroy() {
            eprintln!("{e}");
        };

        if let Err(e) = self.display.destroy() {
            eprintln!("{e}");
        };

        if let Err(e) = self.window.destroy() {
            eprintln!("{e}");
        };
    }
}

impl exposed::window::utility::ExtendedEvent for App {}

fn create_program(vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
    unsafe fn compile_shader(shader: u32, source: &str) -> Result<(), String> {
        gl::ShaderSource(
            shader,
            1,
            [source.as_ptr()].as_ptr() as _,
            [source.len()].as_ptr() as _,
        );
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
