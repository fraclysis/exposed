use std::{
    io::{Error, ErrorKind},
    mem::zeroed,
    sync::atomic::Ordering,
};

use ndk_sys::{
    AHardwareBuffer_Format, ALooper_wake, ANativeWindow, ANativeWindow_getHeight, ANativeWindow_getWidth, ANativeWindow_lock,
    ANativeWindow_unlockAndPost,
};
use unsafe_utilities::broke_checker::AsReference;

use crate::{
    destroy::Destroy,
    window::{platform::WaitState, Event, Rect, Size},
};

use super::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle {
    pub context: Context,
}

impl Destroy for WindowHandle {
    fn destroy(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl WindowHandle {
    pub fn release_capture(self) -> Result<(), Error> {
        Ok(())
    }

    pub fn set_capture(self) {}

    pub fn debug_paint(self) -> Result<(), Error> {
        unsafe {
            let mut buffer = zeroed();
            let mut bounds = zeroed();

            if ANativeWindow_lock(self.native_handle(), &mut buffer, &mut bounds) != 0 {
                return Err(Error::new(std::io::ErrorKind::Other, "Failed to debug paint window."));
            }

            match AHardwareBuffer_Format(buffer.format as _) {
                AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM => {
                    let color = u32::MAX;
                    std::slice::from_raw_parts_mut(buffer.bits as *mut u32, (buffer.height * buffer.width) as _).fill(color)
                }

                AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R5G6B5_UNORM => {
                    let color: u16 = u16::MAX;
                    std::slice::from_raw_parts_mut(buffer.bits as *mut u16, (buffer.height * buffer.width) as _).fill(color)
                }

                _ => todo!(),
            }

            if ANativeWindow_unlockAndPost(self.native_handle()) != 0 {
                return Err(Error::new(std::io::ErrorKind::Other, "Failed to debug paint window."));
            }

            Ok(())
        }
    }

    pub fn native_handle(self) -> *mut ANativeWindow {
        unsafe { self.context.0.to_ref().native_window.load(std::sync::atomic::Ordering::Acquire) }
    }

    pub fn show(self) -> Result<(), Error> {
        // TODO:(fraclysis) Check for if window is resized
        Ok(())
    }

    pub fn update(self) -> Result<(), Error> {
        todo!()
    }

    pub fn redraw(self) -> Result<(), Error> {
        // REMOVE
        unsafe {
            let context = self.context.0.to_ref();
            let looper = context.looper.load(Ordering::Acquire);

            ALooper_wake(looper);
        }

        Ok(())
    }

    pub fn window_title(self) -> Result<String, Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented."))
    }

    pub fn set_window_title(self, _title: &str) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented."))
    }

    pub fn dpi(self) -> Result<u32, Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented."))
    }

    pub fn client_size(&self) -> Result<Size, Error> {
        unsafe {
            let width = ANativeWindow_getWidth(self.native_handle());
            let height = ANativeWindow_getHeight(self.native_handle());

            Ok(Size { width, height })
        }
    }

    pub fn client_rect(self) -> Result<Rect, Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented."))
    }

    pub fn window_rect(self) -> Result<Rect, Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented."))
    }

    pub fn get_window_size(&self) -> Result<Size, Error> {
        self.client_size()
    }
}

impl Into<crate::window::WindowHandle> for WindowHandle {
    fn into(self) -> crate::window::WindowHandle {
        crate::window::WindowHandle(self)
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowBuilder {}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(&mut self, _title: &str) -> &mut Self {
        self
    }

    pub fn with_size(&mut self, _width: i32, _height: i32) -> &mut Self {
        self
    }

    /// Blocks Android thread until WindowHandle is created
    pub fn build<E: Event>(&self, context: Context) -> Result<WindowHandle, Error> {
        use WaitState::*;

        let context_ = context.get();

        if context_.window_created.load(Ordering::Acquire) {
            return Err(Error::new(ErrorKind::Other, "Only one window can be created in Android."));
        }

        loop {
            let waits = context_.waits_at();

            match waits {
                Running => continue,

                OnCreate => {
                    // Android thread is waiting app to initialize
                    context_.post_receiver()?;
                }

                OnNativeWindowCreated => {
                    return Ok(WindowHandle { context });
                }

                OnDestroy => return Err(Error::new(ErrorKind::Other, "Received onDestroy!")),

                OnNativeWindowResized
                | OnNativeWindowRedrawNeeded
                | OnNativeWindowDestroyed
                | OnWindowFocusChanged
                | OnContentRectChanged => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Window needs to present in order to get {:?} messages.", waits),
                    ))
                }

                OnConfigurationChanged => {
                    // TODO:(fraclysis) Update configuration
                }

                OnSaveInstanceState | OnLowMemory => {
                    // TODO:(fraclysis) Unexpected Message that we cannot discard
                    context_.unhandled_messages.push(1);
                    context_.post_receiver()?;
                }

                OnInputQueueCreated => {
                    context_.looper_attach();
                    context_.post_receiver()?;
                }
                OnInputQueueDestroyed => {
                    context_.looper_detach();
                    context_.post_receiver()?;
                }

                None => unreachable!(),
            }
        }
    }
}
