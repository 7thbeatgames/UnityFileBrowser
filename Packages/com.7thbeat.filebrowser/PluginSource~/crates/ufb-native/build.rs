use std::{env, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let libs = vec!["x11", "gtk+-3.0", "glib-2.0", "libglvnd"];

    let mut libs = libs
        .into_iter()
        .map(|x| pkg_config::probe_library(x).unwrap())
        .map(|x| x.include_paths)
        .flatten()
        .collect::<Vec<_>>();

    libs.push("include".into());

    let cflags = libs
        .iter()
        .map(|x| format!("-I{}", x.to_string_lossy()))
        .collect::<Vec<_>>();

    // wrapper.hpp
    {
        let bindings = bindgen::Builder::default()
            .header("wrapper.hpp")
            .clang_args(&cflags)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    // wrapper_gl.hpp
    {
        let bindings = bindgen::Builder::default()
            .header("wrapper_gl.hpp")
            .clang_args(&cflags)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .dynamic_library_name("GL")
            .generate()
            .expect("Unable to generate egl bindings");
        bindings
            .write_to_file(out_path.join("bindings_gl.rs"))
            .expect("Couldn't write egl bindings!");
    }

    // wrapper_egl.hpp
    {
        let bindings = bindgen::Builder::default()
            .header("wrapper_egl.hpp")
            .clang_args(&cflags)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .dynamic_library_name("EGL")
            .generate()
            .expect("Unable to generate egl bindings");
        bindings
            .write_to_file(out_path.join("bindings_egl.rs"))
            .expect("Couldn't write egl bindings!");
    }
}
