#![allow(unused)]
use ash::vk;
use crate::vk_obj::device;
pub struct DescriptorLayout {
    pub layout: vk::DescriptorSetLayout,
}
pub struct DescriptorLayoutBuilder {
    device: std::sync::Arc<device::Device>,
    flags: vk::DescriptorSetLayoutCreateFlags,
    bindings: Vec<vk::DescriptorSetLayoutBinding>,
    
}
impl DescriptorLayoutBuilder {
    pub fn new(device: std::sync::Arc<device::Device>) -> Self {
        Self { device, flags: vk::DescriptorSetLayoutCreateFlags::empty(), bindings: vec![] }
    }
    pub fn add_binding(mut self, binding: u32, ty: vk::DescriptorType, descriptor_count: u32, flags: vk::ShaderStageFlags) -> Self {
        let binding = vk::DescriptorSetLayoutBinding {
            binding: binding,
            descriptor_count: descriptor_count,
            descriptor_type: ty,
            stage_flags: flags,
            ..Default::default()
        };
        self.bindings.push(binding);
        self
    }
    pub fn set_flag(mut self, flags: vk::DescriptorSetLayoutCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn build(self) -> DescriptorLayout{
        let create_info = vk::DescriptorSetLayoutCreateInfo {
            p_bindings: self.bindings.as_ptr(),
            binding_count: self.bindings.len() as u32,
            flags: self.flags,
            ..Default::default()
        };
        let layout = unsafe { self.device.device.create_descriptor_set_layout(&create_info, None).unwrap() };
        DescriptorLayout { layout }
    }
}