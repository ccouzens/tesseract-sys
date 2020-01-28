extern crate bindgen;

use std::env;
use std::path::PathBuf;
#[cfg(windows)]
use vcpkg;

#[cfg(windows)]
fn find_tesseract_system_lib() -> Option<String> {
    let lib = vcpkg::Config::new().find_package("leptonica").unwrap();

    let include = lib
        .include_paths
        .iter()
        .map(|x| x.to_string_lossy())
        .collect::<String>();
    Some(include)
}

#[cfg(not(windows))]
fn find_tesseract_system_lib() -> Option<String> {
    println!("cargo:rustc-link-lib=tesseract");
    None
}

fn main() {
    // Tell cargo to tell rustc to link the system tesseract
    // and leptonica shared libraries.
    let clang_extra_include = find_tesseract_system_lib();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut capi_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper_capi.h")
        .whitelist_function("^Tess.*")
        .blacklist_type("Boxa")
        .blacklist_type("Pix")
        .blacklist_type("Pixa")
        .blacklist_type("_IO_FILE")
        .blacklist_type("_IO_codecvt")
        .blacklist_type("_IO_marker")
        .blacklist_type("_IO_wide_data");

    if let Some(clang_extra_include) = &clang_extra_include {
        capi_bindings = capi_bindings.clang_arg(format!("-I{}", clang_extra_include));
    }
    // Finish the builder and generate the bindings.
    let capi_bindings = capi_bindings
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate capi bindings");

    let mut public_types_bindings = bindgen::Builder::default()
        .header("wrapper_public_types.hpp")
        .whitelist_var("^k.*")
        .blacklist_item("kPolyBlockNames");

    if let Some(clang_extra_include) = &clang_extra_include {
        public_types_bindings =
            public_types_bindings.clang_arg(format!("-I{}", clang_extra_include));
    }
    
    let public_types_bindings = public_types_bindings
        .generate()
        .expect("Unable to generate public types bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    capi_bindings
        .write_to_file(out_path.join("capi_bindings.rs"))
        .expect("Couldn't write capi bindings!");
    public_types_bindings
        .write_to_file(out_path.join("public_types_bindings.rs"))
        .expect("Couldn't write public types bindings!");
}
