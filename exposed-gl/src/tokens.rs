pub mod pgl {
    #[cfg(target_os = "windows")]
    pub use glutin_wgl_sys::wgl_extra::{types::*, *};

    #[cfg(target_os = "linux")]
    pub use glutin_glx_sys::glx_extra::{types::*, *};

    #[cfg(target_os = "android")]
    pub use glutin_egl_sys::egl::*;
}

#[cfg(not(target_os = "android"))]
pub const END: u32 = 0;

#[cfg(target_os = "android")]
pub const END: u32 = pgl::NONE;

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! surface_config {
    ($($arg:expr),*) => {
        [$($arg),* $crate::tokens::END]
    };
}

#[macro_export]
#[cfg(target_os = "linux")]
macro_rules! surface_config {
    ($($arg:tt)*) => {
        [
            $crate::tokens::pgl::X_RENDERABLE, 1,
            $crate::tokens::pgl::DRAWABLE_TYPE, $crate::tokens::pgl::WINDOW_BIT,
            $crate::tokens::pgl::RENDER_TYPE, $crate::tokens::pgl::RGBA_BIT,
            $crate::tokens::pgl::X_VISUAL_TYPE, $crate::tokens::pgl::TRUE_COLOR,
            $crate::tokens::pgl::DOUBLEBUFFER, 1,
            $crate::tokens::DEPTH_BITS_ARB, 1,
            $crate::tokens::STENCIL_BITS_ARB, 1,
            $($arg)*
            0
        ]
    };
}

#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! surface_config {
    ($($arg:tt)*) => {
        [
            $crate::tokens::pgl::DRAW_TO_WINDOW_ARB, 1,
            $crate::tokens::pgl::SUPPORT_OPENGL_ARB, 1,
            $crate::tokens::pgl::DOUBLE_BUFFER_ARB, 1,
            $crate::tokens::pgl::ACCELERATION_ARB, $crate::tokens::pgl::FULL_ACCELERATION_ARB,
            $crate::tokens::pgl::PIXEL_TYPE_ARB, $crate::tokens::pgl::TYPE_RGBA_ARB,
            $crate::tokens::DEPTH_BITS_ARB, 1,
            $crate::tokens::STENCIL_BITS_ARB, 1,
            $($arg)*
            0
        ]
    };
}

macro_rules! token_def {
    ($name:tt, $wgl:tt, $glx:tt, $egl:tt) => {
        #[cfg(target_os = "windows")]
        pub const $name: u32 = pgl::$wgl;

        #[cfg(target_os = "linux")]
        pub const $name: u32 = pgl::$glx;

        #[cfg(target_os = "android")]
        pub const $name: u32 = pgl::$egl;
    };
}

token_def!(RED_BITS_ARB, RED_BITS_ARB, RED_SIZE, RED_SIZE);
token_def!(GREEN_BITS_ARB, GREEN_BITS_ARB, GREEN_SIZE, GREEN_SIZE);
token_def!(BLUE_BITS_ARB, BLUE_BITS_ARB, BLUE_SIZE, BLUE_SIZE);
token_def!(ALPHA_BITS_ARB, ALPHA_BITS_ARB, ALPHA_SIZE, ALPHA_SIZE);
token_def!(COLOR_BITS_ARB, COLOR_BITS_ARB, BUFFER_SIZE, BUFFER_SIZE);

token_def!(DEPTH_BITS_ARB, DEPTH_BITS_ARB, DEPTH_SIZE, DEPTH_SIZE);
token_def!(STENCIL_BITS_ARB, STENCIL_BITS_ARB, STENCIL_SIZE, STENCIL_SIZE);

token_def!(SAMPLE_BUFFERS_ARB, SAMPLE_BUFFERS_ARB, SAMPLE_BUFFERS_ARB, SAMPLE_BUFFERS);
token_def!(SAMPLES_ARB, SAMPLES_ARB, SAMPLES_ARB, SAMPLES);

token_def!(CONTEXT_MAJOR_VERSION_ARB, CONTEXT_MAJOR_VERSION_ARB, CONTEXT_MAJOR_VERSION_ARB, CONTEXT_MAJOR_VERSION);
token_def!(CONTEXT_MINOR_VERSION_ARB, CONTEXT_MINOR_VERSION_ARB, CONTEXT_MINOR_VERSION_ARB, CONTEXT_MINOR_VERSION);

token_def!(CONTEXT_FLAGS_ARB, CONTEXT_FLAGS_ARB, CONTEXT_FLAGS_ARB, CONTEXT_FLAGS_KHR);
token_def!(CONTEXT_DEBUG_BIT_ARB, CONTEXT_DEBUG_BIT_ARB, CONTEXT_DEBUG_BIT_ARB, CONTEXT_OPENGL_DEBUG_BIT_KHR);
token_def!(
    CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
    CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
    CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
    CONTEXT_OPENGL_FORWARD_COMPATIBLE_BIT_KHR
);
token_def!(
    CONTEXT_ROBUST_ACCESS_BIT_ARB,
    CONTEXT_ROBUST_ACCESS_BIT_ARB,
    CONTEXT_ROBUST_ACCESS_BIT_ARB,
    CONTEXT_OPENGL_ROBUST_ACCESS_BIT_KHR
);

token_def!(CONTEXT_PROFILE_MASK_ARB, CONTEXT_PROFILE_MASK_ARB, CONTEXT_PROFILE_MASK_ARB, CONTEXT_OPENGL_PROFILE_MASK_KHR);
token_def!(
    CONTEXT_CORE_PROFILE_BIT_ARB,
    CONTEXT_CORE_PROFILE_BIT_ARB,
    CONTEXT_CORE_PROFILE_BIT_ARB,
    CONTEXT_OPENGL_CORE_PROFILE_BIT_KHR
);
token_def!(
    CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
    CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
    CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
    CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT_KHR
);
