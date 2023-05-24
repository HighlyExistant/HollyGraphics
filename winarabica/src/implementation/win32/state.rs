use windows_sys::{Win32::{UI::WindowsAndMessaging::*, Foundation::HMODULE, Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, FillRect, BRUSH_STYLE, CreateSolidBrush, EndPaint}}, core::PCWSTR};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use std::{sync::{Arc, Mutex}, ffi::c_void};
pub(crate) struct  State {
    pub msg: MSG,
    pub callback: Option<Box<dyn FnMut(*const c_void) -> bool + 'static>>,
    pub keypressed: usize,
}

impl State {
    pub fn new() -> Self {
        let msg = MSG {
            hwnd: 0,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT {
                x: 0,
                y: 0,
            }
        };
        Self { msg, callback: None, keypressed: 0 }
    }
}