pub mod triangle;
pub mod vk;

// #[cfg(target_os = "android")]
// pub mod android;
// #[cfg(not(target_os = "android"))]
pub mod app;

exposed::window::android_on_create!(exposed::window::Android<app::App>);
