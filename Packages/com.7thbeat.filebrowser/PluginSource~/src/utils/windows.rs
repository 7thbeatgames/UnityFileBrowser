use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle,
    Win32WindowHandle, WindowHandle,
};
use std::num::{NonZeroIsize};
use windows::Win32::Foundation::{FALSE, HWND, LPARAM, TRUE};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumChildWindows, EnumWindows, GetClassNameW, GetWindowThreadProcessId, InternalGetWindowText,
};

pub fn get_windows_hwnd() -> Option<HWND> {
    let mut value: Option<HWND> = None;

    unsafe {
        _ = EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut value as *mut _ as isize),
        );
    }

    value
}

pub struct WindowsWindowParent {
    pub hwnd: HWND,
}

impl HasWindowHandle for WindowsWindowParent {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = Win32WindowHandle::new(NonZeroIsize::new(self.hwnd.0 as _).unwrap());
        let handle = RawWindowHandle::from(handle);
        Ok(unsafe { WindowHandle::borrow_raw(handle) })
    }
}

impl HasDisplayHandle for WindowsWindowParent {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::windows())
    }
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> windows::core::BOOL {
    let value = unsafe { &mut *(lparam.0 as *mut Option<HWND>) };

    let pid = std::process::id();

    let mut window_pid = 0;

    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut window_pid)) };

    if pid != window_pid {
        return TRUE;
    }

    let mut buf: [u16; 512] = [0; 512];
    let len = unsafe { GetClassNameW(hwnd, &mut buf) };
    let window_class = String::from_utf16_lossy(&buf[..len as usize]);

    if window_class == "UnityWndClass" {
        *value = Some(hwnd);
        return FALSE;
    }

    if window_class == "UnityContainerWndClass" {
        _ = unsafe { EnumChildWindows(Some(hwnd), Some(enum_windows_callback), lparam) };
        if value.is_some() {
            return FALSE;
        }
        return TRUE;
    }

    if window_class == "UnityGUIViewWndClass" {
        let len = unsafe { InternalGetWindowText(hwnd, &mut buf) };
        let internal_text = String::from_utf16_lossy(&buf[..len as usize]);

        if internal_text == "UnityEditor.GameView" {
            *value = Some(hwnd);
            return FALSE;
        }
    }

    TRUE
}
