use std::{cell::UnsafeCell, ops::Deref};

use windows_sys::core::PCWSTR;

use crate::window::Event;

thread_local! {
    static CONTEXT: UnsafeCell<ThreadContext> = UnsafeCell::new(ThreadContext::zeroed());
}

#[derive(Clone, Copy)]
pub struct ThreadContext {
    pub user_data: *mut u8,
    pub window_class: PCWSTR,
    pub last_char: u16,
}

impl ThreadContext {
    pub unsafe fn get() -> ThreadContext {
        CONTEXT.with(|c: &UnsafeCell<ThreadContext>| *c.get())
    }

    pub unsafe fn get_ref() -> &'static mut ThreadContext {
        CONTEXT.with(|c: &UnsafeCell<ThreadContext>| &mut *c.get())
    }

    pub unsafe fn user_data<E: Event>() -> *mut E {
        Self::get().user_data as *mut E
    }

    fn zeroed() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context(pub *mut ThreadContext);

impl Context {
    pub fn current_thread() -> Self {
        Self(unsafe { ThreadContext::get_ref() })
    }
}

impl Deref for Context {
    type Target = ThreadContext;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
