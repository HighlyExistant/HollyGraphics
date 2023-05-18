use ash::{Entry, vk};
pub mod models;
use crate::{device, rendering, pipelines::{self, graphics}, holly_types::{vertex::Vertex2D}, buffer::{allocator, self}};
use std::sync::Arc;
pub struct App {
    pub device: Arc<device::Device>,
    pub window: Arc<winit::window::Window>,
    pub renderer: rendering::Renderer,
    pub graphics: pipelines::graphics::GraphicsPipeline,
    pub layout: vk::PipelineLayout,
    pub allocator: allocator::BufferAllocator,
}

impl App {
    pub fn new(entry: &Entry, window: Arc<winit::window::Window>) -> Self {
        let device = device::Device::new(entry, &window);

        let renderer = rendering::Renderer::new(&device, window.clone());
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::ALL_GRAPHICS,
            size: std::mem::size_of::<Vertex2D>() as u32,
            ..Default::default()
        };
        let layout_info = vk::PipelineLayoutCreateInfo {
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range,
            ..Default::default()
        };
        let layout = unsafe { device.device.create_pipeline_layout(&layout_info, None).unwrap() };
    
        let graphics_info = graphics::GraphicsPipelineInfo {
            culling: vk::CullModeFlags::NONE,
            vertex_entry: String::from("main\0"),
            fragment_entry: String::from("main\0"),
            vertex_filepath: String::from("./shaders/vertex.vert.spv"),
            fragment_filepath: String::from("./shaders/vertex.frag.spv"),
            layout: layout,
            renderpass: renderer.swapchain.renderpass,
            ..Default::default()
        };
        let allocator = allocator::BufferAllocator::new(device.clone());
        let graphics = graphics::GraphicsPipeline::new::<Vertex2D>(device.clone(), &graphics_info);
        Self { device: device, window, renderer, graphics, layout, allocator }
    }
}   