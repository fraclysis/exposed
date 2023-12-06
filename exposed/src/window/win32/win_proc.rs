use std::ptr::null_mut;

use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{MapVirtualKeyW, MAPVK_VSC_TO_VK_EX, VK_CONTROL, VK_MENU, VK_SHIFT},
        Shell::{DragFinish, DragQueryFileW, HDROP},
    },
};

use crate::window::{win32::ThreadContext, Event, Key, MouseButton};

use super::WindowHandle;

#[inline(never)]
pub unsafe extern "system" fn win_proc<E: Event>(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    use windows_sys::Win32::{
        Devices::HumanInterfaceDevice::MOUSE_MOVE_RELATIVE,
        UI::{
            Input::{GetRawInputData, RAWINPUT, RAWINPUTHEADER, RID_INPUT, RIM_TYPEMOUSE},
            WindowsAndMessaging::*,
        },
    };

    let handler = ThreadContext::user_data::<E>();
    if handler.is_null() {
        return E::missed_events(hwnd, msg, wparam, lparam);
    }

    let handler = &mut *handler;

    match msg {
        WM_ERASEBKGND => 1,

        WM_PAINT => {
            handler.low_render(WindowHandle(hwnd).into());
            0
        }

        WM_INPUT => {
            fn _get_rawinput_code_wparam(wparam: WPARAM) -> u32 {
                wparam as u32 & 0xff
            }

            let mut data: RAWINPUT = unsafe { std::mem::zeroed() };
            let mut data_size = std::mem::size_of::<RAWINPUT>() as u32;
            let header_size = std::mem::size_of::<RAWINPUTHEADER>() as u32;

            let status = unsafe { GetRawInputData(lparam, RID_INPUT, &mut data as *mut _ as _, &mut data_size, header_size) };

            if status == u32::MAX || status == 0 {
                return DefWindowProcW(hwnd, msg, wparam, lparam);
            }

            match data.header.dwType {
                RIM_TYPEMOUSE => {
                    let mouse = data.data.mouse;

                    // TODO check #[deny(clippy::bad_bit_mask)]
                    #[allow(clippy::bad_bit_mask)] // Win32 code
                    if (mouse.usFlags as u32 & MOUSE_MOVE_RELATIVE) == MOUSE_MOVE_RELATIVE {
                        let x = mouse.lLastX as i32;
                        let y = mouse.lLastY as i32;

                        handler.raw_mouse_motion(x, y);
                    }
                }
                // RIM_TYPEKEYBOARD => (),
                // RIM_TYPEHID => (),
                _ => eprintln!("Undefined Raw input"),
            }

            DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        WM_CLOSE => {
            handler.close_requested(WindowHandle(hwnd).into());
            0
        }

        WM_DESTROY => {
            handler.destroyed(WindowHandle(hwnd).into());

            0
        }

        WM_KEYDOWN | WM_KEYUP => {
            let mut vk_code = loword(wparam as u32);
            let key_flag = hiword(lparam as u32);
            let mut scan_code: u16 = lobyte(key_flag as u32) as u16;

            let is_extended_key = (key_flag as u32 & KF_EXTENDED) == KF_EXTENDED;

            if is_extended_key {
                scan_code = make_word(scan_code as _, 0xE0);
            }

            // let was_key_down = (key_flag as u32 & KF_REPEAT) == KF_REPEAT;
            // let repeat_count = loword(lparam as u32);
            // let is_key_released = (key_flag as u32 & KF_UP) == KF_UP;

            match vk_code {
                VK_SHIFT | VK_CONTROL | VK_MENU => {
                    vk_code = loword(MapVirtualKeyW(scan_code as u32, MAPVK_VSC_TO_VK_EX) as u32);
                }
                _ => (),
            }

            match msg {
                WM_KEYDOWN => {
                    handler.key_down(WindowHandle(hwnd).into(), Key(vk_code as _), scan_code as _);
                    0
                }
                WM_KEYUP => {
                    handler.key_up(WindowHandle(hwnd).into(), Key(vk_code as _), scan_code as _);
                    0
                }
                WM_SYSKEYDOWN => {
                    handler.key_down(WindowHandle(hwnd).into(), Key(vk_code as _), scan_code as _);
                    DefWindowProcW(hwnd, msg, wparam, lparam)
                }
                WM_SYSKEYUP => {
                    handler.key_up(WindowHandle(hwnd).into(), Key(vk_code as _), scan_code as _);
                    DefWindowProcW(hwnd, msg, wparam, lparam)
                }
                _ => std::hint::unreachable_unchecked(),
            }
        }

        WM_MOUSEMOVE => {
            let x = get_x_lparam(lparam as u32) as i32;
            let y = get_y_lparam(lparam as u32) as i32;

            handler.cursor_moved(WindowHandle(hwnd).into(), x, y);

            0
        }

        WM_MOUSEWHEEL => {
            let value = (wparam >> 16) as i16;
            let value = value as i32;
            let value = value as f32 / WHEEL_DELTA as f32;

            handler.mouse_wheel(WindowHandle(hwnd).into(), 0.0, value);

            0
        }

        WM_MOUSEHWHEEL => {
            let value = (wparam >> 16) as i16;
            let value = value as i32;
            let value = -value as f32 / WHEEL_DELTA as f32;

            handler.mouse_wheel(WindowHandle(hwnd).into(), value, 0.0);

            0
        }

        WM_SETFOCUS => {
            handler.focused(WindowHandle(hwnd).into(), true);
            0
        }

        WM_KILLFOCUS => {
            handler.focused(WindowHandle(hwnd).into(), false);
            0
        }

        WM_SIZE => {
            let w = loword(lparam as u32) as i32;
            let h = hiword(lparam as u32) as i32;

            handler.resized(WindowHandle(hwnd).into(), w, h);
            0
        }

        WM_SETCURSOR => DefWindowProcW(hwnd, msg, wparam, lparam),

        WM_LBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle(hwnd).into(), MouseButton::LEFT);
            0
        }

        WM_LBUTTONUP => {
            handler.mouse_button_release(WindowHandle(hwnd).into(), MouseButton::LEFT);
            0
        }

        WM_RBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle(hwnd).into(), MouseButton::RIGHT);
            0
        }

        WM_RBUTTONUP => {
            handler.mouse_button_release(WindowHandle(hwnd).into(), MouseButton::RIGHT);
            0
        }

        WM_MBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle(hwnd).into(), MouseButton::MIDDLE);
            0
        }

        WM_MBUTTONUP => {
            handler.mouse_button_release(WindowHandle(hwnd).into(), MouseButton::MIDDLE);
            0
        }

        WM_XBUTTONDOWN => {
            let b = hiword(wparam as _);
            handler.mouse_button_down(WindowHandle(hwnd).into(), MouseButton(10 + b as u32));
            0
        }

        WM_XBUTTONUP => {
            let b = hiword(wparam as _);
            handler.mouse_button_release(WindowHandle(hwnd).into(), MouseButton(10 + b as u32));
            0
        }

        WM_CHAR | WM_SYSCHAR => {
            let char: u16 = wparam as u16;

            #[inline]
            fn is_high_surrogate(char: u16) -> bool {
                (0xD800..=0xDBFF).contains(&char)
            }

            #[inline]
            fn is_low_surrogate(char: u16) -> bool {
                (0xDC00..=0xDFFF).contains(&char)
            }

            #[inline]
            fn send_char_event<E: Event, I>(handler: &mut E, hwnd: HWND, iter: I)
            where
                I: IntoIterator<Item = u16>,
            {
                for c in std::char::decode_utf16(iter) {
                    if let Ok(c) = c {
                        handler.received_character(WindowHandle(hwnd).into(), c);
                    } else {
                        eprintln!("Decode error");
                    }
                }
            }

            if is_high_surrogate(char) {
                ThreadContext::get_ref().last_char = wparam as u16;
            } else if is_low_surrogate(char) {
                let context = ThreadContext::get_ref();
                let chars = [context.last_char, char];
                send_char_event(handler, hwnd, chars);
                context.last_char = 0;
            } else {
                send_char_event(handler, hwnd, [char]);
            }

            if msg == WM_SYSCHAR {
                DefWindowProcW(hwnd, msg, wparam, lparam)
            } else {
                0
            }
        }

        WM_WINDOWPOSCHANGED => {
            let windowpos = lparam as *const WINDOWPOS;

            if (*windowpos).flags & SWP_NOMOVE != SWP_NOMOVE {
                handler.moved(WindowHandle(hwnd).into(), (*windowpos).x, (*windowpos).y);
            }

            DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        WM_DROPFILES => {
            const GET_FILE_COUNT_FLAG: u32 = !0;
            let hdrop: HDROP = wparam as _;
            let count = DragQueryFileW(hdrop, GET_FILE_COUNT_FLAG, null_mut(), 0);

            for i in 0..count {
                const MAX_BUFFER_SIZE: usize = 255;

                let file_name_size = DragQueryFileW(hdrop, i, null_mut(), 0) + 1;

                if file_name_size > MAX_BUFFER_SIZE as u32 {
                } else {
                    let mut buffer = [0u16; MAX_BUFFER_SIZE];

                    if DragQueryFileW(hdrop, i, buffer.as_mut_ptr(), file_name_size) == 0 {
                        todo!("Error handling")
                    }

                    let file_name = String::from_utf16_lossy(&buffer[..file_name_size as usize - 1]);

                    handler.file_received(WindowHandle(hwnd).into(), file_name)
                }
            }

            DragFinish(hdrop);
            0
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[inline(always)]
const fn get_x_lparam(x: u32) -> i16 {
    loword(x) as _
}

#[inline(always)]
const fn get_y_lparam(x: u32) -> i16 {
    hiword(x) as _
}

#[inline(always)]
const fn loword(x: u32) -> u16 {
    (x & 0xFFFF) as u16
}

#[inline(always)]
const fn hiword(x: u32) -> u16 {
    ((x >> 16) & 0xFFFF) as u16
}

#[inline(always)]
const fn lobyte(x: u32) -> u8 {
    (x & 0xff) as u8
}

#[inline(always)]
const fn make_word(a: u8, b: u8) -> u16 {
    let a: u16 = (a & 0xff) as u16;
    let b: u16 = ((b & 0xff) as u16) << 8;

    a | b
}
