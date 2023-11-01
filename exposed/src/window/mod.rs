#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "windows")]
pub use win32 as platform;

#[cfg(target_os = "linux")]
pub mod _x11;
#[cfg(target_os = "linux")]
pub use _x11 as platform;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use android as platform;

mod event;
pub use event::*;

/// cbindgen:ignore
mod keys;
/// cbindgen:ignore
pub use keys::*;

pub mod utility;

mod window;
pub use window::*;

mod event_handler;
pub use event_handler::*;

pub use exposed_macro;
pub use exposed_macro::android_on_create;

pub use platform::{Android, Context};

#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MouseButton(pub u32);

impl MouseButton {
    pub const LEFT: Self = Self(1);
    pub const RIGHT: Self = Self(2);
}

pub type ScanCode = u32;

pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Touch {
    pub phase: TouchPhase,
    pub location: (f32, f32),
    pub id: u64,
}
