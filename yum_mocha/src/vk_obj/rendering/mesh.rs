use ash::vk;
use num_traits;
use std::sync::Arc;
use crate::{vk_obj::{buffer::{raw, self}, device::{self, Device}}, camera::Camera};
pub trait VulkanIndexable: num_traits::Num + core::clone::Clone + num_traits::AsPrimitive<u8> + num_traits::AsPrimitive<u16> + num_traits::AsPrimitive<u32> + num_traits::AsPrimitive<usize> + core::clone::Clone {}
pub trait Vertex: Sized + Copy + Clone {
    fn binding_description() -> vk::VertexInputBindingDescription;
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription>;
}

impl VulkanIndexable for u8 {}
impl VulkanIndexable for u16 {}
impl VulkanIndexable for u32 {}
pub trait Mesh<V: Vertex, I: VulkanIndexable> {
    fn vertices(&self) -> Vec<V>;
    fn indices(&self) -> Vec<I>;
}
pub trait Renderable<V: Vertex, I: VulkanIndexable> {
    // should return vertex count
    fn bind_data(&self, device: Arc<Device>, command_buffer: vk::CommandBuffer) -> (Option<u32>, Option<u32>);
    // fn transformations(&self, device: Arc<Device>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, camera: &Camera);
    fn get_buffers(&self, device: Arc<Device>) -> (Vec<buffer::raw::Buffer<V>>, buffer::raw::Buffer<I>);
}
