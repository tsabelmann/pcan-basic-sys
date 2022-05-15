use std::env;

fn main() {
    // use the following target
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // set up correct linker paths for the according architecture
    if cfg!(target_arch = "x86") {
        println!("cargo:rustc-link-search=native={}/libs/x86/", manifest_dir);
    } else if cfg!(target_arch = "x86_64") {
        println!(
            "cargo:rustc-link-search=native={}/libs/x86_64/",
            manifest_dir
        );
    }

    // link against the correct target library
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=PCANBasic");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=pcanbasic");
    }
}
