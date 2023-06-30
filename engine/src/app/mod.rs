#![allow(unused)]

use ash::{vk, Entry};
use drowsed_math::linear::{FMat2, FVec2, FMat4};
use yum_mocha::{vk_obj::{pipelines::{self, graphics}, rendering, descriptors, device::{self, WindowOption}}, model::vertex::GlobalDebugVertex};
use std::sync::Arc;
pub struct App {
    pub device: Arc<device::Device>,
    pub window: WindowOption,
    pub renderer: rendering::Renderer,
    pub graphics: pipelines::graphics::GraphicsPipelines,
    pub descriptor_pool: descriptors::DescriptorPool,
    pub descriptor_layout: descriptors::DescriptorLayout,
    pub sets: Vec<vk::DescriptorSet>,
    pub layout: vk::PipelineLayout,
}
#[repr(C)]
#[derive(Clone, Copy) ]
pub struct PushData2D {
    pub rot_mat: FMat2,
    pub pos: FVec2,
    pub rotation: f32
}
#[repr(C)]
#[derive(Clone, Copy, Default) ]
pub struct PushData3D {
    pub transform: FMat4,
    pub model: FMat4,
}
impl App {
    pub fn new(entry: &Entry, window: WindowOption) -> Self {
        let mut device: Arc<device::Device>;
        match window {
            WindowOption::Winit(ref b) => device = device::Device::new(entry, b.clone()),
        }
        // let device = device::Device::new(entry, &window);

        let renderer = rendering::Renderer::new(device.clone(), window.clone());
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::ALL_GRAPHICS,
            size: std::mem::size_of::<PushData3D>() as u32,
            ..Default::default()
        };
        let descriptor_pool = descriptors::DescriptorPoolBuilder::new(device.clone())
        .set_max_sets(10) // Arbitrary Number
        .add_pool_size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 2)
        .add_pool_size(vk::DescriptorType::UNIFORM_BUFFER, 2)
        .build();

        let descriptor_layout = descriptors::DescriptorLayoutBuilder::new(device.clone())
        .add_binding(
            0, 
            vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 
            1, 
            vk::ShaderStageFlags::ALL_GRAPHICS
        )
        .build();

        let layouts = [descriptor_layout.layout, descriptor_layout.layout].as_ptr();
        let sets = descriptor_pool.allocate(device.clone(), layouts, 2);


        let layout_info = vk::PipelineLayoutCreateInfo {
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range,
            p_set_layouts: &descriptor_layout.layout,
            set_layout_count: 1,
            ..Default::default()
        };
        let layout = unsafe { device.device.create_pipeline_layout(&layout_info, None).unwrap() };
    
        let graphics_info = graphics::GraphicsPipelineInfo {
            vertex_entry: String::from("main\0"),
            fragment_entry: String::from("main\0"),
            vertex_filepath: String::from("./shaders/vertex3normaluv.vert.spv"),
            fragment_filepath: String::from("./shaders/vertex3normaluv.frag.spv"),
            layout: layout,
            renderpass: renderer.swapchain.renderpass,
            ..Default::default()
        };

        let graphics = graphics::GraphicsPipelines::new::<GlobalDebugVertex>(device.clone(), &graphics_info);
        Self { device: device, window, renderer, graphics, layout, descriptor_pool: descriptor_pool, sets: sets, descriptor_layout: descriptor_layout }
    }
}   