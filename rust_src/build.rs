use std::env;
use std::process::Command;

/// Runs before the rust is compiled, doing any required setup and printing rust compiler flags.
fn main() {
    // The ouput directory is provided by the cargo build tool as an environment variable.
    let out_dir = env::var("OUT_DIR").unwrap();
    // This is the library we wish to use
    let cpp_lib_path = "../cpp_src/libexample.a";
    // It is probbaly best to work on a copy of the library.
    Command::new("cp")
        .args(&[cpp_lib_path, &out_dir])
        .status()
        .unwrap();
    // Rust build flags:
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=example");
    println!("cargo:rerun-if-changed={}", cpp_lib_path);
}
