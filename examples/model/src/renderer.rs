use std::{
    io::{Error, ErrorKind::Other},
    mem::size_of,
};

use exposed::log::cstr;
use gl::types::{GLint, GLuint};

use crate::{camera::Camera, model::Pmx};

pub fn gl_get_error() -> u32 {
    unsafe { gl::GetError() }
}

#[macro_export]
macro_rules! gl_log {
    () => {
        loop {
            let error = $crate::renderer::gl_get_error();
            if error == 0 {
                break;
            }
            exposed::log::log_error!("GlError", "[{}:{}] GL {}", file!(), line!(), error);
        }
    };
}

pub struct PmxRenderer {
    u_texture: GLint,
    u_mvp: GLint,
    program: GlProgram,
}

impl PmxRenderer {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::DepthMask(gl::TRUE);
        }

        let vertex_source = include_str!("../shaders/pmx.vert");
        let fragment_source = include_str!("../shaders/pmx.frag");

        let program = match create_program(vertex_source, fragment_source) {
            Ok(p) => GlProgram(p),
            Err(e) => return Err(Error::new(Other, e)),
        };

        let u_texture = unsafe { gl::GetUniformLocation(program.0, cstr!("u_texture")) };
        let u_mvp = unsafe { gl::GetUniformLocation(program.0, cstr!("u_mvp")) };
        gl_log!();

        Ok(Self { program, u_texture, u_mvp })
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        unsafe { gl::Viewport(0, 0, w, h) }
    }

    pub fn render(&mut self, model: &Pmx, camera: &Camera) {
        unsafe {
            gl_log!();

            gl::UseProgram(self.program.0);

            gl::BindVertexArray(model.vao.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, model.vertex_data.0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, model.index_data.0);

            let mvp = camera.perspective * camera.view * model.model;

            gl_log!();
            gl::UniformMatrix4fv(self.u_mvp, 1, gl::FALSE, mvp.as_ptr());
            gl_log!();

            for material in &model.gl_materials {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, material.gl_texture);
                gl::Uniform1i(self.u_texture, 0);

                gl::DrawElements(
                    gl::TRIANGLES,
                    material.index_size as i32,
                    gl::UNSIGNED_INT,
                    (material.index_start as usize * size_of::<u32>()) as _,
                );
            }
        }
    }
}

fn create_program(vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
    unsafe fn compile_shader(shader: u32, source: &str) -> Result<(), String> {
        #[cfg(not(target_os = "android"))]
        let version_info = "#version 430 core\n";
        #[cfg(target_os = "android")]
        let version_info = "#version 300 es\nprecision mediump float;";

        gl::ShaderSource(
            shader,
            2,
            [version_info.as_ptr().cast(), source.as_ptr()].as_ptr().cast(),
            [version_info.len() as i32, source.len() as i32].as_ptr(),
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

pub struct GlTexture(pub GLuint);

impl Drop for GlTexture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.0) };
        gl_log!();
    }
}

pub struct GlBuffer(pub GLuint);

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.0) };
        gl_log!();
    }
}

pub struct GlProgram(pub GLuint);

impl Drop for GlProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) };
        gl_log!();
    }
}

pub struct GlVao(pub GLuint);

impl Drop for GlVao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.0) };
        gl_log!();
    }
}
