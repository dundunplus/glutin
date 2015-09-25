#![cfg(target_os = "windows")]

use libc;
use Window;

/// Additional methods on `Window` that are specific to unix.
pub trait WindowExt {
    /// Returns a pointer to the `Window` object of xlib that is used by this window.
    ///
    /// Returns `None` if the window doesn't use xlib (if it uses wayland for example).
    ///
    /// The pointer will become invalid when the glutin `Window` is destroyed.
    fn get_hwnd(&self) -> *mut libc::c_void;
}

impl WindowExt for Window {
    #[inline]
    fn get_hwnd(&self) -> *mut libc::c_void {
        self.window.platform_window()
    }
}
