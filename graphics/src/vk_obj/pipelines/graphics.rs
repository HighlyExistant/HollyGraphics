#![allow(unused)]
use ash::vk;
use ash::vk::StencilOpState;
use ash::vk::VertexInputAttributeDescription;
use ash::vk::VertexInputBindingDescription;
use crate::model::vertex::Vertex;
use crate::vk_obj::device::Device;
use crate::vk_obj::pipelines;
use crate::vk_obj::device;
use std::sync::Arc;
pub struct GraphicsPipeline {
    pub pipeline: vk::Pipeline,
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
#[derive(Default)]
pub struct GraphicsPipelineBuilder {
    flags: vk::PipelineCreateFlags,
    stage_count: u32,
    stages: vk::PipelineShaderStageCreateInfo,
    vertex_input_state: vk::PipelineVertexInputStateCreateInfo,
    viewport_state: vk::PipelineViewportStateCreateInfo,
    rasterization_state: vk::PipelineRasterizationStateCreateInfo,
    dynamic_state: vk::PipelineDynamicStateCreateInfo,
    layout: vk::PipelineLayout,
    render_pass: vk::RenderPass,
    subpass: u32,
    shader_stages: Vec<vk::PipelineShaderStageCreateInfo>,
}
impl GraphicsPipelineBuilder {
    pub fn new() -> Self {
        GraphicsPipelineBuilder::default()
    }
    pub fn subpass(mut self, subpass: u32) -> Self {self.subpass = subpass; self }
    pub fn rasterization(mut self, polygon_mode: vk::PolygonMode, culling: vk::CullModeFlags) -> Self {
        self.rasterization_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: 0,  // VK_FALSE
            rasterizer_discard_enable: 0,  // VK_FALSE
            polygon_mode: polygon_mode,
            line_width: 1.0,
            cull_mode: culling,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: 0, // VK_FALSE
            depth_bias_clamp: 0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_slope_factor: 0.0,
            ..Default::default()
        };
        self
    }
    pub fn dynamic_states(mut self, dynamic: &[vk::DynamicState]) -> Self {
        self.dynamic_state = vk::PipelineDynamicStateCreateInfo {
            p_dynamic_states: dynamic.as_ptr(),
            dynamic_state_count: dynamic.len() as u32,
            ..Default::default()
        };
        self
    }
    pub fn add_shader_stage(mut self, device: Arc<device::Device>, filepath: &str, entry: &str, stage: vk::ShaderStageFlags) -> Self {
        let module = pipelines::create_shader_module(&device.device, filepath);
        self.shader_stages.push(
            vk::PipelineShaderStageCreateInfo {
            stage: stage,
            module: module,
            p_name: entry.as_bytes().as_ptr() as *const i8,
            ..Default::default()
        });
        self
    }
    pub fn vertex_input_state<V: Vertex>(mut self, binding: &VertexInputBindingDescription, attribute: &Vec<VertexInputAttributeDescription>) -> Self {
        self.vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            vertex_attribute_description_count: attribute.len() as u32,
            p_vertex_attribute_descriptions: attribute.as_ptr(),
            vertex_binding_description_count: 1,
            p_vertex_binding_descriptions: binding,
            ..Default::default()
        };
        self
    }
    pub fn pipeline_layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.layout = layout;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.render_pass = render_pass;
        self
    }
    pub fn build(mut self, device: std::sync::Arc<Device>) -> GraphicsPipeline{
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: 0, // VK_FALSE
            ..Default::default()
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
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
            front: StencilOpState::default(),
            back: StencilOpState::default(),
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
        let create_info = vk::GraphicsPipelineCreateInfo {
            stage_count: self.shader_stages.len() as u32,
            p_stages: self.shader_stages.as_ptr(),
            p_vertex_input_state: &self.vertex_input_state,
            p_input_assembly_state: &input_assembly_state,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &self.rasterization_state,
            p_multisample_state: &multisample_state,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: &self.dynamic_state,
            layout: self.layout,
            render_pass: self.render_pass,
            subpass: self.subpass,
            base_pipeline_index: -1,

            ..Default::default()
        };

        let pipeline = unsafe { device.device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None).unwrap()[0] };
        GraphicsPipeline { pipeline, device }
    }
}

impl GraphicsPipeline {
    pub fn new<T>(device: Arc<device::Device>, info: &GraphicsPipelineInfo) -> Self
    where T: crate::model::vertex::Vertex {
        let binding = T::binding_description();
        let attribute = T::attribute_description();
        GraphicsPipelineBuilder::new()
        .add_shader_stage(device.clone(), &info.vertex_filepath, &info.vertex_entry, vk::ShaderStageFlags::VERTEX)
        .add_shader_stage(device.clone(), &info.fragment_filepath, &info.fragment_entry, vk::ShaderStageFlags::FRAGMENT)
        .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR])
        .pipeline_layout(info.layout)
        .render_pass(info.renderpass)
        .subpass(info.subpass)
        .rasterization(vk::PolygonMode::FILL, vk::CullModeFlags::NONE)
        .vertex_input_state::<T>(&binding, &attribute)
        .build(device.clone())
    }

    pub fn builder() -> GraphicsPipelineBuilder {
        GraphicsPipelineBuilder::default()
    }
}