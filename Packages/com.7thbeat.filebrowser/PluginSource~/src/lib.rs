mod filedialog;
mod opener;
mod utils;

use std::ffi::{CString, c_char};

pub use filedialog::*;
pub use opener::*;

/// # Safety
///
/// the pointer must not be freed yet
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cstring_free(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            drop(CString::from_raw(ptr));
        }
    }
}

#[test]
fn test_file_dialog() {
    FileDialog::new().pick_file();
}

#[test]
fn test_file_opener() {
    reveal("Cargo.toml");
}
