use ash::vk;
use crate::pipelines;
use crate::device;
pub struct GraphicsPipeline {
    pub pipeline: vk::Pipeline,
    vertex: vk::ShaderModule,
    fragment: vk::ShaderModule,
    device: std::sync::Arc<device::Device>,
}
#[derive(Default)]
pub struct GraphicsPipelineInfo
{
    pub vertex_filepath: String,
    pub vertex_entry: String,
    pub fragment_filepath: String,
    pub fragment_entry: String,
    pub culling: vk::CullModeFlags,
    pub layout: vk::PipelineLayout,
    pub renderpass: vk::RenderPass,
    pub subpass: u32,
}

impl GraphicsPipeline {
    pub fn new<T>(device: std::sync::Arc<device::Device>, info: &GraphicsPipelineInfo) -> Self
    where T: crate::holly_types::Vertex {
        let vertex_module = pipelines::create_shader_module(&device.device, info.vertex_filepath.clone());
        let fragment_module = pipelines::create_shader_module(&device.device, info.fragment_filepath.clone());

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_module,
                p_name: info.vertex_entry.as_bytes().as_ptr() as *const i8,
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_module,
                p_name: info.fragment_entry.as_bytes().as_ptr() as *const i8,
                ..Default::default()
            }
        ];
        let binding_description = T::binding_description();
        let attribute_description = T::attribute_description();

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            vertex_attribute_description_count: attribute_description.len() as u32,
            p_vertex_attribute_descriptions: attribute_description.as_ptr(),
            vertex_binding_description_count: 1,
            p_vertex_binding_descriptions: &binding_description,
            ..Default::default()
        };

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_STRIP,
            primitive_restart_enable: 0, // VK_FALSE
            ..Default::default()
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };

        let raserization_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: 0,  // VK_FALSE
            rasterizer_discard_enable: 0,  // VK_FALSE
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: info.culling,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: 0, // VK_FALSE
            ..Default::default()
        };
        let multisample_state = vk::PipelineMultisampleStateCreateInfo {
            sample_shading_enable: 0, // VK_FALSE
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            min_sample_shading: 1.0,
            alpha_to_coverage_enable: 0, // VK_FALSE
            alpha_to_one_enable: 0, // VK_FALSE
            ..Default::default()
        };
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1, // VK_TRUE
            depth_write_enable: 1, // VK_TRUE
            depth_compare_op: vk::CompareOp::LESS,
            depth_bounds_test_enable: 0, // VK_FALSE
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
            stencil_test_enable: 0, // VK_FALSE
            ..Default::default()
        };

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A,
            blend_enable: 1, // VK_TRUE
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            ..Default::default()
        };

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: 0, // VK_FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: &color_blend_attachment,
            ..Default::default()
        };
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_state = vk::PipelineDynamicStateCreateInfo {
            p_dynamic_states: dynamic_states.as_ptr(),
            dynamic_state_count: dynamic_states.len() as u32,
            ..Default::default()
        };

        let create_info = vk::GraphicsPipelineCreateInfo {
            stage_count: 2, // fragment and vertex shader
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_state,
            p_input_assembly_state: &input_assembly_state,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &raserization_state,
            p_multisample_state: &multisample_state,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: &dynamic_state_state,
            layout: info.layout,
            render_pass: info.renderpass,
            subpass: info.subpass,
            base_pipeline_index: -1,
            ..Default::default()
        };

        let pipeline = unsafe { device.device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None).unwrap()[0] };
        Self { pipeline, vertex: vertex_module, fragment: fragment_module, device }
    }
}