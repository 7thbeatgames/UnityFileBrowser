use std::{
    ffi::{CString, c_char},
    path::Path,
};

pub struct FileDialog {
    pub dialog: Option<rfd::FileDialog>,
}

impl FileDialog {
    pub fn new() -> Self {
        FileDialog {
            dialog: Some(rfd::FileDialog::new()),
        }
    }

    pub fn set_directory(&mut self, directory: &std::path::Path) {
        self.dialog = Some(self.dialog.take().unwrap().set_directory(directory));
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.dialog = Some(self.dialog.take().unwrap().set_file_name(filename));
    }

    pub fn pick_file(&self) -> Option<std::path::PathBuf> {
        self.dialog.as_ref().unwrap().clone().pick_file()
    }

    pub fn save_file(&self) -> Option<std::path::PathBuf> {
        self.dialog.as_ref().unwrap().clone().save_file()
    }

    pub fn add_filter(&mut self, name: &str, extensions: &[&str]) {
        self.dialog = Some(self.dialog.take().unwrap().add_filter(name, extensions));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_create() -> *mut FileDialog {
    Box::into_raw(Box::new(FileDialog::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_destroy(dialog: *mut FileDialog) {
    unsafe {
        drop(Box::from_raw(dialog));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_set_directory(dialog: *mut FileDialog, directory: *const c_char) {
    unsafe {
        let directory = std::ffi::CStr::from_ptr(directory).to_str().unwrap();
        let dialog = &mut *dialog;
        dialog.set_directory(Path::new(directory));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_set_file_name(dialog: *mut FileDialog, filename: *const c_char) {
    unsafe {
        let filename = std::ffi::CStr::from_ptr(filename).to_str().unwrap();
        let dialog = &mut *dialog;
        dialog.set_file_name(filename);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_pick_file(dialog: *mut FileDialog) -> *mut c_char {
    unsafe {
        let dialog = &mut *dialog;
        match dialog.pick_file() {
            Some(path) => CString::new(path.to_string_lossy().to_string())
                .unwrap()
                .into_raw(),
            None => std::ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_save_file(dialog: *mut FileDialog) -> *mut c_char {
    unsafe {
        let dialog = &mut *dialog;
        match dialog.save_file() {
            Some(path) => CString::new(path.to_string_lossy().to_string())
                .unwrap()
                .into_raw(),
            None => std::ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_add_filter(
    dialog: *mut FileDialog,
    name: *const c_char,
    extensions: *const *const c_char,
    count: i32,
) {
    unsafe {
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap();
        let extensions = std::slice::from_raw_parts(extensions, count as _);
        let extensions = extensions
            .iter()
            .map(|ext| std::ffi::CStr::from_ptr(*ext).to_str().unwrap())
            .collect::<Vec<_>>();
        let dialog = &mut *dialog;
        dialog.add_filter(name, &extensions);
    }
}
