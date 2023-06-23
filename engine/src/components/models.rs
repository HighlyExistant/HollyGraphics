use ash::vk;
use drowsed_math::linear::{FVec3, FVec2};
use yum_mocha::model::model_loader::StandardModelData;
use yum_mocha::model::vertex::{Vertex2D, Vertex3DTexture, Vertex3DNormalUV};
use crate::vk_obj::{device, buffer};
use yum_mocha::vk_obj::rendering::mesh::{Mesh, Vertex, Renderable};
pub struct Mesh2D {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u32>,
}

impl Mesh<Vertex2D, u32> for Mesh2D {
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&self) -> Vec<Vertex2D> {
        self.vertices.clone()
    }
}

pub trait FromFBX {
    fn from_fbx(filepath: &str) -> Self::Output
        where Self: Sized;
    type Output;
}

#[derive(Debug, Clone)]
pub struct Mesh3D<T: Clone> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
}
impl<T: Clone> Mesh3D<T> {
    pub fn create(&self, device: std::sync::Arc<device::Device> ) -> (Vec<buffer::raw::Buffer<T>>, buffer::raw::Buffer<u32>) {
        let vertex_buffer = buffer::raw::Buffer::<T>::from_vec(device.clone(), 
            vk::BufferUsageFlags::VERTEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            self.vertices.clone()
        );
        let index_buffer = buffer::raw::Buffer::<u32>::from_vec(device.clone(), 
            vk::BufferUsageFlags::INDEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            self.indices.clone()
        );
        (vec![vertex_buffer], index_buffer)
    }
}
impl FromFBX for Mesh3D<Vertex3DTexture> {
    fn from_fbx(filepath: &str) -> Vec<Self> {
        let data = StandardModelData::new(filepath);

        let return_type = data.iter().filter_map(|model| {
            if model.vertices.is_empty() || model.indices.is_empty() {
                None
            } else {
                let modelvertices: Vec<Vertex3DTexture> = model.vertices.iter().map(|vertex| {
                    Vertex3DTexture {
                        coords: *vertex,
                        ..Default::default()                    
                    }
                }).collect();
                return Some(Mesh3D {
                    vertices: modelvertices.clone(),
                    indices: model.indices.clone(),
                });
            }
        }).collect();
        return_type
    }
    type Output = Vec<Self>;
}

impl FromFBX for Mesh3D<Vertex3DNormalUV> {
    fn from_fbx(filepath: &str) -> Vec<Self> {
        let data = StandardModelData::new(filepath);

        let return_type = data.iter().filter_map(|model| {
            if model.vertices.is_empty() || model.indices.is_empty() {
                None
            } else {
                let modelvertices: Vec<Vertex3DNormalUV> = model.vertices.iter().enumerate().map(|(i, vertex)| {
                    Vertex3DNormalUV {
                        pos: *vertex,
                        normal: model.normals[i],
                        ..Default::default()                    
                    }
                }).collect();
                return Some(Mesh3D {
                    vertices: modelvertices.clone(),
                    indices: model.indices.clone(),
                });
            }
        }).collect();
        return_type
    }
    type Output = Vec<Self>;
}

impl<T: Clone + Vertex> Mesh<T, u32> for Mesh3D<T> {
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&self) -> Vec<T> {
        self.vertices.clone()
    }
}

impl<T: Clone + Vertex> Renderable<T, u32> for Mesh3D<T> {
    fn bind_data(&self, device: std::sync::Arc<device::Device>, command_buffer: vk::CommandBuffer) -> (Option<u32>, Option<u32>) {
        (None, Some(self.indices.len() as u32))
    }
    fn get_buffers(&self, device: std::sync::Arc<device::Device>) -> (Vec<buffer::raw::Buffer<T>>, buffer::raw::Buffer<u32>) {
        self.create(device.clone())
    }
}