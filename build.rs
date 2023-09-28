use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(&manifest_dir).join("lib");
    let wrapper_path = PathBuf::from(&lib_dir).join("wrapper.h");

    println!("cargo:rustc-link-lib=OpenImageDenoise");
    println!("cargo:rerun-if-changed={}", wrapper_path.display());

    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let bindings_source = bindings.to_string();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("bindings.rs");

    std::fs::write(out_path, bindings_source).expect("Couldn't write bindings!");
}
