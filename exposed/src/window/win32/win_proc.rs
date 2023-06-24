use std::ptr::null_mut;

use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::Shell::{DragFinish, DragQueryFileW, HDROP},
};

use crate::window::{
    extern_ffi::{MessageLevel, MESSAGE_PROC},
    Event, EventHandler, MouseButton, WindowHandle,
};

#[inline(never)]
pub unsafe extern "system" fn win_proc<E: Event>(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    use std::mem::transmute;

    use windows_sys::Win32::{
        Devices::HumanInterfaceDevice::MOUSE_MOVE_RELATIVE,
        UI::{
            Input::{GetRawInputData, RAWINPUT, RAWINPUTHEADER, RID_INPUT, RIM_TYPEMOUSE},
            WindowsAndMessaging::*,
        },
    };

    if hwnd == 0 {
        if let Some(pfn) = MESSAGE_PROC {
            let message = format!("Hwnd 0 message! {}", msg);
            pfn(
                message.as_ptr().cast(),
                message.len() as i32,
                MessageLevel::Warn,
            );
        }
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    let handler_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut EventHandler<E>;
    if handler_ptr.is_null() {
        if let Some(pfn) = MESSAGE_PROC {
            let message = format!("Missed event {msg}. Cause GWLP_USERDATA is not set yet.");
            pfn(
                message.as_ptr().cast(),
                message.len() as i32,
                MessageLevel::Warn,
            );
        }
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    let event_handler = &mut *handler_ptr;

    if !event_handler.isUserDataValid {
        if let Some(pfn) = MESSAGE_PROC {
            let message = format!("Missed event {msg}. Cause isUserDataValid is false.");
            pfn(
                message.as_ptr().cast(),
                message.len() as i32,
                MessageLevel::Warn,
            );
        }
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    event_handler.last_hwnd = hwnd;
    event_handler.last_msg = msg;
    event_handler.last_wparam = wparam;
    event_handler.last_lparam = lparam;

    let handler = &mut *event_handler.userData;

    match msg {
        WM_ERASEBKGND => {
            // handler.low_render(WindowHandle{ windowHandle: hwnd});
            1
        }

        WM_PAINT => {
            handler.low_render(WindowHandle { windowHandle: hwnd });
            0
        }

        WM_INPUT => {
            fn _get_rawinput_code_wparam(wparam: WPARAM) -> u32 {
                wparam as u32 & 0xff
            }

            let mut data: RAWINPUT = unsafe { std::mem::zeroed() };
            let mut data_size = std::mem::size_of::<RAWINPUT>() as u32;
            let header_size = std::mem::size_of::<RAWINPUTHEADER>() as u32;

            let status = unsafe {
                GetRawInputData(
                    lparam,
                    RID_INPUT,
                    &mut data as *mut _ as _,
                    &mut data_size,
                    header_size,
                )
            };

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
            handler.close_requested(WindowHandle { windowHandle: hwnd });
            0
        }

        WM_DESTROY => {
            handler.destroyed(WindowHandle { windowHandle: hwnd });

            0
        }

        WM_KEYDOWN => {
            handler.key_down(
                WindowHandle { windowHandle: hwnd },
                std::mem::transmute(wparam as u32),
                0,
            );
            0
        }

        WM_KEYUP => {
            handler.key_up(
                WindowHandle { windowHandle: hwnd },
                std::mem::transmute(wparam as u32),
                0,
            );
            0
        }

        WM_MOUSEMOVE => {
            let x = get_x_lparam(lparam as u32) as i32;
            let y = get_y_lparam(lparam as u32) as i32;

            handler.cursor_moved(WindowHandle { windowHandle: hwnd }, x, y);

            0
        }

        WM_NCMOUSEMOVE => {
            let x = get_x_lparam(lparam as u32) as i32;
            let y = get_y_lparam(lparam as u32) as i32;

            handler.cursor_moved(WindowHandle { windowHandle: hwnd }, x, y);

            DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        WM_MOUSEWHEEL => {
            let value = (wparam >> 16) as i16;
            let value = value as i32;
            let value = value as f32 / WHEEL_DELTA as f32;

            handler.mouse_wheel(WindowHandle { windowHandle: hwnd }, 0.0, value);

            0
        }

        WM_MOUSEHWHEEL => {
            let value = (wparam >> 16) as i16;
            let value = value as i32;
            let value = -value as f32 / WHEEL_DELTA as f32; // NOTE: inverted! See https://github.com/rust-windowing/winit/pull/2105/

            handler.mouse_wheel(WindowHandle { windowHandle: hwnd }, value, 0.0);

            0
        }

        WM_SETFOCUS => {
            handler.focused(WindowHandle { windowHandle: hwnd }, true);
            0
        }

        WM_KILLFOCUS => {
            handler.focused(WindowHandle { windowHandle: hwnd }, false);
            0
        }

        WM_SIZE => {
            let w = loword(lparam as u32) as i32;
            let h = hiword(lparam as u32) as i32;

            handler.resized(WindowHandle { windowHandle: hwnd }, w, h);
            0
        }

        WM_SETCURSOR => {
            // if USE_DEFAULT_CURSOR {
            // DefWindowProcW(hwnd, msg, wparam, lparam)
            // } else {
            // SetCursor(LoadCursorW(0, IDC_ARROW))
            DefWindowProcW(hwnd, msg, wparam, lparam)
            // }
        }

        WM_LBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle { windowHandle: hwnd }, MouseButton(1));
            0
        }

        WM_LBUTTONUP => {
            handler.mouse_button_release(WindowHandle { windowHandle: hwnd }, MouseButton(1));
            0
        }

        WM_RBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle { windowHandle: hwnd }, MouseButton(2));
            0
        }

        WM_RBUTTONUP => {
            handler.mouse_button_release(WindowHandle { windowHandle: hwnd }, MouseButton(2));
            0
        }

        WM_MBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle { windowHandle: hwnd }, MouseButton(3));
            0
        }

        WM_MBUTTONUP => {
            handler.mouse_button_release(WindowHandle { windowHandle: hwnd }, MouseButton(3));
            0
        }

        WM_XBUTTONDOWN => {
            handler.mouse_button_down(WindowHandle { windowHandle: hwnd }, MouseButton(4));
            0
        }

        WM_XBUTTONUP => {
            handler.mouse_button_release(WindowHandle { windowHandle: hwnd }, MouseButton(5));
            0
        }

        WM_CHAR | WM_SYSCHAR => {
            #[cfg(target_pointer_width = "64")]
            const POINTER_U16_CAP: usize = 4;
            #[cfg(target_pointer_width = "32")]
            const POINTER_U16_CAP: usize = 2;
            let chars: [u16; POINTER_U16_CAP] = transmute(wparam);

            let chars = std::char::decode_utf16(chars);

            let is_high_surrogate = (0xD800..=0xDBFF).contains(&wparam);
            let is_low_surrogate = (0xDC00..=0xDFFF).contains(&wparam);

            // TODO research if `char::from_u32_unchecked` causes problem

            if is_high_surrogate {
            } else if is_low_surrogate {
            } else {
                handler.received_character(
                    WindowHandle { windowHandle: hwnd },
                    std::char::from_u32_unchecked(wparam as u32),
                )
            }

            for c in chars {
                match c {
                    Ok(char) => {
                        if char != '\0' {
                            handler.received_character(WindowHandle { windowHandle: hwnd }, char);
                        }
                    }
                    Err(e) => eprintln!("CharErr: {e}"),
                }
            }

            if msg == WM_SYSCHAR {
                DefWindowProcW(hwnd, msg, wparam, lparam)
            } else {
                0
            }
        }

        // WM_WINDOWPOSCHANGING => {
        //     // TODO: Fix
        //     let windowpos = lparam as *const WINDOWPOS;

        //     if (*windowpos).flags & SWP_NOMOVE != SWP_NOMOVE {
        //         handler.moved(hwnd, (*windowpos).x, (*windowpos).y);
        //     }

        //     0
        // }
        WM_WINDOWPOSCHANGED => {
            // TODO: Fix
            let windowpos = lparam as *const WINDOWPOS;

            if (*windowpos).flags & SWP_NOMOVE != SWP_NOMOVE {
                handler.moved(
                    WindowHandle { windowHandle: hwnd },
                    (*windowpos).x,
                    (*windowpos).y,
                );
            }

            // This is necessary for us to still get sent WM_SIZE.
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        WM_DROPFILES => {
            const GET_FILE_COUNT_FLAG: u32 = !0;
            let hdrop: HDROP = wparam as _;
            let count = DragQueryFileW(hdrop, GET_FILE_COUNT_FLAG, null_mut(), 0);

            for i in 0..count {
                const MAX_BUFFER_SIZE: usize = 255;

                let file_name_size = DragQueryFileW(hdrop, i, null_mut(), 0) + 1; // add space for null terminator so it dont replaces last character with null

                if file_name_size > MAX_BUFFER_SIZE as u32 {
                } else {
                    let mut buffer = [0u16; MAX_BUFFER_SIZE];

                    if DragQueryFileW(hdrop, i, buffer.as_mut_ptr(), file_name_size) == 0 {
                        todo!("Error handling")
                    }

                    // remove null terminator
                    let file_name =
                        String::from_utf16_lossy(&buffer[..file_name_size as usize - 1]);

                    handler.file_recived(WindowHandle { windowHandle: hwnd }, file_name)
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
