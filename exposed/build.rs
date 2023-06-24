fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(target_os = "linux")]
    lib_x11();
}

#[cfg(target_os = "linux")]
fn lib_x11() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=X11");
}
