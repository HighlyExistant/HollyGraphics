#![allow(unused)]
use ash::{Entry, vk::{self, Extent2D}};
use drowsed_math::linear::{FMat2, FVec2, FMat4};
pub mod models;
pub mod basic;
use crate::{device::{self, Device}, rendering, pipelines::{self, graphics}, holly_types::{vertex::{Vertex2D, Vertex3D, Vertex3DRGB}}};
use std::sync::Arc;
pub struct App {
    pub device: Arc<device::Device>,
    pub window: WindowOption,
    pub renderer: rendering::Renderer,
    pub graphics: pipelines::graphics::GraphicsPipeline,
    pub layout: vk::PipelineLayout,
}
#[repr(C)]
#[derive(Clone, Copy) ]
pub struct PushData2D {
    pub rot_mat: FMat2,
    pub pos: FVec2,
    pub rotation: f32
}
pub struct PushData3D {
    pub rot_mat: FMat4,
}
#[derive(Clone)]
pub enum WindowOption {
    Winit(Arc<winit::window::Window>),
}
impl WindowOption {
    pub fn get_extent2d(&self) -> ash::vk::Extent2D {
        match self {
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
            WindowOption::Winit(ref b) => device = device::Device::new(entry, b.clone()),
        }
        // let device = device::Device::new(entry, &window);

        let renderer = rendering::Renderer::new(&device, window.clone());
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::ALL_GRAPHICS,
            size: std::mem::size_of::<PushData3D>() as u32,
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
            vertex_filepath: String::from("./shaders/vertex3rgb.vert.spv"),
            fragment_filepath: String::from("./shaders/vertex3rgb.frag.spv"),
            layout: layout,
            renderpass: renderer.swapchain.renderpass,
            ..Default::default()
        };
        let graphics = graphics::GraphicsPipeline::new::<Vertex3DRGB>(device.clone(), &graphics_info);
        Self { device: device, window, renderer, graphics, layout }
    }
}   