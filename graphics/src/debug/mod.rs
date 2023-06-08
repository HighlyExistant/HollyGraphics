#![allow(unused)]
use drowsed_math::{linear::FVec3, complex::quaternion::Quaternion};
use winit::event::KeyboardInput;

use crate::{input::{InputKey, self}, model::transform::{TransformQuaternion3D, Transform3D}};

/// 
/// This file is used for movement for debugging purposes
/// 

pub struct DebugMovement {
    pub user_key_w: input::InputKey,
    pub user_key_a: input::InputKey,
    pub user_key_s: input::InputKey,
    pub user_key_d: input::InputKey,
    pub user_key_e: input::InputKey,
    pub user_key_q: input::InputKey,
    pub user_key_up: input::InputKey,
    pub user_key_down: input::InputKey,
    pub user_key_left: input::InputKey,
    pub user_key_right: input::InputKey,
    pub transform: Transform3D,
}

impl DebugMovement {
    pub fn new() -> Self {
        Self { 
            user_key_w: InputKey::new(), user_key_a: InputKey::new(), user_key_s: InputKey::new(), user_key_d: InputKey::new(), user_key_e: InputKey::new(), user_key_q: InputKey::new(), user_key_up: InputKey::new(), user_key_down: InputKey::new(), user_key_left: InputKey::new(), user_key_right: InputKey::new(),
            transform: Transform3D { 
                translation: FVec3::from(0.0), 
                scale: FVec3::from(1.0), 
                rotation: FVec3::from(0.0) 
            }
        }
    }
    pub fn right(&self) -> FVec3 {
        Quaternion::<f32>::from_euler(self.transform.rotation) * FVec3::new(1.0, 0.0, 0.0)
    }
    pub fn up(&self) -> FVec3 {
        Quaternion::<f32>::from_euler(self.transform.rotation) * FVec3::new(0.0, 1.0, 0.0)
    }
    pub fn forward(&self) -> FVec3 {
        Quaternion::<f32>::from_euler(self.transform.rotation) * FVec3::new(0.0, 0.0, 1.0)
    }
    pub fn poll(&mut self, input: KeyboardInput) {
        match input.virtual_keycode {
            Some(key) => match key {
                winit::event::VirtualKeyCode::W => {
                    self.user_key_w.poll(input.state);
                }
                winit::event::VirtualKeyCode::A => {
                    self.user_key_a.poll(input.state);
                }
                winit::event::VirtualKeyCode::S => {
                    self.user_key_s.poll(input.state);
                }
                winit::event::VirtualKeyCode::D => {
                    self.user_key_d.poll(input.state);
                }
                winit::event::VirtualKeyCode::E => {
                    self.user_key_e.poll(input.state);
                }
                winit::event::VirtualKeyCode::Q => {
                    self.user_key_q.poll(input.state);
                }
                winit::event::VirtualKeyCode::Up => {
                    self.user_key_up.poll(input.state);
                }
                winit::event::VirtualKeyCode::Down => {
                    self.user_key_down.poll(input.state);
                }
                winit::event::VirtualKeyCode::Left => {
                    self.user_key_left.poll(input.state);
                }
                winit::event::VirtualKeyCode::Right => {
                    self.user_key_right.poll(input.state);
                }
                _ => {}
            }
            None => {}
        }
    }
    pub fn movement(&self, delta_time: f32) -> Transform3D {
        let mut transform = self.transform;
        let mut x = 0.0;
        let mut z = 0.0;
        if self.user_key_w.pressed {
            z = 1.0;
        }
        if self.user_key_s.pressed {
            z = -1.0;
        }
        if self.user_key_a.pressed {
            x = -1.0;
        }
        if self.user_key_d.pressed {
            x = 1.0;
        }
        if self.user_key_e.pressed {
            transform.translation.y -= 1.0 * delta_time;
        }
        if self.user_key_q.pressed {
            transform.translation.y += 1.0 * delta_time;
        }
        if self.user_key_up.pressed {
            transform.rotation.x += 1.0 * delta_time;
        }
        if self.user_key_down.pressed {
            transform.rotation.x -= 1.0 * delta_time;
        }
        if self.user_key_left.pressed {
            transform.rotation.y -= 1.0 * delta_time;
        }
        if self.user_key_right.pressed {
            transform.rotation.y += 1.0 * delta_time;
        }
        let right = self.right();
        let forward = self.forward();
        // println!("right: {:?}", right);

        let movement = right * x + forward * z;
        transform.translation += movement * delta_time;

        transform
    }
}