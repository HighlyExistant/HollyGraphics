#![allow(unused)]
use std::sync::{Arc, Mutex};

use crate::input::input_state::GlobalInputState;

pub enum CodeComponentResult {
    Update,
    Finish,
}
pub struct ProgramState {
    delta_time: f32,
    input: Arc<Mutex<GlobalInputState>>
}
pub trait CodeComponent {
    fn start(&self);
    fn update(&self, dt: f32) -> CodeComponentResult;
}