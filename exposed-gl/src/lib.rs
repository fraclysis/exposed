#[cfg(target_os = "windows")]
mod win32_opengl;
#[cfg(target_os = "windows")]
pub use win32_opengl::*;
#[cfg(target_os = "windows")]
pub mod wgl;
#[cfg(target_os = "windows")]
pub use wgl::*;

#[cfg(target_os = "linux")]
mod x11_gl;
#[cfg(target_os = "linux")]
pub use x11_gl::*;
#[cfg(target_os = "linux")]
pub mod glx;
#[cfg(target_os = "linux")]
pub use glx::*;

#[cfg(target_os = "windows")]
pub mod extern_ffi;
