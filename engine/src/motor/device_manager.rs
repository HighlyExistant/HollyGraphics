use ash::{vk, Entry};
use drowsed_math::{FMat4, FVec2, FMat2};
use yum_mocha::{vk_obj::{descriptors, self, pipelines::graphics, device::{WindowOption, LogicalDevice, QueueFamilyIndices, LogicalDeviceBuilder}, rendering::Renderer}, model::vertex::GlobalDebugVertex};

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

pub struct DeviceManager {
    pub device: std::sync::Arc<LogicalDevice>,
    pub window: WindowOption,
    pub renderer: vk_obj::rendering::Renderer,
    pub graphics: graphics::GraphicsPipelines,
    pub descriptor_pool: descriptors::DescriptorPool,
    pub descriptor_layout: descriptors::DescriptorLayout,
    pub sets: Vec<vk::DescriptorSet>,
    pub layout: vk::PipelineLayout,
}

impl DeviceManager {
    pub fn new(entry: &Entry, window: WindowOption) -> Self {
        let device: std::sync::Arc<LogicalDevice>;
        match window {
            WindowOption::Winit(ref b) => device = std::sync::Arc::new(LogicalDeviceBuilder::new()
            .set_window(b.clone())
            .check_queue_support(vk::QueueFlags::GRAPHICS)
            .check_queue_support(vk::QueueFlags::COMPUTE)
            .add_swapchain_extension()
            .build(entry, |prop, physical_device, surface, funcs|{
                let mut indices = QueueFamilyIndices::default();
                let mut queueinfo: Vec<(u32, u32, vk::CommandPoolCreateFlags)> = vec![];
                let mut i = 0;
                for family in prop {
                    if family.queue_count > 0 && (family.queue_flags & vk::QueueFlags::GRAPHICS) == vk::QueueFlags::GRAPHICS {
                        indices.graphics = Some(i);
                    }
                    let present_support = unsafe { ash::extensions::khr::Surface::get_physical_device_surface_support(funcs, *physical_device, i, surface.unwrap()).unwrap() };
                    if family.queue_count > 0 && present_support {
                        indices.surface = Some(i);
                    }
                    if indices.graphics.is_some() && indices.surface.is_some() {
                        queueinfo.push((i, 1, vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER));
                        break;
                    }
                    i += 1;
                }
                queueinfo
            })),
        }
        let renderer = Renderer::new(device.clone(), window.clone());
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
        Self { device, window, renderer, graphics, descriptor_pool, descriptor_layout, sets, layout }
    }
}