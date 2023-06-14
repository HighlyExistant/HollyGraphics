use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::Arc;
use drowsed_math::complex::quaternion::Quaternion;
use drowsed_math::linear::{FVec2, FMat2, FVec3, FMat4};

use crate::app::models;
use crate::collision::oriented::OrientedSquareCollider;
use crate::model::vertex::Vertex3DRGB;
use crate::model::{vertex};
use drowsed_math::linear::Transform2D;
use drowsed_math::linear::Transform3D;
use super::models::{Model2D, Model3D};
pub enum ColliderOptions {
    Oriented(OrientedSquareCollider),
    None
}
pub struct BasicObject2D {
    pub model: models::Model2D,
    pub transform: RefCell<Transform2D>,
    pub collider: ColliderOptions,
}
impl BasicObject2D {
    pub fn from_raw(vertices: Vec<vertex::Vertex2D>, indices: Vec<u32>, transform: RefCell<Transform2D>, collider: ColliderOptions) -> Self {
        let model = Model2D {
            vertices,
            indices
        };
        Self { model, transform, collider }
    }
    pub fn new(model: Model2D, transform: RefCell<Transform2D>, collider: ColliderOptions) -> Self {
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


pub struct BasicObject3D<T: Clone> {
    pub model: models::Model3D<T>,
    pub transform: RefCell<Transform3D>,
    pub collider: ColliderOptions,
}
impl<T: Clone> BasicObject3D<T> {
    pub fn from_raw(vertices: Vec<T>, indices: Vec<u32>, transform: RefCell<Transform3D>, collider: ColliderOptions) -> Self {
        let model = Model3D {
            vertices,
            indices
        };
        Self { model, transform, collider }
    }
    pub fn new(model: Model3D<T>, transform: RefCell<Transform3D>, collider: ColliderOptions) -> Self {
        Self { model, transform, collider }
    }
    pub fn rotation(&mut self, rotation: FVec3) {
        let mut interior = self.transform.borrow_mut();
        interior.rotation = rotation;
    }
    pub fn translate(&mut self, translation: FVec3) {
        let mut interior = self.transform.borrow_mut();
        interior.translation = translation;
    }
    pub fn scale(&mut self, scale: FVec3) {
        let mut interior = self.transform.borrow_mut();
        interior.scale = scale;
    }
    pub fn matrix_4(&self) -> FMat4 {
        let interior = self.transform.borrow();
        interior.mat4()
    }
}