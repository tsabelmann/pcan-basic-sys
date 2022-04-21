use std::env;

fn main() {
    // use the following target
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // link against correct target
    if cfg!(windows) {
        if cfg!(target_arch = "x86") {
            println!("cargo:rustc-link-search=native={}/libs/x86/", manifest_dir);
        } else if cfg!(target_arch = "x86_64") {
            println!(
                "cargo:rustc-link-search=native={}/libs/x86_64/",
                manifest_dir
            );
        }
    }

    // link against this library
    println!("cargo:rustc-link-lib=PCANBasic");
}
