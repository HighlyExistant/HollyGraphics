use ash::vk::{self, Extent3D, Offset3D, ImageSubresourceLayers};
use std::{sync::Arc, collections::btree_set::Iter};

use crate::vk_obj::device::Device;

pub struct Buffer<T> {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
    pub mapped: *mut T,
}

impl<T> Buffer<T> {
    /// Contstructs a new *buffer* using *Arc<<Device>>*
    /// # Examples
    /// ```
    /// use holly::buffer::allocator;
    /// use holly::buffer::raw;
    /// fn main() {
    ///     ...
    ///     let buffer = raw::Buffer::new(device.clone(), 4096, 
    ///         vk::BufferUsageFlags::TRANSFER_SRC, 
    ///         vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ///     );
    /// }
    /// ```
    pub fn new(device: Arc<Device>, size: usize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Self {
        let buffer = device.allocate_buffer(size, usage, properties);
        let requirements = unsafe { device.device.get_buffer_memory_requirements(buffer) };
        let memory_index = Self::get_memory_type_index(device.clone(), properties, requirements);

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_index,
            ..Default::default()
        };
        
        let memory = unsafe { device.device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.device.bind_buffer_memory(buffer, memory, 0).unwrap() };
        
        Self { buffer, memory, size: size as u64, mapped: [].as_mut_ptr() as *mut T }
    }
    fn get_memory_type_index(device: Arc<Device>, properties: vk::MemoryPropertyFlags, requirements: vk::MemoryRequirements) -> u32 {
        let memory_properties = unsafe { device.instance.instance.get_physical_device_memory_properties(device.physical_device) };
        let i = (0..memory_properties.memory_type_count).find_map(|i| {
            if requirements.memory_type_bits & (1 << i) == (1 << i) &&
				memory_properties.memory_types[i as usize].property_flags & properties == properties {
				Some(i)
			} else {
                None
            }
        }).unwrap();

        i
    }
    pub fn map(&mut self, device: Arc<Device>, size: usize, offset: vk::DeviceSize) {
        self.mapped = unsafe { device.device.map_memory(self.memory, offset, size as u64, vk::MemoryMapFlags::empty()).unwrap() } as *mut T;
    }
    pub fn write_vec(&self, data: Vec<T>) {
        let size = data.len() * std::mem::size_of::<T>();
        if !self.mapped.is_null() && size <= self.size as usize  {
            unsafe { libc::memcpy(self.mapped as *mut libc::c_void , data.as_ptr() as *const libc::c_void, size); };
        }
    }
    pub fn unmap(&self, device: std::sync::Arc<Device>) {
        if !self.mapped.is_null() {
            unsafe { device.device.unmap_memory(self.memory) };
        }
    }
    pub fn from_iter(device: Arc<Device>, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags, iter: Iter<T>) {
        let size = iter.len() * std::mem::size_of::<T>();
        let buffer = device.allocate_buffer(size, usage, properties);
        let requirements = unsafe { device.device.get_buffer_memory_requirements(buffer) };
        let memory_index = Self::get_memory_type_index(device.clone(), properties, requirements);

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_index,
            ..Default::default()
        };
        
        let memory = unsafe { device.device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.device.bind_buffer_memory(buffer, memory, 0).unwrap() };
        let mut this = Self { buffer, memory, size: size as u64, mapped: [].as_mut_ptr() as *mut T };
        this.map(device.clone(), size, 0);
        let mapped = this.mapped;
        for val in iter {
            
            unsafe { libc::memcpy(mapped as *mut libc::c_void , val as *const _ as _, std::mem::size_of::<T>()); };
            unsafe { mapped.add(std::mem::size_of::<T>()) };
        }
    }
    pub fn from_vec(device: Arc<Device>, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags, vec: Vec<T>) -> Self {
        let size = vec.len() * std::mem::size_of::<T>();
        let buffer = device.allocate_buffer(size, usage, properties);
        let requirements = unsafe { device.device.get_buffer_memory_requirements(buffer) };
        
        let memory_properties = unsafe { device.instance.instance.get_physical_device_memory_properties(device.physical_device) };
        let mut i = Self::get_memory_type_index(device.clone(), properties, requirements);

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: i as u32,
            ..Default::default()
        };
        
        let memory = unsafe { device.device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.device.bind_buffer_memory(buffer, memory, 0).unwrap() };
        
        let mut ret = Self { buffer, memory, size: size as u64, mapped: [].as_mut_ptr() as *mut T };
        ret.map(device.clone(), size, 0);

        ret.write_vec(vec);

        ret
    }
    pub fn to_image(&self, device: Arc<Device>, image: &vk::Image, width: u32, height: u32) {
        let command_buffer = device.single_time_commands();
        let copy = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
                ..Default::default()
            },
            image_offset: Offset3D {
                x: 0,
                y: 0,
                z: 0,
            },
            image_extent: Extent3D {
                width,
                height,
                depth: 1,
            }
        };
        unsafe { device.device.cmd_copy_buffer_to_image(command_buffer, self.buffer, *image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[copy]) };
        device.end_single_time_commands_graphics(command_buffer);
    }
}