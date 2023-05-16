use ash::vk;
use libc;
use crate::device;
pub struct Buffer<T> {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
    pub mapped: *mut T,
}

impl<T> Buffer<T> {
    pub fn new(device: std::sync::Arc<device::Device>, size: vk::DeviceSize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Self {
        let buffer = Self::create_buffer(device.clone(), size, usage);
        let requirements = unsafe { device.device.get_buffer_memory_requirements(buffer) };

        let memory_properties = unsafe { device.instance.instance.get_physical_device_memory_properties(device.physical_device) };
        let mut i = 0;
        for _ in 0..memory_properties.memory_type_count {
            if ((requirements.memory_type_bits & (1 << i) == (1 << i))) &&
				(memory_properties.memory_types[i].property_flags & properties) == properties {
				break; // i has memory type index
			}
            i += 1;
        }

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: i as u32,
            ..Default::default()
        };

        let memory = unsafe { device.device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.device.bind_buffer_memory(buffer, memory, 0).unwrap() };

        Self { buffer, memory, size, mapped: [].as_mut_ptr() as *mut T }
    }
    pub fn map(&mut self, device: std::sync::Arc<device::Device>, size: vk::DeviceSize, offset: vk::DeviceSize) {
        self.mapped = unsafe { device.device.map_memory(self.memory, offset, size, vk::MemoryMapFlags::empty()).unwrap() } as *mut T;
        
    }
    pub fn write_vec(&self, data: Vec<T>) {
        let size = data.len() * std::mem::size_of::<T>();
        if !self.mapped.is_null() && size <= self.size as usize  {
            unsafe { libc::memcpy(self.mapped as *mut libc::c_void , data.as_ptr() as *const libc::c_void, size); };
        }
    }
    pub fn unmap(&self, device: std::sync::Arc<device::Device>) {
        if !self.mapped.is_null() {
            unsafe { device.device.unmap_memory(self.memory) };
        }
    }
    fn create_buffer(device: std::sync::Arc<device::Device>, size: vk::DeviceSize, usage: vk::BufferUsageFlags) -> vk::Buffer {
        let create_info = vk::BufferCreateInfo {
            size: size,
            usage: usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let buffer = unsafe { device.device.create_buffer(&create_info, None).unwrap() };
        buffer
    }
}