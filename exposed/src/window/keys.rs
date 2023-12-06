// This file is automatically @generated.
// It is not intended for manual editing.
use std::fmt::Debug;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Wraps inner os specific virtual key to make matching more easier.
/// ```no_run
/// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
/// let key = Key(VK_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
///
/// use ndk_sys::AKEYCODE_ESCAPE;
/// let key = Key(AKEYCODE_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
/// ```
pub struct Key(pub u64);

#[cfg(target_os = "windows")]
impl Key {
    pub const TAB: Self = Self(VK_TAB as u64);
    pub const LEFT_ARROW: Self = Self(VK_LEFT as u64);
    pub const RIGHT_ARROW: Self = Self(VK_RIGHT as u64);
    pub const UP_ARROW: Self = Self(VK_UP as u64);
    pub const DOWN_ARROW: Self = Self(VK_DOWN as u64);
    pub const PAGE_UP: Self = Self(VK_PRIOR as u64);
    pub const PAGE_DOWN: Self = Self(VK_NEXT as u64);
    pub const HOME: Self = Self(VK_HOME as u64);
    pub const END: Self = Self(VK_END as u64);
    pub const INSERT: Self = Self(VK_INSERT as u64);
    pub const DELETE: Self = Self(VK_DELETE as u64);
    pub const BACKSPACE: Self = Self(VK_BACK as u64);
    pub const SPACE: Self = Self(VK_SPACE as u64);
    pub const ENTER: Self = Self(VK_RETURN as u64);
    pub const ESCAPE: Self = Self(VK_ESCAPE as u64);
    pub const APOSTROPHE: Self = Self(VK_OEM_7 as u64);
    pub const COMMA: Self = Self(VK_OEM_COMMA as u64);
    pub const MINUS: Self = Self(VK_OEM_MINUS as u64);
    pub const PERIOD: Self = Self(VK_OEM_PERIOD as u64);
    pub const SLASH: Self = Self(VK_OEM_2 as u64);
    pub const SEMICOLON: Self = Self(VK_OEM_1 as u64);
    pub const EQUAL: Self = Self(VK_OEM_PLUS as u64);
    pub const LEFT_BRACKET: Self = Self(VK_OEM_4 as u64);
    pub const BACKSLASH: Self = Self(VK_OEM_5 as u64);
    pub const RIGHT_BRACKET: Self = Self(VK_OEM_6 as u64);
    pub const GRAVEACCENT: Self = Self(VK_OEM_3 as u64);
    pub const CAPSLOCK: Self = Self(VK_CAPITAL as u64);
    pub const SCROLLLOCK: Self = Self(VK_SCROLL as u64);
    pub const NUMLOCK: Self = Self(VK_NUMLOCK as u64);
    pub const PRINTSCREEN: Self = Self(VK_SNAPSHOT as u64);
    pub const PAUSE: Self = Self(VK_PAUSE as u64);
    pub const NUMPAD_0: Self = Self(VK_NUMPAD0 as u64);
    pub const NUMPAD_1: Self = Self(VK_NUMPAD1 as u64);
    pub const NUMPAD_2: Self = Self(VK_NUMPAD2 as u64);
    pub const NUMPAD_3: Self = Self(VK_NUMPAD3 as u64);
    pub const NUMPAD_4: Self = Self(VK_NUMPAD4 as u64);
    pub const NUMPAD_5: Self = Self(VK_NUMPAD5 as u64);
    pub const NUMPAD_6: Self = Self(VK_NUMPAD6 as u64);
    pub const NUMPAD_7: Self = Self(VK_NUMPAD7 as u64);
    pub const NUMPAD_8: Self = Self(VK_NUMPAD8 as u64);
    pub const NUMPAD_9: Self = Self(VK_NUMPAD9 as u64);
    pub const KEYPAD_DECIMAL: Self = Self(VK_DECIMAL as u64);
    pub const KEYPAD_DIVIDE: Self = Self(VK_DIVIDE as u64);
    pub const KEYPAD_MULTIPLY: Self = Self(VK_MULTIPLY as u64);
    pub const KEYPAD_SUBTRACT: Self = Self(VK_SUBTRACT as u64);
    pub const KEYPAD_ADD: Self = Self(VK_ADD as u64);
    pub const KEYPAD_ENTER: Self = Self(VK_RETURN as u64);
    pub const LEFT_SHIFT: Self = Self(VK_LSHIFT as u64);
    pub const LEFT_CTRL: Self = Self(VK_LCONTROL as u64);
    pub const LEFT_ALT: Self = Self(VK_LMENU as u64);
    pub const LEFT_SUPER: Self = Self(VK_LWIN as u64);
    pub const RIGHT_SHIFT: Self = Self(VK_RSHIFT as u64);
    pub const RIGHT_CTRL: Self = Self(VK_RCONTROL as u64);
    pub const RIGHT_ALT: Self = Self(VK_RMENU as u64);
    pub const RIGHT_SUPER: Self = Self(VK_RWIN as u64);
    pub const MENU: Self = Self(VK_APPS as u64);
    pub const KEY_0: Self = Self(VK_0  as u64);
    pub const KEY_1: Self = Self(VK_1  as u64);
    pub const KEY_2: Self = Self(VK_2  as u64);
    pub const KEY_3: Self = Self(VK_3  as u64);
    pub const KEY_4: Self = Self(VK_4  as u64);
    pub const KEY_5: Self = Self(VK_5  as u64);
    pub const KEY_6: Self = Self(VK_6  as u64);
    pub const KEY_7: Self = Self(VK_7  as u64);
    pub const KEY_8: Self = Self(VK_8  as u64);
    pub const KEY_9: Self = Self(VK_9  as u64);
    pub const A: Self = Self(VK_A  as u64);
    pub const B: Self = Self(VK_B  as u64);
    pub const C: Self = Self(VK_C  as u64);
    pub const D: Self = Self(VK_D  as u64);
    pub const E: Self = Self(VK_E  as u64);
    pub const F: Self = Self(VK_F  as u64);
    pub const G: Self = Self(VK_G  as u64);
    pub const H: Self = Self(VK_H  as u64);
    pub const I: Self = Self(VK_I  as u64);
    pub const J: Self = Self(VK_J  as u64);
    pub const K: Self = Self(VK_K  as u64);
    pub const L: Self = Self(VK_L  as u64);
    pub const M: Self = Self(VK_M  as u64);
    pub const N: Self = Self(VK_N  as u64);
    pub const O: Self = Self(VK_O  as u64);
    pub const P: Self = Self(VK_P  as u64);
    pub const Q: Self = Self(VK_Q  as u64);
    pub const R: Self = Self(VK_R  as u64);
    pub const S: Self = Self(VK_S  as u64);
    pub const T: Self = Self(VK_T  as u64);
    pub const U: Self = Self(VK_U  as u64);
    pub const V: Self = Self(VK_V  as u64);
    pub const W: Self = Self(VK_W  as u64);
    pub const X: Self = Self(VK_X  as u64);
    pub const Y: Self = Self(VK_Y  as u64);
    pub const Z: Self = Self(VK_Z  as u64);
    pub const F1: Self = Self(VK_F1 as u64);
    pub const F2: Self = Self(VK_F2 as u64);
    pub const F3: Self = Self(VK_F3 as u64);
    pub const F4: Self = Self(VK_F4 as u64);
    pub const F5: Self = Self(VK_F5 as u64);
    pub const F6: Self = Self(VK_F6 as u64);
    pub const F7: Self = Self(VK_F7 as u64);
    pub const F8: Self = Self(VK_F8 as u64);
    pub const F9: Self = Self(VK_F9 as u64);
    pub const F10: Self = Self(VK_F10 as u64);
    pub const F11: Self = Self(VK_F11 as u64);
    pub const F12: Self = Self(VK_F12 as u64);
}

#[cfg(target_os = "linux")]
use x11::keysym::*;
#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Wraps inner os specific virtual key to make matching more easier.
/// ```no_run
/// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
/// let key = Key(VK_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
///
/// use ndk_sys::AKEYCODE_ESCAPE;
/// let key = Key(AKEYCODE_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
/// ```
pub struct Key(pub u64);

#[cfg(target_os = "linux")]
impl Key {
    pub const TAB: Self = Self(XK_Tab as u64);
    pub const LEFT_ARROW: Self = Self(XK_Left as u64);
    pub const RIGHT_ARROW: Self = Self(XK_Right as u64);
    pub const UP_ARROW: Self = Self(XK_Up as u64);
    pub const DOWN_ARROW: Self = Self(XK_Down as u64);
    pub const PAGE_UP: Self = Self(XK_Page_Up as u64);
    pub const PAGE_DOWN: Self = Self(XK_Page_Down as u64);
    pub const HOME: Self = Self(XK_Home as u64);
    pub const END: Self = Self(XK_End as u64);
    pub const INSERT: Self = Self(XK_Insert as u64);
    pub const DELETE: Self = Self(XK_Delete as u64);
    pub const BACKSPACE: Self = Self(XK_BackSpace as u64);
    pub const SPACE: Self = Self(XK_space as u64);
    pub const ENTER: Self = Self(XK_Return as u64);
    pub const ESCAPE: Self = Self(XK_Escape as u64);
    pub const APOSTROPHE: Self = Self(XK_apostrophe as u64);
    pub const COMMA: Self = Self(XK_comma as u64);
    pub const MINUS: Self = Self(XK_minus as u64);
    pub const PERIOD: Self = Self(XK_period as u64);
    pub const SLASH: Self = Self(XK_slash as u64);
    pub const SEMICOLON: Self = Self(XK_semicolon as u64);
    pub const EQUAL: Self = Self(XK_equal as u64);
    pub const LEFT_BRACKET: Self = Self(XK_bracketleft as u64);
    pub const BACKSLASH: Self = Self(XK_backslash as u64);
    pub const RIGHT_BRACKET: Self = Self(XK_bracketright as u64);
    pub const GRAVEACCENT: Self = Self(XK_grave as u64);
    pub const CAPSLOCK: Self = Self(XK_Caps_Lock as u64);
    pub const SCROLLLOCK: Self = Self(XK_Scroll_Lock as u64);
    pub const NUMLOCK: Self = Self(XK_Num_Lock as u64);
    pub const PRINTSCREEN: Self = Self(XK_Print as u64);
    pub const PAUSE: Self = Self(XK_Pause as u64);
    pub const NUMPAD_0: Self = Self(XK_KP_0 as u64);
    pub const NUMPAD_1: Self = Self(XK_KP_1 as u64);
    pub const NUMPAD_2: Self = Self(XK_KP_2 as u64);
    pub const NUMPAD_3: Self = Self(XK_KP_3 as u64);
    pub const NUMPAD_4: Self = Self(XK_KP_4 as u64);
    pub const NUMPAD_5: Self = Self(XK_KP_5 as u64);
    pub const NUMPAD_6: Self = Self(XK_KP_6 as u64);
    pub const NUMPAD_7: Self = Self(XK_KP_7 as u64);
    pub const NUMPAD_8: Self = Self(XK_KP_8 as u64);
    pub const NUMPAD_9: Self = Self(XK_KP_9 as u64);
    pub const KEYPAD_DECIMAL: Self = Self(XK_KP_Decimal as u64);
    pub const KEYPAD_DIVIDE: Self = Self(XK_KP_Divide as u64);
    pub const KEYPAD_MULTIPLY: Self = Self(XK_KP_Multiply as u64);
    pub const KEYPAD_SUBTRACT: Self = Self(XK_KP_Subtract as u64);
    pub const KEYPAD_ADD: Self = Self(XK_KP_Add as u64);
    pub const KEYPAD_ENTER: Self = Self(XK_KP_Enter as u64);
    pub const LEFT_SHIFT: Self = Self(XK_Shift_L as u64);
    pub const LEFT_CTRL: Self = Self(XK_Control_L as u64);
    pub const LEFT_ALT: Self = Self(XK_Alt_L as u64);
    pub const LEFT_SUPER: Self = Self(XK_Super_L as u64);
    pub const RIGHT_SHIFT: Self = Self(XK_Shift_R as u64);
    pub const RIGHT_CTRL: Self = Self(XK_Control_R as u64);
    pub const RIGHT_ALT: Self = Self(XK_Alt_R as u64);
    pub const RIGHT_SUPER: Self = Self(XK_Super_R as u64);
    pub const MENU: Self = Self(XK_Menu as u64);
    pub const KEY_0: Self = Self(XK_0 as u64);
    pub const KEY_1: Self = Self(XK_1 as u64);
    pub const KEY_2: Self = Self(XK_2 as u64);
    pub const KEY_3: Self = Self(XK_3 as u64);
    pub const KEY_4: Self = Self(XK_4 as u64);
    pub const KEY_5: Self = Self(XK_5 as u64);
    pub const KEY_6: Self = Self(XK_6 as u64);
    pub const KEY_7: Self = Self(XK_7 as u64);
    pub const KEY_8: Self = Self(XK_8 as u64);
    pub const KEY_9: Self = Self(XK_9 as u64);
    pub const A: Self = Self(XK_a as u64);
    pub const B: Self = Self(XK_b as u64);
    pub const C: Self = Self(XK_c as u64);
    pub const D: Self = Self(XK_d as u64);
    pub const E: Self = Self(XK_e as u64);
    pub const F: Self = Self(XK_f as u64);
    pub const G: Self = Self(XK_g as u64);
    pub const H: Self = Self(XK_h as u64);
    pub const I: Self = Self(XK_i as u64);
    pub const J: Self = Self(XK_j as u64);
    pub const K: Self = Self(XK_k as u64);
    pub const L: Self = Self(XK_l as u64);
    pub const M: Self = Self(XK_m as u64);
    pub const N: Self = Self(XK_n as u64);
    pub const O: Self = Self(XK_o as u64);
    pub const P: Self = Self(XK_p as u64);
    pub const Q: Self = Self(XK_q as u64);
    pub const R: Self = Self(XK_r as u64);
    pub const S: Self = Self(XK_s as u64);
    pub const T: Self = Self(XK_t as u64);
    pub const U: Self = Self(XK_u as u64);
    pub const V: Self = Self(XK_v as u64);
    pub const W: Self = Self(XK_w as u64);
    pub const X: Self = Self(XK_x as u64);
    pub const Y: Self = Self(XK_y as u64);
    pub const Z: Self = Self(XK_z as u64);
    pub const F1: Self = Self(XK_F1 as u64);
    pub const F2: Self = Self(XK_F2 as u64);
    pub const F3: Self = Self(XK_F3 as u64);
    pub const F4: Self = Self(XK_F4 as u64);
    pub const F5: Self = Self(XK_F5 as u64);
    pub const F6: Self = Self(XK_F6 as u64);
    pub const F7: Self = Self(XK_F7 as u64);
    pub const F8: Self = Self(XK_F8 as u64);
    pub const F9: Self = Self(XK_F9 as u64);
    pub const F10: Self = Self(XK_F10 as u64);
    pub const F11: Self = Self(XK_F11 as u64);
    pub const F12: Self = Self(XK_F12 as u64);
}

#[cfg(target_os = "android")]
use ndk_sys::*;
#[cfg(target_os = "android")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Wraps inner os specific virtual key to make matching more easier.
/// ```no_run
/// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
/// let key = Key(VK_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
///
/// use ndk_sys::AKEYCODE_ESCAPE;
/// let key = Key(AKEYCODE_ESCAPE as u64);
/// assert!(key == KEY::ESCAPE);
/// ```
pub struct Key(pub u64);

#[cfg(target_os = "android")]
impl Key {
    pub const TAB: Self = Self(AKEYCODE_TAB as u64);
    pub const LEFT_ARROW: Self = Self(AKEYCODE_DPAD_LEFT as u64);
    pub const RIGHT_ARROW: Self = Self(AKEYCODE_DPAD_RIGHT as u64);
    pub const UP_ARROW: Self = Self(AKEYCODE_DPAD_UP as u64);
    pub const DOWN_ARROW: Self = Self(AKEYCODE_DPAD_DOWN as u64);
    pub const PAGE_UP: Self = Self(AKEYCODE_PAGE_UP as u64);
    pub const PAGE_DOWN: Self = Self(AKEYCODE_PAGE_DOWN as u64);
    pub const HOME: Self = Self(AKEYCODE_HOME as u64);
    pub const END: Self = Self(AKEYCODE_MOVE_END as u64);
    pub const INSERT: Self = Self(AKEYCODE_INSERT as u64);
    pub const DELETE: Self = Self(AKEYCODE_DEL as u64);
    pub const BACKSPACE: Self = Self(AKEYCODE_DEL as u64);
    pub const SPACE: Self = Self(AKEYCODE_SPACE as u64);
    pub const ENTER: Self = Self(AKEYCODE_ENTER as u64);
    pub const ESCAPE: Self = Self(AKEYCODE_ESCAPE as u64);
    pub const APOSTROPHE: Self = Self(AKEYCODE_APOSTROPHE as u64);
    pub const COMMA: Self = Self(AKEYCODE_COMMA as u64);
    pub const MINUS: Self = Self(AKEYCODE_MINUS as u64);
    pub const PERIOD: Self = Self(AKEYCODE_PERIOD as u64);
    pub const SLASH: Self = Self(AKEYCODE_SLASH as u64);
    pub const SEMICOLON: Self = Self(AKEYCODE_SEMICOLON as u64);
    pub const EQUAL: Self = Self(AKEYCODE_EQUALS as u64);
    pub const LEFT_BRACKET: Self = Self(AKEYCODE_LEFT_BRACKET as u64);
    pub const BACKSLASH: Self = Self(AKEYCODE_BACKSLASH as u64);
    pub const RIGHT_BRACKET: Self = Self(AKEYCODE_RIGHT_BRACKET as u64);
    pub const GRAVEACCENT: Self = Self(AKEYCODE_GRAVE as u64);
    pub const CAPSLOCK: Self = Self(AKEYCODE_CAPS_LOCK as u64);
    pub const SCROLLLOCK: Self = Self(AKEYCODE_SCROLL_LOCK as u64);
    pub const NUMLOCK: Self = Self(AKEYCODE_NUM_LOCK as u64);
    pub const PRINTSCREEN: Self = Self(AKEYCODE_SYSRQ as u64);
    pub const PAUSE: Self = Self(AKEYCODE_MEDIA_PLAY_PAUSE as u64);
    pub const NUMPAD_0: Self = Self(AKEYCODE_NUMPAD_0 as u64);
    pub const NUMPAD_1: Self = Self(AKEYCODE_NUMPAD_1 as u64);
    pub const NUMPAD_2: Self = Self(AKEYCODE_NUMPAD_2 as u64);
    pub const NUMPAD_3: Self = Self(AKEYCODE_NUMPAD_3 as u64);
    pub const NUMPAD_4: Self = Self(AKEYCODE_NUMPAD_4 as u64);
    pub const NUMPAD_5: Self = Self(AKEYCODE_NUMPAD_5 as u64);
    pub const NUMPAD_6: Self = Self(AKEYCODE_NUMPAD_6 as u64);
    pub const NUMPAD_7: Self = Self(AKEYCODE_NUMPAD_7 as u64);
    pub const NUMPAD_8: Self = Self(AKEYCODE_NUMPAD_8 as u64);
    pub const NUMPAD_9: Self = Self(AKEYCODE_NUMPAD_9 as u64);
    pub const KEYPAD_DECIMAL: Self = Self(AKEYCODE_NUMPAD_COMMA as u64);
    pub const KEYPAD_DIVIDE: Self = Self(AKEYCODE_NUMPAD_DIVIDE as u64);
    pub const KEYPAD_MULTIPLY: Self = Self(AKEYCODE_NUMPAD_MULTIPLY as u64);
    pub const KEYPAD_SUBTRACT: Self = Self(AKEYCODE_NUMPAD_SUBTRACT as u64);
    pub const KEYPAD_ADD: Self = Self(AKEYCODE_NUMPAD_ADD as u64);
    pub const KEYPAD_ENTER: Self = Self(AKEYCODE_ENTER as u64);
    pub const LEFT_SHIFT: Self = Self(AKEYCODE_SHIFT_LEFT as u64);
    pub const LEFT_CTRL: Self = Self(AKEYCODE_CTRL_LEFT as u64);
    pub const LEFT_ALT: Self = Self(AKEYCODE_ALT_LEFT as u64);
    pub const LEFT_SUPER: Self = Self(AKEYCODE_META_LEFT as u64);
    pub const RIGHT_SHIFT: Self = Self(AKEYCODE_SHIFT_RIGHT as u64);
    pub const RIGHT_CTRL: Self = Self(AKEYCODE_CTRL_RIGHT as u64);
    pub const RIGHT_ALT: Self = Self(AKEYCODE_ALT_RIGHT as u64);
    pub const RIGHT_SUPER: Self = Self(AKEYCODE_META_RIGHT as u64);
    pub const MENU: Self = Self(AKEYCODE_MENU as u64);
    pub const KEY_0: Self = Self(AKEYCODE_0 as u64);
    pub const KEY_1: Self = Self(AKEYCODE_1 as u64);
    pub const KEY_2: Self = Self(AKEYCODE_2 as u64);
    pub const KEY_3: Self = Self(AKEYCODE_3 as u64);
    pub const KEY_4: Self = Self(AKEYCODE_4 as u64);
    pub const KEY_5: Self = Self(AKEYCODE_5 as u64);
    pub const KEY_6: Self = Self(AKEYCODE_6 as u64);
    pub const KEY_7: Self = Self(AKEYCODE_7 as u64);
    pub const KEY_8: Self = Self(AKEYCODE_8 as u64);
    pub const KEY_9: Self = Self(AKEYCODE_9 as u64);
    pub const A: Self = Self(AKEYCODE_A as u64);
    pub const B: Self = Self(AKEYCODE_B as u64);
    pub const C: Self = Self(AKEYCODE_C as u64);
    pub const D: Self = Self(AKEYCODE_D as u64);
    pub const E: Self = Self(AKEYCODE_E as u64);
    pub const F: Self = Self(AKEYCODE_F as u64);
    pub const G: Self = Self(AKEYCODE_G as u64);
    pub const H: Self = Self(AKEYCODE_H as u64);
    pub const I: Self = Self(AKEYCODE_I as u64);
    pub const J: Self = Self(AKEYCODE_J as u64);
    pub const K: Self = Self(AKEYCODE_K as u64);
    pub const L: Self = Self(AKEYCODE_L as u64);
    pub const M: Self = Self(AKEYCODE_M as u64);
    pub const N: Self = Self(AKEYCODE_N as u64);
    pub const O: Self = Self(AKEYCODE_O as u64);
    pub const P: Self = Self(AKEYCODE_P as u64);
    pub const Q: Self = Self(AKEYCODE_Q as u64);
    pub const R: Self = Self(AKEYCODE_R as u64);
    pub const S: Self = Self(AKEYCODE_S as u64);
    pub const T: Self = Self(AKEYCODE_T as u64);
    pub const U: Self = Self(AKEYCODE_U as u64);
    pub const V: Self = Self(AKEYCODE_V as u64);
    pub const W: Self = Self(AKEYCODE_W as u64);
    pub const X: Self = Self(AKEYCODE_X as u64);
    pub const Y: Self = Self(AKEYCODE_Y as u64);
    pub const Z: Self = Self(AKEYCODE_Z as u64);
    pub const F1: Self = Self(AKEYCODE_F1 as u64);
    pub const F2: Self = Self(AKEYCODE_F2 as u64);
    pub const F3: Self = Self(AKEYCODE_F3 as u64);
    pub const F4: Self = Self(AKEYCODE_F4 as u64);
    pub const F5: Self = Self(AKEYCODE_F5 as u64);
    pub const F6: Self = Self(AKEYCODE_F6 as u64);
    pub const F7: Self = Self(AKEYCODE_F7 as u64);
    pub const F8: Self = Self(AKEYCODE_F8 as u64);
    pub const F9: Self = Self(AKEYCODE_F9 as u64);
    pub const F10: Self = Self(AKEYCODE_F10 as u64);
    pub const F11: Self = Self(AKEYCODE_F11 as u64);
    pub const F12: Self = Self(AKEYCODE_F12 as u64);
}

impl Debug for Key {
    #[allow(unreachable_patterns)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Key::TAB => f.write_str("Key::TAB"),
            Key::LEFT_ARROW => f.write_str("Key::LEFT_ARROW"),
            Key::RIGHT_ARROW => f.write_str("Key::RIGHT_ARROW"),
            Key::UP_ARROW => f.write_str("Key::UP_ARROW"),
            Key::DOWN_ARROW => f.write_str("Key::DOWN_ARROW"),
            Key::PAGE_UP => f.write_str("Key::PAGE_UP"),
            Key::PAGE_DOWN => f.write_str("Key::PAGE_DOWN"),
            Key::HOME => f.write_str("Key::HOME"),
            Key::END => f.write_str("Key::END"),
            Key::INSERT => f.write_str("Key::INSERT"),
            Key::DELETE => f.write_str("Key::DELETE"),
            Key::BACKSPACE => f.write_str("Key::BACKSPACE"),
            Key::SPACE => f.write_str("Key::SPACE"),
            Key::ENTER => f.write_str("Key::ENTER"),
            Key::ESCAPE => f.write_str("Key::ESCAPE"),
            Key::APOSTROPHE => f.write_str("Key::APOSTROPHE"),
            Key::COMMA => f.write_str("Key::COMMA"),
            Key::MINUS => f.write_str("Key::MINUS"),
            Key::PERIOD => f.write_str("Key::PERIOD"),
            Key::SLASH => f.write_str("Key::SLASH"),
            Key::SEMICOLON => f.write_str("Key::SEMICOLON"),
            Key::EQUAL => f.write_str("Key::EQUAL"),
            Key::LEFT_BRACKET => f.write_str("Key::LEFT_BRACKET"),
            Key::BACKSLASH => f.write_str("Key::BACKSLASH"),
            Key::RIGHT_BRACKET => f.write_str("Key::RIGHT_BRACKET"),
            Key::GRAVEACCENT => f.write_str("Key::GRAVEACCENT"),
            Key::CAPSLOCK => f.write_str("Key::CAPSLOCK"),
            Key::SCROLLLOCK => f.write_str("Key::SCROLLLOCK"),
            Key::NUMLOCK => f.write_str("Key::NUMLOCK"),
            Key::PRINTSCREEN => f.write_str("Key::PRINTSCREEN"),
            Key::PAUSE => f.write_str("Key::PAUSE"),
            Key::NUMPAD_0 => f.write_str("Key::NUMPAD_0"),
            Key::NUMPAD_1 => f.write_str("Key::NUMPAD_1"),
            Key::NUMPAD_2 => f.write_str("Key::NUMPAD_2"),
            Key::NUMPAD_3 => f.write_str("Key::NUMPAD_3"),
            Key::NUMPAD_4 => f.write_str("Key::NUMPAD_4"),
            Key::NUMPAD_5 => f.write_str("Key::NUMPAD_5"),
            Key::NUMPAD_6 => f.write_str("Key::NUMPAD_6"),
            Key::NUMPAD_7 => f.write_str("Key::NUMPAD_7"),
            Key::NUMPAD_8 => f.write_str("Key::NUMPAD_8"),
            Key::NUMPAD_9 => f.write_str("Key::NUMPAD_9"),
            Key::KEYPAD_DECIMAL => f.write_str("Key::KEYPAD_DECIMAL"),
            Key::KEYPAD_DIVIDE => f.write_str("Key::KEYPAD_DIVIDE"),
            Key::KEYPAD_MULTIPLY => f.write_str("Key::KEYPAD_MULTIPLY"),
            Key::KEYPAD_SUBTRACT => f.write_str("Key::KEYPAD_SUBTRACT"),
            Key::KEYPAD_ADD => f.write_str("Key::KEYPAD_ADD"),
            Key::KEYPAD_ENTER => f.write_str("Key::KEYPAD_ENTER"),
            Key::LEFT_SHIFT => f.write_str("Key::LEFT_SHIFT"),
            Key::LEFT_CTRL => f.write_str("Key::LEFT_CTRL"),
            Key::LEFT_ALT => f.write_str("Key::LEFT_ALT"),
            Key::LEFT_SUPER => f.write_str("Key::LEFT_SUPER"),
            Key::RIGHT_SHIFT => f.write_str("Key::RIGHT_SHIFT"),
            Key::RIGHT_CTRL => f.write_str("Key::RIGHT_CTRL"),
            Key::RIGHT_ALT => f.write_str("Key::RIGHT_ALT"),
            Key::RIGHT_SUPER => f.write_str("Key::RIGHT_SUPER"),
            Key::MENU => f.write_str("Key::MENU"),
            Key::KEY_0 => f.write_str("Key::KEY_0"),
            Key::KEY_1 => f.write_str("Key::KEY_1"),
            Key::KEY_2 => f.write_str("Key::KEY_2"),
            Key::KEY_3 => f.write_str("Key::KEY_3"),
            Key::KEY_4 => f.write_str("Key::KEY_4"),
            Key::KEY_5 => f.write_str("Key::KEY_5"),
            Key::KEY_6 => f.write_str("Key::KEY_6"),
            Key::KEY_7 => f.write_str("Key::KEY_7"),
            Key::KEY_8 => f.write_str("Key::KEY_8"),
            Key::KEY_9 => f.write_str("Key::KEY_9"),
            Key::A => f.write_str("Key::A"),
            Key::B => f.write_str("Key::B"),
            Key::C => f.write_str("Key::C"),
            Key::D => f.write_str("Key::D"),
            Key::E => f.write_str("Key::E"),
            Key::F => f.write_str("Key::F"),
            Key::G => f.write_str("Key::G"),
            Key::H => f.write_str("Key::H"),
            Key::I => f.write_str("Key::I"),
            Key::J => f.write_str("Key::J"),
            Key::K => f.write_str("Key::K"),
            Key::L => f.write_str("Key::L"),
            Key::M => f.write_str("Key::M"),
            Key::N => f.write_str("Key::N"),
            Key::O => f.write_str("Key::O"),
            Key::P => f.write_str("Key::P"),
            Key::Q => f.write_str("Key::Q"),
            Key::R => f.write_str("Key::R"),
            Key::S => f.write_str("Key::S"),
            Key::T => f.write_str("Key::T"),
            Key::U => f.write_str("Key::U"),
            Key::V => f.write_str("Key::V"),
            Key::W => f.write_str("Key::W"),
            Key::X => f.write_str("Key::X"),
            Key::Y => f.write_str("Key::Y"),
            Key::Z => f.write_str("Key::Z"),
            Key::F1 => f.write_str("Key::F1"),
            Key::F2 => f.write_str("Key::F2"),
            Key::F3 => f.write_str("Key::F3"),
            Key::F4 => f.write_str("Key::F4"),
            Key::F5 => f.write_str("Key::F5"),
            Key::F6 => f.write_str("Key::F6"),
            Key::F7 => f.write_str("Key::F7"),
            Key::F8 => f.write_str("Key::F8"),
            Key::F9 => f.write_str("Key::F9"),
            Key::F10 => f.write_str("Key::F10"),
            Key::F11 => f.write_str("Key::F11"),
            Key::F12 => f.write_str("Key::F12"),
            any => write!(f, "Key({})", any.0),
        }
    }
}
