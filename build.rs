extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Use the following directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Set up correct linker paths for the according architecture
    if cfg!(target_arch = "x86") {
        println!(
            "cargo:rustc-link-search=native={}/libs/x86/", 
            manifest_dir
        );
    } else if cfg!(target_arch = "x86_64") {
        println!(
            "cargo:rustc-link-search=native={}/libs/x86_64/",
            manifest_dir
        );
    }

    // Link against the correct target library
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=PCANBasic");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=pcanbasic");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}/header/wrapper.h", manifest_dir);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/header/wrapper.h", manifest_dir))
        // Allow function that begin with CAN
        .allowlist_function("CAN_.*")
        // Allow variables that begin with PCAN
        .allowlist_var("PCAN_.*")
        // Allow variables that begin with LOG
        .allowlist_var("LOG_.*")
        // Allow variables that begin with TRACE
        .allowlist_var("TRACE_.*")
        // Allow variables that begin with FEATURE
        .allowlist_var("FEATURE_.*")
        // Allow variables that begin with SERVICE
        .allowlist_var("SERVICE_.*")
        // Allow variables that begin with MAX
        .allowlist_var("MAX_.*")
        // Allow variables that begin with LOOKUP
        .allowlist_var("LOOKUP_.*")
        // Allow variables that begin with TPC
        .allowlist_var("TPC_.*")
        // Allow types that begin with PCAN
        .allowlist_type("TPC_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
