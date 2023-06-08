use ash::vk;
extern crate bytemuck;
use bytemuck::offset_of;
use drowsed_math::linear::{FVec3, FVec2};
pub trait Vertex: Sized + core::marker::Copy + core::clone::Clone {
    fn binding_description() -> vk::VertexInputBindingDescription;
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription>;
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex2D {
    pub coords: FVec2,
}
impl Vertex for Vertex2D {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, coords) as u32,
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

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex3D {
    pub coords: FVec3,
}
impl Vertex for Vertex3D {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, coords) as u32,
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

#[repr(C, align(16))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex3DRGB {
    pub coords: FVec3,
    pub rgb: FVec3,
}
impl Vertex for Vertex3DRGB {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, coords) as u32,
        };
        let attr2 = vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, rgb) as u32,
        };
        let attributes = vec![attr, attr2];
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


#[repr(C, align(16))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex3DTexture {
    pub coords: FVec3,
    pub text_coords: FVec2,
}
impl Vertex for Vertex3DTexture {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, coords) as u32,
        };
        let attr2 = vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, text_coords) as u32,
        };
        let attributes = vec![attr, attr2];
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
///
/// Globaal Vertex Type im using for every type so that I dont need to change every
/// single value that uses a vertex, only this value.
/// 
pub type GlobalDebugVertex = Vertex3DTexture;