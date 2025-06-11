use std::{ffi::c_void, ptr::NonNull};

use dispatch2::run_on_main;
use objc2::rc::Retained;
use objc2_app_kit::NSApplication;
use raw_window_handle::{
    AppKitWindowHandle, DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle,
    RawWindowHandle, WindowHandle,
};

pub unsafe fn get_game_window() -> MacOSWindowHandle {
    run_on_main(|mtm| unsafe {
        let app = NSApplication::sharedApplication(mtm);
        let window =
            Retained::as_ptr(&app.mainWindow().unwrap().contentView().unwrap()) as *mut c_void;

        let handle = AppKitWindowHandle::new(NonNull::new(window).unwrap());
        let handle = RawWindowHandle::from(handle);

        MacOSWindowHandle { handle }
    })
}

pub struct MacOSWindowHandle {
    handle: RawWindowHandle,
}

unsafe impl Send for MacOSWindowHandle {}
unsafe impl Sync for MacOSWindowHandle {}

impl HasWindowHandle for MacOSWindowHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(unsafe { WindowHandle::borrow_raw(self.handle) })
    }
}

impl HasDisplayHandle for MacOSWindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::appkit())
    }
}
