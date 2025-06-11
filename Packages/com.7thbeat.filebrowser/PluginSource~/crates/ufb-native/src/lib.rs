#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "linux")]
pub mod egl {
    include!(concat!(env!("OUT_DIR"), "/bindings_egl.rs"));
}

#[cfg(target_os = "linux")]
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings_gl.rs"));
}

#[cfg(target_os = "linux")]
pub mod gtk {
    include!(concat!(env!("OUT_DIR"), "/bindings_gtk3.rs"));
    include!(concat!(env!("OUT_DIR"), "/bindings_gio.rs"));
}
