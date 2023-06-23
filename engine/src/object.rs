use std::{rc::Rc, cell::RefCell};

use ash::vk;
use drowsed_math::{linear::{TransformQuaternion3D, FVec3, Transform}, complex::quaternion::Quaternion};
use yum_mocha::{model::vertex::GlobalDebugVertex, vk_obj::rendering::mesh::{Renderable, Vertex}, camera::Camera};
use yum_mocha::vk_obj::rendering::mesh::Mesh;

use yum_mocha::vk_obj::buffer;

use crate::{components::models::{FromFBX, Mesh3D}, app::PushData3D};
pub struct BasicObject3D {
    mesh: Mesh3D<GlobalDebugVertex>,
    pub transform: TransformQuaternion3D,
    // pub pushconstants: Box<PushData3D>,
}

impl BasicObject3D {
    pub fn get_buffers(&self, device: std::sync::Arc<crate::vk_obj::device::Device>) -> (Vec<buffer::raw::Buffer<GlobalDebugVertex>>, buffer::raw::Buffer<u32>) {
        self.mesh.create(device)
    }
}
impl FromFBX for BasicObject3D {
    fn from_fbx(filepath: &str) -> Vec<Rc<Self>>
            where Self: Sized {
        let meshes = Mesh3D::<GlobalDebugVertex>::from_fbx(filepath);

        meshes.iter().map(|mesh|{
            Rc::new(Self {
                mesh: mesh.clone(),
                transform: TransformQuaternion3D { translation: FVec3::default(), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::default()) },
                // pushconstants: Box::default(),

            })
        }).collect()
    }
    type Output = Vec<Rc<Self>>;
}
impl Mesh<GlobalDebugVertex, u32> for BasicObject3D {
    fn indices(&self) -> Vec<u32> {
        self.mesh.indices.clone()
    }
    fn vertices(&self) -> Vec<GlobalDebugVertex> {
        self.mesh.vertices.clone()
    }
}

impl Renderable<GlobalDebugVertex, u32> for BasicObject3D {
    fn bind_data(&self, device: std::sync::Arc<yum_mocha::vk_obj::device::Device>, command_buffer: ash::vk::CommandBuffer) -> (Option<u32>, Option<u32>) {
        (None, Some(self.mesh.indices.len() as u32))
    }

    // fn transformations(&self, device: std::sync::Arc<yum_mocha::vk_obj::device::Device>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, camera: &Camera) {
    //     let model = self.transform.matrix4();
    //     let normal_mat = self.transform.set_scaling(FVec3::from(1.0) / self.transform.scale).matrix3();
    //     let projection = camera.projection * camera.view;
    //     let projection_mat = projection * model;
    //     let push_constants: PushData3D = PushData3D {
    //         transform: projection_mat,
    //         model: normal_mat.into()
    //     };
        
    //     let data = unsafe { std::mem::transmute::<&PushData3D, &[u8; std::mem::size_of::<PushData3D>()]>(&push_constants) };
    //     unsafe { device.device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
    // }

    fn get_buffers(&self, device: std::sync::Arc<yum_mocha::vk_obj::device::Device>) -> (Vec<buffer::raw::Buffer<GlobalDebugVertex>>, buffer::raw::Buffer<u32>) {
        self.mesh.create(device.clone())
    }
}

// impl CodeSegment for BasicObject3D {
//     fn start(&mut self) {
        
//     }
//     fn update(&mut self, state: &super::codeclass::ProgramState) {

//         let input = state.input;
//         if input.is_just_pressed(winit::event::VirtualKeyCode::U) {
//             self.transform.translation.z += 1.0 * state.delta_time;
//         }
//         if input.is_just_pressed(winit::event::VirtualKeyCode::J) {
//             self.transform.translation.z -= 1.0 * state.delta_time;
//         }
//         if input.is_just_pressed(winit::event::VirtualKeyCode::H) {
//             self.transform.translation.x -= 1.0 * state.delta_time;
//         }
//         if input.is_just_pressed(winit::event::VirtualKeyCode::K) {
//             self.transform.translation.x += 1.0 * state.delta_time;
//         }
//     }
// }