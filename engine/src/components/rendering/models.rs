use std::sync::Arc;

use ash::vk;
use yum_mocha::model::model_loader::StandardModelData;
use yum_mocha::model::vertex::{Vertex3DTexture, Vertex3DNormalUV};
use yum_mocha::vk_obj::device::ReplacingDevice;
use yum_mocha::vk_obj::rendering::mesh::{Vertex, VulkanIndexable, Mesh};
use crate::vk_obj::buffer;

pub trait Renderable<V: Vertex, I: VulkanIndexable> {
    // should return vertex count
    fn bind_data(&self, device: Arc<ReplacingDevice>, command_buffer: vk::CommandBuffer) -> (Option<u32>, Option<u32>);
    // fn transformations(&self, device: Arc<Device>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, camera: &Camera);
    fn get_buffers(&self, device: Arc<ReplacingDevice>) -> (Vec<buffer::raw::Buffer<V>>, buffer::raw::Buffer<I>);
}

pub trait FromFBX {
    fn from_fbx(filepath: &str) -> Self::Output
        where Self: Sized;
    type Output;
}
#[derive(Debug, Clone)]
pub struct Model<T: Clone> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
}
impl<T: Clone> Model<T> {
    pub fn create(&self, device: std::sync::Arc<ReplacingDevice> ) -> (Vec<buffer::raw::Buffer<T>>, buffer::raw::Buffer<u32>) {
        let vertex_buffer = buffer::raw::Buffer::<T>::from_vec(device.clone(), 
            vk::BufferUsageFlags::VERTEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &self.vertices
        );
        let index_buffer = buffer::raw::Buffer::<u32>::from_vec(device.clone(), 
            vk::BufferUsageFlags::INDEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &self.indices
        );
        (vec![vertex_buffer], index_buffer)
    }
}
impl FromFBX for Model<Vertex3DTexture> {
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
                return Some(Model {
                    vertices: modelvertices.clone(),
                    indices: model.indices.clone(),
                });
            }
        }).collect();
        return_type
    }
    type Output = Vec<Self>;
}

impl FromFBX for Model<Vertex3DNormalUV> {
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
                return Some(Model {
                    vertices: modelvertices.clone(),
                    indices: model.indices.clone(),
                });
            }
        }).collect();
        return_type
    }
    type Output = Vec<Self>;
}

impl<T: Clone + Vertex> Mesh<T, u32> for Model<T> {
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&self) -> Vec<T> {
        self.vertices.clone()
    }
}

impl<T: Clone + Vertex> Renderable<T, u32> for Model<T> {
    fn bind_data(&self, _device: std::sync::Arc<ReplacingDevice>, _command_buffer: vk::CommandBuffer) -> (Option<u32>, Option<u32>) {
        (None, Some(self.indices.len() as u32))
    }
    fn get_buffers(&self, device: std::sync::Arc<ReplacingDevice>) -> (Vec<buffer::raw::Buffer<T>>, buffer::raw::Buffer<u32>) {
        self.create(device.clone())
    }
}