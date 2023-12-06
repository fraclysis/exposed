#[cfg(target_os = "windows")]
pub mod win32;

use std::fmt::Debug;

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

mod keys;
pub use keys::*;

pub mod utility;

mod window;
pub use window::*;

mod event_handler;
pub use event_handler::*;

pub use exposed_macro::android_on_create;

mod touch;
pub use touch::*;

pub use platform::Android;
pub use platform::Context;

#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MouseButton(pub u32);

impl MouseButton {
    pub const LEFT: Self = Self(1);
    pub const MIDDLE: Self = Self(2);
    pub const RIGHT: Self = Self(3);
    pub const X1: Self = Self(11);
    pub const X2: Self = Self(12);
}

impl Debug for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LEFT => write!(f, "MouseButton::LEFT"),
            Self::RIGHT => write!(f, "MouseButton::RIGHT"),
            Self::MIDDLE => write!(f, "MouseButton::MIDDLE"),
            Self::X1 => write!(f, "MouseButton::X1"),
            Self::X2 => write!(f, "MouseButton::X2"),
            _ => write!(f, "MouseButton({})", self.0),
        }
    }
}

pub type ScanCode = u32;

#[repr(C)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}
