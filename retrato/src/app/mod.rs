#![allow(unused)]
use ash::{Entry, vk::{self, Extent2D}};
use puraexpr::linear::f32::{FMat2, FVec2};
pub mod models;
pub mod basic;
use crate::{device::{self, Device}, rendering, pipelines::{self, graphics}, holly_types::{vertex::Vertex2D}, buffer::{allocator, self}};
use std::sync::Arc;
pub struct App {
    pub device: Arc<device::Device>,
    pub window: WindowOption,
    pub renderer: rendering::Renderer,
    pub graphics: pipelines::graphics::GraphicsPipeline,
    pub layout: vk::PipelineLayout,
    pub allocator: allocator::BufferAllocator,
}
#[repr(C)]
#[derive(Clone, Copy) ]
pub struct PushData {
    pub rot_mat: FMat2,
    pub pos: FVec2,
    pub rotation: f32
}
#[derive(Clone)]
pub enum WindowOption {
    Winit(Arc<winit::window::Window>),
    Winarabica(Arc<winarabica::window::Window>)
}
impl WindowOption {
    pub fn get_extent2d(&self) -> ash::vk::Extent2D {
        match self {
            WindowOption::Winarabica(a) => {
                let rect = a.extent2d();
                return Extent2D {
                    width: (rect.right - rect.left) as u32,
                    height: (rect.bottom - rect.top) as u32,
                };
            }
            WindowOption::Winit(b) => {
                let inner = b.inner_size();
                return Extent2D {
                    width: inner.width,
                    height: inner.height,
                };
            }
        };
    }
}
impl App {
    pub fn new(entry: &Entry, window: WindowOption) -> Self {
        let mut device: Arc<Device>;
        match window {
            WindowOption::Winarabica(ref a) => device = device::Device::unstable_from_winarabica(entry, &a),
            WindowOption::Winit(ref b) => device = device::Device::new(entry, b.clone()),
        }
        // let device = device::Device::new(entry, &window);

        let renderer = rendering::Renderer::new(&device, window.clone());
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::ALL_GRAPHICS,
            size: std::mem::size_of::<PushData>() as u32,
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