use std::{sync::Arc, collections::HashMap, rc::Rc, i128};

use ash::vk;
use drowsed_math::linear::{Transform, FMat4, FMat3, FVec3};
use yum_mocha::{vk_obj::{self, device::Device, rendering::mesh::{Renderable, VulkanIndexable, Vertex}}, camera::Camera};

use super::{scene::Scene, models::Mesh3D};
use crate::{vk_obj::{pipelines::graphics::GraphicsPipelines, buffer}, app::PushData3D};

pub struct RenderSystem<V: Vertex, I: VulkanIndexable> {
    /// 0: Renderable containing the vector of vertices and indices
    /// 1: Vertex Buffer for rendering
    /// 2: Index Buffer for rendering
    /// 3: Transform Matrix for scaling, rotations, translations and other linear transformations
    /// 4: Model Matrix for normals.
    objects: HashMap<i128, (Rc<dyn Renderable<V, I>>, Vec<buffer::raw::Buffer<V>>, buffer::raw::Buffer<I>, FMat4, FMat3)>
}

impl<V: Vertex, I: VulkanIndexable> RenderSystem<V, I> {
    
    pub fn push(&mut self, device: Arc<Device>, id: i128, renderable: Rc<dyn Renderable<V, I>>) {
        let (vertex, index) = renderable.get_buffers(device.clone());
        self.objects.insert(id, (renderable, vertex, index, FMat4::identity(1.0), FMat3::identity(1.0)));
    }
    pub fn render_all(&mut self, device: Arc<Device>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, scene: &Scene) {
        let camera = scene.get_camera();
        let projection = camera.projection * camera.view;
        // scene.objects()
        for (id, (renderable, vertices, indices, mut transform, mut model)) in &mut self.objects {
            let object = scene.get_object_by_id(*id).unwrap();
            transform = projection * object.transform().matrix4();
            model = object.transform().set_scaling(FVec3::from(1.0) / object.transform().scaling()).matrix3();
            
            let push_constants: PushData3D = PushData3D {
                transform: transform,
                model: model.into()
            };
            let data = unsafe { std::mem::transmute::<&PushData3D, &[u8; std::mem::size_of::<PushData3D>()]>(&push_constants) };
            unsafe { device.device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
    
            // renderable.transformations(device.clone(), command_buffer, layout, scene.get_camera());
            let (vertex, index) = renderable.bind_data(device.clone(), command_buffer);
            let buffers: Vec<_> = vertices.iter().map(|buffer| {
                buffer.buffer
            }).collect();
            unsafe { device.device.cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &[0]) };
            unsafe { device.device.cmd_bind_index_buffer(command_buffer, indices.buffer, 0, vk::IndexType::UINT32) };

            renderable.bind_data(device.clone(), command_buffer);
            unsafe { device.device.cmd_draw_indexed(command_buffer, index.unwrap(), 1, 0, 0, 0) };
        }
    }
}


impl<V: Vertex, I: VulkanIndexable> Default for RenderSystem<V, I> {
    fn default() -> Self { Self { objects: HashMap::new() } }
}