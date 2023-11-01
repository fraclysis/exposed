use std::{
    alloc::Layout,
    ffi::{c_ulong, c_ushort},
    io::Error,
    ptr::{null, null_mut},
};

use exposed_macro::cstr;
use libc::{c_void, printf, setlocale, LC_ALL};
use x11::xlib::{
    Screen, True, XDefaultScreen, XDefaultScreenOfDisplay, XGetIMValues, XInternAtom, XNQueryInputStyle_0, XOpenDisplay, XOpenIM,
    XSetLocaleModifiers, XSupportsLocale, _XIC,
};

use crate::window::{Context, Event, EventHandler};

#[repr(C)]
#[allow(non_snake_case)]
pub struct EventHandlerBuilder {
    pub place_holder: i32,
}

impl Default for EventHandlerBuilder {
    fn default() -> Self {
        Self { place_holder: 0 }
    }
}

impl EventHandlerBuilder {
    pub unsafe fn build_with<E: Event>(&self, user_data: *mut E) -> Result<EventHandler<E>, Error> {
        static mut CALL_COUNTER: i32 = 0;

        type XIMStyle = c_ulong;

        #[repr(C)]
        struct XIMStyles {
            count_styles: c_ushort,
            supported_styles: *mut XIMStyle,
        }

        unsafe {
            if CALL_COUNTER == 0 {
                CALL_COUNTER += 1;

                if setlocale(LC_ALL, cstr!("")).is_null() {
                    todo!()
                }

                if XSupportsLocale() == 0 {
                    todo!("Error Handling")
                }

                if XSetLocaleModifiers(cstr!("@im=none")).is_null() {
                    todo!("Error Handling")
                }
            }
        }

        let display = unsafe { XOpenDisplay(null()) };
        if display.is_null() {
            todo!()
        }

        let screen: *mut Screen = unsafe { XDefaultScreenOfDisplay(display) };
        if screen.is_null() {
            todo!()
        }

        let screen_id = unsafe { XDefaultScreen(display) };

        let im = unsafe { XOpenIM(display, null_mut(), null_mut(), null_mut()) };
        if im.is_null() {
            todo!()
        }

        let mut styles: *mut XIMStyles = null_mut();
        let styles_ptr = &mut styles as *mut *mut XIMStyles;

        let failed_args = unsafe { XGetIMValues(im, XNQueryInputStyle_0.as_ptr(), styles_ptr, 0usize) };
        if !failed_args.is_null() {
            todo!()
        }

        // TODO make here more safer
        let styles = unsafe { &mut *styles };

        for i in 0..styles.count_styles {
            let string = unsafe { *(styles.supported_styles.add(i as usize)) };
            unsafe { printf(cstr!("style %d"), string) };
        }

        let wm_delete = unsafe { XInternAtom(display, "WM_DELETE_WINDOW\0".as_ptr().cast(), True) };

        let window_data: *mut intmap::IntMap<*mut _XIC> = std::alloc::alloc(Layout::new::<intmap::IntMap<*mut _XIC>>()) as _;
        window_data.write(intmap::IntMap::new());

        let event_handler = EventHandler {
            user_data,
            display,
            screen_id,
            screen,
            im,
            wm_delete,
            window_data: window_data as *mut c_void,
            key_event_buffer: [0; 25],
            event: std::mem::zeroed(),
            running: true,
        };

        if let Some(e) = E::create(Context { display }) {
            user_data.write(e);
        }

        Ok(event_handler)
    }
}
