use raw_window_handle::{RawDisplayHandle, WindowsDisplayHandle};
use windows_sys::{Win32::{UI::WindowsAndMessaging::{*, self}, Foundation::HMODULE, Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, FillRect, BRUSH_STYLE, CreateSolidBrush, EndPaint}}, core::PCWSTR};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use std::{sync::{Arc, Mutex}, ffi::c_void};

use self::state::State;
mod state;

pub(crate) struct Window {
    pub(crate) window: HWND,
    pub(crate) state: *mut State,
    pub(crate) callback: Option<Box<dyn FnMut(*const c_void) -> bool + 'static>>,
}

impl Window {
    pub fn new() -> Self {
        let (window, state) = unsafe { Self::create_window() };
        // let mutex_state = Mutex::new(state);
        let s = Self { window, state, callback: None };
        unsafe { s.show() };
        s
    }
    unsafe fn create_window() -> (HWND, *mut State) {
        let class_name = wchar::wch!("Window Class Name\0");
        let window_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: Self::get_instance(),
            hIcon: 0,
            hCursor: 0,
            hbrBackground: 0,
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: 0,
        };
        RegisterClassExW(&window_class);
        let mut program_data = Box::into_raw(Box::new(state::State::new()));
        // (*program_data).keypressed = 10;
        // let program_data = state::State::new();

        let window = CreateWindowExW(0, 
            class_name.as_ptr(), 
            class_name.as_ptr(), 
            WS_OVERLAPPEDWINDOW, 
            0, 
            0, 
            CW_USEDEFAULT, 
            CW_USEDEFAULT, 
            0, 
            0, 
            Self::get_instance(), 
            program_data as _
        );
        if window == 0 {
            panic!("Change this panic to return an option to make it more idiomatic to rust");
        }
        (window, program_data)
    }

    unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        
        let create: *mut CREATESTRUCTW;
        let program = if msg == WM_CREATE {
            create = std::mem::transmute::<isize, *mut CREATESTRUCTW>(lparam);
            let retrieved = std::mem::transmute::<*mut c_void, *mut State>((*create).lpCreateParams);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, retrieved as *const _ as isize);
            retrieved
        } else {
            let retrieved = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            (retrieved as *mut State)
        };

        match msg {
            WM_PAINT => {
                let mut paint = PAINTSTRUCT {
                    hdc: 0,
                    fErase: 0,
                    rcPaint: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                    fRestore: 0,
                    fIncUpdate: 0,
                    rgbReserved: [0u8; 32],
                };
                let hdc = BeginPaint(hwnd, &mut paint);
                
                EndPaint(hwnd, &paint);
                return 0;
            }
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                println!("keypressed: {}", wparam as u8 as char);
                println!("data: {}", (*program).keypressed);
                (*program).keypressed = wparam;
            }
            WM_CLOSE => {
                DestroyWindow(hwnd);
            }
            _ => {}
        }
        // println!("loop");
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
    pub unsafe fn get_state(&self) -> isize {
        let ptr =GetWindowLongPtrW(self.window, GWLP_USERDATA);
        ptr
    }
    fn get_instance() -> HMODULE {
        extern "C" {
            static __ImageBase: IMAGE_DOS_HEADER;
        }
        unsafe { &__ImageBase as *const _ as _ }
    }
    
    fn rgb(r: u8, g: u8, b: u8) -> u32 {
        let rgb = (r as u32) | ((g as u32) << 8) | ((b as u32) << 16); 
        rgb
    }
    unsafe fn show(&self) {
        ShowWindow(self.window, SW_SHOW);
    }
    pub unsafe fn get_messege_nonblock<F>(&self, user: Option<F>)
        where F: FnOnce(usize) {
        let mut get = (self.state);
        

        if PeekMessageW(&mut (*get).msg, self.window, 0, 0, PM_REMOVE) != 0 {
            
            TranslateMessage(&(*get).msg);
            if let Some(f) = user {
                f((*get).keypressed);
            }
            DispatchMessageW(&(*get).msg);
            
        } else {
            
        }
    }
    pub unsafe fn get_messege_block<F>(&self, user: Option<F>)
        where F: FnOnce(usize) {
        let mut get = (self.state);
        

        if GetMessageW(&mut (*get).msg, self.window, 0, 0) != 0 {
            
            TranslateMessage(&(*get).msg);
            
            DispatchMessageW(&(*get).msg);
            
        } else {
            
        }
    }
    #[inline]
    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
    }
    pub fn get_hwnd(&self) -> isize {
        self.window
    }
    pub fn hinstance(&self) -> isize {
        // Only works in 64 bits
        return unsafe { WindowsAndMessaging::GetWindowLongPtrW(self.window, GWLP_HINSTANCE) };
    }
    pub fn window_rect(&self) -> RECT {
        let mut rect: RECT = unsafe { std::mem::zeroed() };
        unsafe { GetClientRect(self.get_hwnd(), &mut rect) };
        rect
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let _boxed = unsafe { Box::from_raw(self.state) };
    }
}