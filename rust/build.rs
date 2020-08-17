extern crate bindgen;
extern crate cbindgen;

use std::env;
//use std::path::PathBuf;

fn main() {
    // ###################### cbindgen #################################

    // let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // println!("crate_dir: {}", crate_dir);
    // //let config = cbindgen::Config::from_file("cbindgen.toml");
    // let mut config: cbindgen::Config = Default::default();
    // config.language = cbindgen::Language::C;
    // cbindgen::generate_with_config(&crate_dir, config)
    //     .unwrap()
    //     .write_to_file("box.h");

    // ################################################################

    // Tell cargo to tell rustc to link the system rofi shared library.
    println!("cargo:rustc-link-lib=rofi");

    // println!("cargo:rustc-link-search=native={}", "../build/source");
    // println!("cargo:rustc-link-search=native={}", "../include");
    // println!("cargo:rustc-link-search=native={}", "../include/widgets");
    // println!("cargo:rustc-link-search=native={}", "../build/lexer");

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
        .header("../include/widgets/widget.h")
        .header("../include/widgets/widget-internal.h")
        .trust_clang_mangling(false)
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        .clang_arg("-I/usr/include/cairo")
        .clang_arg("-I../include")
        .clang_arg("-I../include/widgets")
        // .clang_arg("-I../build/source")
        // .clang_arg("-I../build/lexer")
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
