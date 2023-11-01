use std::{
    f32::consts::PI,
    ffi::{c_double, c_float},
};

use exposed::window::Size;
use nalgebra_glm::{look_at, vec3, Mat4, Vec3};

#[derive(Debug, Default)]
pub struct Camera {
    pub radius: f32,
    pub polar: f32,
    pub azimuth: f32,
    pub eye: Vec3,
    pub view: Mat4,
    pub perspective: Mat4,
}

impl Camera {
    pub fn new(size: Size) -> Self {
        let mut this = Self {
            view: Mat4::identity(),
            perspective: Mat4::new_perspective(size.width as f32 / size.height as f32, PI / 3.0, 0.001, 100.0),
            radius: 15.0,
            polar: PI / 2.0,
            azimuth: PI + PI / 2.0,
            ..Default::default()
        };

        this.update();

        this
    }

    pub fn resize(&mut self, size: Size) {
        self.perspective = Mat4::new_perspective(size.width as f32 / size.height as f32, PI / 3.0, 0.001, 100.0);
    }

    pub fn orbit(&mut self, x: f32, y: f32) {
        self.azimuth -= x;
        self.polar += y;
    }

    pub fn update(&mut self) {
        const S: f32 = 0.01;

        if !(self.radius > 0.0) {
            self.radius = S;
        }

        if !(0.0 < self.polar) {
            self.polar = S;
        }

        if !(self.polar < PI) {
            self.polar = PI - S;
        }

        self.azimuth = unsafe { fmodf(self.azimuth, 2.0 * PI) };

        let radius = self.radius;
        let polar = self.polar;
        let azimuth = self.azimuth;

        let x = radius * polar.sin() * azimuth.cos();
        let y = radius * polar.cos();
        let z = radius * polar.sin() * azimuth.sin();

        self.eye = Vec3::new(x, y, z);

        self.view = look_at(&self.eye, &vec3(0.0, 0.0, 0.0), &vec3(0.0, 1.0, 0.0));
    }
}

#[allow(unused)]
extern "C" {
    fn fmod(a: c_double, b: c_double) -> c_double;
    fn fmodf(a: c_float, b: c_float) -> c_float;
}
