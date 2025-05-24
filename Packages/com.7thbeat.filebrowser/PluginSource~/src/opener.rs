use std::{
    ffi::{CStr, c_char},
    path::Path,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn opener_reveal_file(path: *const c_char) {
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap() };
    reveal_file(path);
}

pub fn reveal_file(path: &str) -> bool {
    opener::reveal(Path::new(path)).is_ok()
}
