use ash::vk;

use crate::device;
use std::sync::Arc;

pub struct BufferAllocator {
    pub device: Arc<device::Device>,
    buffers: Vec<*mut vk::Buffer>
}

impl BufferAllocator {
    pub fn new(device: Arc<device::Device>) -> Self {
        let buffers: Vec<*mut vk::Buffer> = vec![];

        Self { device, buffers }
    }
    pub(crate) fn allocate(&mut self, size: vk::DeviceSize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> vk::Buffer {
        let create_info = vk::BufferCreateInfo {
            size: size,
            usage: usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let buffer = unsafe { self.device.device.create_buffer(&create_info, None).unwrap() };
        buffer
    }
}