use std::env;

fn main() {
    // use the following target
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // link against correct target
    if cfg!(windows) {
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
        } else {
            panic!("Unknown architecture");
        }
    } else {
        panic!("Unsupported platform");
    }

    // link against this library
    println!("cargo:rustc-link-lib=PCANBasic");
}


// fn main() {
//     // Tell cargo to tell rustc to link the system bzip2
//     // shared library.

//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("PCANBasic.h")
//         // Finish the builder and generate the bindings.
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");

//     let out_dir = env::var("OUT_DIR").unwrap();

//     // Create .lib file for windows
//     /* if cfg!(windows) {
//         let machine = if cfg!(target_arch = "x86") {
//                 "/Machine:X86"
//             } else if cfg!(target_arch = "x86_64") {
//                 "/Machine:X64"
//             } else {
//                 panic!("Unknown architecture");
//             };


//         let mut output_lib_path = PathBuf::from(&out_dir);
//         output_lib_path.push("PCANBasic.lib");
//         Command::new("lib.exe")
//             .args(&["/def:PCANBasic.def", &format!("/OUT:{}", output_lib_path.to_str().unwrap()), machine])
//             .status().expect("Could not run lib.exe");
//     }*/

//     println!("cargo:rustc-link-lib=PCANBasic");
//     println!("cargo:rustc-link-search=native={}", out_dir);
// }
