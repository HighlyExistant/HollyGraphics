use std::{sync::Arc, collections::HashMap, rc::Rc, i128, marker::PhantomData};

use ash::vk;
use drowsed_math::{Transform, FMat4, FMat3};
use yum_mocha::vk_obj::{device::ReplacingDevice, rendering::mesh::{VulkanIndexable, Vertex}};
use crate::{vk_obj::buffer, motor::{device_manager::PushData3D, scene_manager::SceneManager}};

use super::models::Renderable;

pub struct RenderSystem<V: Vertex, I: VulkanIndexable, T: Transform> {
    /// 0: Renderable containing the vector of vertices and indices
    /// 1: Vertex Buffer for rendering
    /// 2: Index Buffer for rendering
    /// 3: Transform Matrix for scaling, rotations, translations and other linear transformations
    /// 4: Model Matrix for normals.
    objects: HashMap<i128, (Rc<dyn Renderable<V, I>>, Vec<buffer::raw::Buffer<V>>, buffer::raw::Buffer<I>, FMat4, FMat3)>,
    phantom: PhantomData<T>
}

impl<V: Vertex, I: VulkanIndexable, T: Transform> RenderSystem<V, I, T> {
    
    pub fn push(&mut self, device: Arc<ReplacingDevice>, id: i128, renderable: Rc<dyn Renderable<V, I>>) {
        let (vertex, index) = renderable.get_buffers(device.clone());
        self.objects.insert(id, (renderable, vertex, index, FMat4::identity(1.0), FMat3::identity(1.0)));
    }
    pub fn render(&mut self, device: Arc<ReplacingDevice>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, scenemanager: &SceneManager<T>) {
        let scene = scenemanager.get_selected_scene();
        let camera = scene.get_camera();
        let projection = camera.projection * camera.view;
        // scene.objects()
        for (id, (renderable, vertices, indices, mut transform, mut model)) in &mut self.objects {
            let object = scene.get_object_by_id(*id).unwrap();
            transform = projection * object.transform().matrix4();
            model = object.transform().normal_matrix();
            
            let push_constants: PushData3D = PushData3D  {
                transform: transform,
                model: model.into()
            };
            let data = unsafe { std::mem::transmute::<&PushData3D, &[u8; std::mem::size_of::<PushData3D>()]>(&push_constants) };
            unsafe { device.device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
    
            // renderable.transformations(device.clone(), command_buffer, layout, scene.get_camera());
            let (_, index) = renderable.bind_data(device.clone(), command_buffer);
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

impl<V: Vertex, I: VulkanIndexable, T: Transform> Default for RenderSystem<V, I, T> {
    fn default() -> Self { Self { objects: HashMap::new(), phantom: PhantomData::default() } }
}