use exposed::window::{
    extern_ffi::{set_error_string, EventHandler},
    WindowHandle,
};

use crate::{
    free_lib_opengl, load_lib_opengl, GlContext, GlContextBuilder, GlDisplay, GlDisplayBuilder,
};

#[no_mangle]
pub unsafe extern "C" fn FractGl_init() -> i32 {
    if let Err(e) = load_lib_opengl() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn FractGl_terminate() -> i32 {
    if let Err(e) = free_lib_opengl() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn GlDisplayBuilder_default(display_builder: *mut GlDisplayBuilder) {
    display_builder.write(GlDisplayBuilder::default())
}

#[no_mangle]
pub unsafe extern "C" fn GlDisplayBuilder_build(
    display_builder: *mut GlDisplayBuilder,
    window: WindowHandle,
    event_handler: *mut EventHandler,
    display: *mut GlDisplay,
) -> i32 {
    let res;

    if display_builder.is_null() {
        res = GlDisplayBuilder::default().build(window, &mut *event_handler);
    } else {
        let display_builder = &mut *display_builder;
        res = display_builder.build(window, &mut *event_handler);
    }

    match res {
        Ok(d) => {
            display.write(d);
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn GlContextBuilder_default(context_builder: *mut GlContextBuilder) {
    context_builder.write(GlContextBuilder::default())
}

#[no_mangle]
pub unsafe extern "C" fn GlContextBuilder_debug_context(context_builder: *mut GlContextBuilder) {
    (&mut *context_builder).with_debug();
}

#[no_mangle]
pub unsafe extern "C" fn GlContextBuilder_with_version(
    context_builder: *mut GlContextBuilder,
    major: i32,
    minor: i32,
) {
    let context_builder = &mut *context_builder;
    context_builder.with_version(major, minor);
}

#[no_mangle]
pub unsafe extern "C" fn GlContextBuilder_build(
    context_builder: *mut GlContextBuilder,
    display: GlDisplay,
    share_context: GlContext,
    context: *mut GlContext,
) -> i32 {
    let res;

    if context_builder.is_null() {
        res = GlContextBuilder::default().build(display, share_context);
    } else {
        let context_builder = &mut *context_builder;
        res = context_builder.build(display, share_context);
    }

    match res {
        Ok(c) => {
            context.write(c);
            0
        }
        Err(e) => {
            set_error_string(e);
            1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn GlDisplay_swap_buffers(display: GlDisplay) -> i32 {
    if let Err(e) = display.swap_buffers() {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn GlDisplay_set_swap_interval(display: GlDisplay, interval: i32) -> i32 {
    if let Err(e) = display.set_swap_interval(interval) {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn GlDisplay_make_current(display: GlDisplay, context: GlContext) -> i32 {
    if let Err(e) = display.make_current(context) {
        set_error_string(e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn GlContext_destroy(context: GlContext) -> i32 {
    if let Err(e) = context.destroy() {
        set_error_string(e);
        return 1;
    };

    0
}
