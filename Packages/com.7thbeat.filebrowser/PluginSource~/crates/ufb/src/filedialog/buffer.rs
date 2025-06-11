use std::ffi::{CString, c_char};

#[repr(C)]
pub struct CStringBuffer {
    data: *mut *mut c_char,
    len: i32,
}

impl CStringBuffer {
    pub fn new(strings: Vec<CString>) -> Self {
        let mut buffer = strings
            .into_iter()
            .map(|path| path.into_raw())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let data = buffer.as_mut_ptr();
        let len = buffer.len() as _;

        std::mem::forget(buffer);

        CStringBuffer { data, len }
    }
}

/// # Safety
/// 
/// the pointer must not be freed yet
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cstring_buffer_free(buffer: *mut CStringBuffer) {
    if !buffer.is_null() {
        unsafe {
            let buffer = Box::from_raw(buffer);
            let slice = std::slice::from_raw_parts_mut(buffer.data, buffer.len as _);
            for path in slice {
                drop(CString::from_raw(*path));
            }
            drop(Box::from_raw(buffer.data));
            std::mem::drop(buffer);
        }
    }
}
