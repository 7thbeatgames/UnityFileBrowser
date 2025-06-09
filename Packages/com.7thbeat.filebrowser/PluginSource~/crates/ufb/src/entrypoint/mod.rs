use std::ffi::{CString, c_char};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

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
