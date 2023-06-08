#![allow(unused)]
use winit::event::VirtualKeyCode;

/// Temporary File for testing user input.

pub struct InputKey {
    pub pressed: bool,
    // This value can be combined as a bitmask
    pub justunpressed: bool,
    // This value can be combined as a bitmask
    pub justpressed: bool,
}

impl InputKey {
    pub fn new() -> Self {
        Self { pressed: false, justunpressed: false, justpressed: false }
    }
    pub fn key_pressed(&mut self) {
        if self.pressed == false {
            self.justpressed = true;
        } else {
            self.justpressed = false;
        }
        self.pressed = true;
    }
    pub fn key_unpressed(&mut self) {
        if self.pressed == true {
            self.justunpressed = true;
        } else {
            self.justunpressed = false;
        }
        self.pressed = false;
    }
    pub fn poll(&mut self, state: winit::event::ElementState) {
        match state {
            winit::event::ElementState::Pressed => {
                self.key_pressed();
            }
            winit::event::ElementState::Released => {
                self.key_unpressed();
            }
        }
    }
}