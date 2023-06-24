#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub use win32::*;

#[cfg(target_os = "linux")]
mod _x11;
#[cfg(target_os = "linux")]
pub use _x11::*;

#[cfg(target_os = "android")]
mod _x11;
#[cfg(target_os = "android")]
pub use _x11::*;

mod event;
pub mod extern_ffi;
mod keys;
pub mod utility;

pub use event::*;
pub use keys::*;

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct MouseButton(pub u32);
pub type ScanCode = u32;

pub struct Size {
    pub width: i32,
    pub height: i32,
}
