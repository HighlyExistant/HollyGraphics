use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::Arc;

use puraexpr::linear::f32::{FVec2, FMat2};

use crate::app::models;
use crate::collision::oriented::OrientedSquareCollider;
use crate::holly_types::{transform, vertex};

use super::PushData;
use super::models::Model2D;
pub enum ColliderOptions {
    Oriented(OrientedSquareCollider),
    None
}
pub struct BasicObject2D {
    pub model: models::Model2D,
    pub transform: RefCell<transform::Transform2D>,
    pub collider: ColliderOptions,
}
impl BasicObject2D {
    pub fn new(vertices: Vec<vertex::Vertex2D>, indices: Vec<u32>, transform: RefCell<transform::Transform2D>, collider: ColliderOptions) -> Self {
        let model = Model2D {
            vertices,
            indices
        };
        Self { model, transform, collider }
    }
    pub fn rotation(&mut self, rotation: f32) {
        let mut interior = self.transform.borrow_mut();
        interior.rotation = rotation;
    }
    pub fn translate(&mut self, translation: FVec2) {
        let mut interior = self.transform.borrow_mut();
        interior.translation = translation;
    }
    pub fn scale(&mut self, scale: FVec2) {
        let mut interior = self.transform.borrow_mut();
        interior.scale = scale;
    }
    pub fn matrix_2(&mut self) -> FMat2 {
        let mut interior = self.transform.borrow_mut();
        interior.mat2()
    }
}