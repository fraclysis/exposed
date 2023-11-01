use std::{cell::UnsafeCell, collections::HashMap, ffi::c_int, ptr::null_mut};

use libc::c_ulong;
use x11::xlib::{Display, Screen, _XIC, _XIM};

#[derive(Debug, Clone, Copy)]
pub struct Context(pub *mut ThreadContext);

thread_local! {
    static CONTEXT:UnsafeCell<ThreadContext> = UnsafeCell::new(ThreadContext::new());
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub display: *mut Display,
    pub screen: *mut Screen,
    pub wm_delete: c_ulong,
    pub im: *mut _XIM,
    pub screen_id: c_int,
    pub window_map: HashMap<c_ulong, *mut _XIC>,
}

impl ThreadContext {
    pub fn new() -> Self {
        Self { display: null_mut(), screen: null_mut(), screen_id: 0, wm_delete: 0, im: null_mut(), window_map: HashMap::new() }
    }

    pub unsafe fn current_thread() -> &'static mut ThreadContext {
        CONTEXT.with(|c| &mut *c.get())
    }
}
