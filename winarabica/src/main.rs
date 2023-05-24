use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, GetMessageW, TranslateMessage, DispatchMessageW};
use windows_sys::{Win32::{UI::WindowsAndMessaging::*, Foundation::HMODULE, Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, FillRect, BRUSH_STYLE, CreateSolidBrush, EndPaint}}, core::PCWSTR};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;

pub mod implementation;
pub mod window;
fn main() {
    let window = window::Window::new();
    loop {
        window.get_messege(window::ProgramState::Nonblocking, None);
        if unsafe { (*window.window.state).keypressed } == 65 {
            println!("pressed");
        }
    }
}
