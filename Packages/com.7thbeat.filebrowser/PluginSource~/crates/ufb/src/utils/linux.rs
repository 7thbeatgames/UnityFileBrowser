use std::{
    ffi::{c_ulong, c_void},
    ptr::NonNull,
    sync::Mutex,
};

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle, XlibDisplayHandle,
    XlibWindowHandle,
};

#[derive(Clone, Debug)]
pub enum LinuxWindowRef {
    Xlib {
        display: NonNull<c_void>,
        window: c_ulong,
    },
}

static CURRENT_WINDOW: Mutex<Option<LinuxWindowRef>> = Mutex::new(None);

unsafe impl Send for LinuxWindowRef {}
unsafe impl Sync for LinuxWindowRef {}

pub fn set_window_handle(handle: LinuxWindowRef) {
    let mut data = CURRENT_WINDOW.lock().unwrap();
    *data = Some(handle);
}

pub fn get_window_handle() -> Option<LinuxWindowRef> {
    let value = CURRENT_WINDOW.lock().unwrap();
    value.clone()
}

impl HasWindowHandle for LinuxWindowRef {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            match self {
                Self::Xlib { window, .. } => Ok(WindowHandle::borrow_raw(
                    XlibWindowHandle::new(*window).into(),
                )),
            }
        }
    }
}
impl HasDisplayHandle for LinuxWindowRef {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            match self {
                Self::Xlib { display, .. } => Ok(DisplayHandle::borrow_raw(
                    XlibDisplayHandle::new(Some(*display), 0).into(),
                )),
            }
        }
    }
}
