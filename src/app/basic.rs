use crate::app::models;
use crate::holly_types::{transform, vertex};

use super::models::Model2D;
pub struct BasicObject2D {
    pub model: models::Model2D,
    pub transform: transform::Transform2D
}
impl BasicObject2D {
    pub fn new(vertices: Vec<vertex::Vertex2D>,indices: Vec<u32>, transform: transform::Transform2D) -> Self {
        let model = Model2D {
            vertices,
            indices
        };
        Self { model, transform }
    }
}