fn main() {
    #[cfg(feature = "clippy")]
    {
        println!("cargo:warning=Skipping tauri_build during Clippy");
    }

    #[cfg(not(feature = "clippy"))]
    tauri_build::build();

    // Ensure Android-specific JNI symbols are exported
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-cfg=target_os=\"android\"");
    }
}
