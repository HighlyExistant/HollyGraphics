use crate::holly_types::{vertex, self};
pub struct Model2D {
    pub vertices: Vec<vertex::Vertex2D>,
    pub indices: Vec<u32>,
}

impl holly_types::model::Model<vertex::Vertex2D, u32> for Model2D {
    fn indices(&mut self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&mut self) -> Vec<vertex::Vertex2D> {
        self.vertices.clone()
    }
}