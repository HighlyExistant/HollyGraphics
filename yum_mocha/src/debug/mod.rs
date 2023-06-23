#![allow(unused)]
use drowsed_math::{linear::FVec3, complex::quaternion::Quaternion};
use winit::event::KeyboardInput;
use std::sync::{Arc, Mutex};
use drowsed_math::linear::Transform3D;

use crate::input::{input_key::{self, InputKey}, input_state::GlobalInputState};
/// 
/// This file is used for movement for debugging purposes
/// 

pub struct DebugMovement {
    pub transform: Transform3D,
}

impl DebugMovement {
    pub fn new(user_input: Arc<Mutex<GlobalInputState>>) -> Self {
        Self { 
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
    pub fn movement(&self, input: Arc<Mutex<GlobalInputState>>, delta_time: f32) -> Transform3D {
        let mut transform = self.transform;
        let inputlock = input.lock().unwrap();
        let mut x = 0.0;
        let mut z = 0.0;
        if inputlock.is_pressed(winit::event::VirtualKeyCode::W) {
            z = 2.5;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::S) {
            z = -2.5;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::A) {
            x = -2.5;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::D) {
            x = 2.5;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::E) {
            transform.translation.y -= 2.5 * delta_time;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::Q) {
            transform.translation.y += 2.5 * delta_time;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::Up) {
            transform.rotation.x += 2.5 * delta_time;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::Down) {
            transform.rotation.x -= 2.5 * delta_time;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::Left) {
            transform.rotation.y -= 2.5 * delta_time;
        }
        if inputlock.is_pressed(winit::event::VirtualKeyCode::Right) {
            transform.rotation.y += 2.5 * delta_time;
        }
        let right = self.right();
        let forward = self.forward();
        // println!("right: {:?}", right);

        let movement = right * x + forward * z;
        transform.translation += movement * delta_time;

        transform
    }
}