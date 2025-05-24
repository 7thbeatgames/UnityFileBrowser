use std::{
    ffi::{CStr, c_char},
    path::Path,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn opener_reveal(path: *const c_char) {
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap() };
    reveal(path);
}

pub fn reveal(path: &str) -> bool {
    opener::reveal(Path::new(path)).is_ok()
}
