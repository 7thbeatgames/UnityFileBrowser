use std::{
    ffi::{CString, c_char},
    path::{Path, PathBuf},
};

pub mod buffer;

pub use buffer::*;

pub struct FileDialog {
    pub dialog: rfd::FileDialog,
}

impl Default for FileDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl FileDialog {
    #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        let mut dialog = Self::_new();

        let window = crate::utils::linux::get_window_handle();
        if let Some(window) = &window {
            dialog.dialog = dialog.dialog.set_parent(window);
        };

        dbg!(&dialog.dialog);

        dialog
    }

    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        let mut dialog = Self::_new();

        let window = unsafe { crate::utils::macos::get_game_window() };
        dialog.dialog = dialog.dialog.set_parent(&window);

        dialog
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        use crate::utils::windows;
        let mut dialog = Self::_new();

        let hwnd = windows::get_windows_hwnd();

        if let Some(hwnd) = hwnd {
            dialog.dialog = dialog
                .dialog
                .set_parent(&windows::WindowsWindowParent { hwnd });
        }

        dialog
    }

    fn _new() -> Self {
        FileDialog {
            dialog: rfd::FileDialog::new(),
        }
    }

    pub fn set_directory(&mut self, directory: &std::path::Path) {
        self.dialog = std::mem::take(&mut self.dialog).set_directory(directory);
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.dialog = std::mem::take(&mut self.dialog).set_file_name(filename);
    }

    pub fn pick_file(&self) -> Option<PathBuf> {
        self.dialog.clone().pick_file()
    }

    pub fn save_file(&self) -> Option<PathBuf> {
        self.dialog.clone().save_file()
    }

    pub fn add_filter(&mut self, name: &str, extensions: &[&str]) {
        self.dialog = std::mem::take(&mut self.dialog).add_filter(name, extensions);
    }

    pub fn pick_files(&self) -> Option<Vec<PathBuf>> {
        self.dialog.clone().pick_files()
    }

    pub fn pick_folder(&self) -> Option<PathBuf> {
        self.dialog.clone().pick_folder()
    }

    pub fn pick_folders(&self) -> Option<Vec<PathBuf>> {
        self.dialog.clone().pick_folders()
    }

    pub fn set_can_create_directories(&mut self, can_create: bool) {
        self.dialog = std::mem::take(&mut self.dialog).set_can_create_directories(can_create);
    }

    pub fn set_title(&mut self, title: &str) {
        self.dialog = std::mem::take(&mut self.dialog).set_title(title);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_dialog_create() -> *mut FileDialog {
    Box::into_raw(Box::new(FileDialog::new()))
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_destroy(dialog: *mut FileDialog) {
    unsafe {
        drop(Box::from_raw(dialog));
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_set_directory(
    dialog: *mut FileDialog,
    directory: *const c_char,
) {
    unsafe {
        let directory = std::ffi::CStr::from_ptr(directory).to_str().unwrap();
        let dialog = &mut *dialog;
        dialog.set_directory(Path::new(directory));
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_set_file_name(
    dialog: *mut FileDialog,
    filename: *const c_char,
) {
    unsafe {
        let filename = std::ffi::CStr::from_ptr(filename).to_str().unwrap();
        let dialog = &mut *dialog;
        dialog.set_file_name(filename);
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_pick_file(dialog: *mut FileDialog) -> *mut c_char {
    unsafe {
        let dialog = &mut *dialog;
        convert_optional_path_to_raw(dialog.pick_file())
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_save_file(dialog: *mut FileDialog) -> *mut c_char {
    unsafe {
        let dialog = &mut *dialog;
        convert_optional_path_to_raw(dialog.save_file())
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_add_filter(
    dialog: *mut FileDialog,
    name: *const c_char,
    extensions: *const *const c_char,
    extensions_count: i32,
) {
    unsafe {
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap();
        let extensions = std::slice::from_raw_parts(extensions, extensions_count as _);
        let extensions = extensions
            .iter()
            .map(|ext| std::ffi::CStr::from_ptr(*ext).to_str().unwrap())
            .collect::<Vec<_>>();
        let dialog = &mut *dialog;
        dialog.add_filter(name, &extensions);
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_pick_files(dialog: *mut FileDialog) -> *mut CStringBuffer {
    unsafe {
        let dialog = &mut *dialog;
        convert_optional_path_vec_to_raw(dialog.pick_files())
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_pick_folder(dialog: *mut FileDialog) -> *mut c_char {
    unsafe {
        let dialog = &mut *dialog;

        convert_optional_path_to_raw(dialog.pick_folder())
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_pick_folders(dialog: *mut FileDialog) -> *mut CStringBuffer {
    unsafe {
        let dialog = &mut *dialog;
        convert_optional_path_vec_to_raw(dialog.pick_folders())
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_set_can_create_directories(
    dialog: *mut FileDialog,
    can_create: bool,
) {
    unsafe {
        let dialog = &mut *dialog;
        dialog.set_can_create_directories(can_create);
    }
}

/// # Safety
///
/// This function must have valid FileDialog pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_dialog_set_title(dialog: *mut FileDialog, filename: *const c_char) {
    unsafe {
        let filename = std::ffi::CStr::from_ptr(filename).to_str().unwrap();
        let dialog = &mut *dialog;
        dialog.set_title(filename);
    }
}

#[inline]
fn convert_optional_path_to_raw(data: Option<PathBuf>) -> *mut c_char {
    match data {
        Some(path) => CString::new(path.to_string_lossy().to_string())
            .unwrap()
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[inline]
fn convert_optional_path_vec_to_raw(data: Option<Vec<PathBuf>>) -> *mut CStringBuffer {
    match data {
        Some(paths) => {
            let paths = paths
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .map(|path| CString::new(path).unwrap())
                .collect::<Vec<_>>();
            let buffer = CStringBuffer::new(paths);
            let buffer = Box::new(buffer);
            Box::into_raw(buffer)
        }
        None => std::ptr::null_mut(),
    }
}
