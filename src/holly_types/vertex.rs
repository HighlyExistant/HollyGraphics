use ash::vk;
extern crate bytemuck;
use bytemuck::offset_of;
pub trait Vertex: Sized + core::marker::Copy {
    fn binding_description() -> vk::VertexInputBindingDescription;
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription>;
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex2D {
    pub coords: [f32; 2],
}
impl Vertex for Vertex2D {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex2D, coords) as u32,
        };
        let attributes = vec![attr];
        attributes
    }
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }
}