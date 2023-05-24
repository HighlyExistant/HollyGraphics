use std::{ffi::c_void, marker::PhantomData};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, Win32WindowHandle};
use windows_sys::Win32::Foundation;
use crate::implementation::platform;

pub struct Window {
    pub(super) window: platform::Window,
}
pub enum ProgramState {
    Blocking,
    Nonblocking
}
impl Window {
    pub fn new() -> Self {
        let window: platform::Window = platform::Window::new();
        Self { window }
    }
    pub fn set_callback(&mut self, callback: Box<dyn FnMut(*const c_void) -> bool + 'static>) {
        self.window.callback = Some(callback);
    }
    pub fn get_messege(&self, state: ProgramState, user: Option<&dyn Fn(usize)>) {
        match state {
            ProgramState::Blocking => {
                unsafe { self.window.get_messege_block(user) };
            }
            ProgramState::Nonblocking => {
                unsafe { self.window.get_messege_nonblock(user) };
            }
        }
    }
    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        self.window.raw_display_handle()
    }
    pub fn raw_window_handle(&self) -> RawWindowHandle {
        let mut window_handle = Win32WindowHandle::empty();
        let hwnd = self.window.get_hwnd();
        window_handle.hwnd = self.window.get_hwnd() as *mut _;
        window_handle.hinstance = self.window.hinstance() as *mut _;
        RawWindowHandle::Win32(window_handle)
    }
    pub fn extent2d(&self) -> Foundation::RECT {
        self.window.window_rect()
    }
}