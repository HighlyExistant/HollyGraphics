// use ash::vk;

// use crate::{vk_obj::{pipelines::graphics::GraphicsPipelines, device::{Device, self}, buffer}, camera::Camera};
// use std::{sync::Arc, rc::Rc, cell::RefCell};
// use super::mesh::{Renderable, Vertex, VulkanIndexable};

// pub struct RenderSystem<V: Vertex, I: VulkanIndexable> {
//     device: Arc<device::Device>,
//     queue: Vec<*const dyn Renderable<V, I>>,
//     buffers: Vec<(Vec<buffer::raw::Buffer<V>>, buffer::raw::Buffer<I>)>
// }

// impl<V: Vertex, I: VulkanIndexable> RenderSystem<V, I> {
//     pub fn new(queue: Vec<*const dyn Renderable<V, I>>, device: Arc<Device>) -> Self {
//         let buffers = queue.iter().map(|renderable| {
//             unsafe { renderable.as_ref().unwrap().get_buffers(device.clone()) }
//         }).collect();
//         Self { queue, buffers, device }
//     }
//     pub fn push(&mut self, renderable: *const dyn Renderable<V, I>) {
//         unsafe { self.buffers.push((*renderable).get_buffers(self.device.clone())) };
//         self.queue.push(renderable);
//     }
//     pub fn render_all(&self, device: Arc<Device>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, camera: &Camera) {
//         for (i, entry) in self.queue.iter().enumerate() {
//             if let Some(a) = unsafe { (*entry).as_ref() } {
//                 a.transformations(device.clone(), command_buffer, layout, camera);
//                 let (vertex, index) = a.bind_data(device.clone(), command_buffer);
//                 let buffers: Vec<_> = self.buffers[i].0.iter().map(|buffer| {
//                     buffer.buffer
//                 }).collect();
//                 unsafe { device.device.cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &[0]) };
//                 unsafe { device.device.cmd_bind_index_buffer(command_buffer, self.buffers[i].1.buffer, 0, vk::IndexType::UINT32) };
    
//                 a.bind_data(device.clone(), command_buffer);
//                 unsafe { device.device.cmd_draw_indexed(command_buffer, index.unwrap(), 1, 0, 0, 0) };
//             }
            
//         }
//     }
// }