extern crate bindgen;

use std::env;
//use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system rofi shared library.
    println!("cargo:rustc-link-lib=rofi");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // make generated code #![no_std] compatible
        .ctypes_prefix("cty")
        .use_core()
        // The input header we would like to generate
        // bindings for.
        .header("../include/theme.h")
        .header("../include/widgets/widget-internal.h")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        .clang_arg("-I/usr/include/cairo")
        .clang_arg("-I../include")
        .clang_arg("-I../subprojects/libnkutils/src")
        // .rustified_enum("hid_keyboard_keypad_usage")     // this might be worth having a look at
        .rustfmt_bindings(true)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    env::set_var("OUT_DIR", "src");
    //let out_path = zPathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
