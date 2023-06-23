#![allow(unused)]
use ash::{vk::{self, Extent2D}, Entry};
// mod instance;
mod replacedevice;
mod instance;
use ash_window;
use raw_window_handle::{ HasRawDisplayHandle, HasRawWindowHandle};
use std::sync::Arc;

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
#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics: Option<u32>,
    pub surface: Option<u32>,
}

#[derive(Default)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}
pub struct Device {
    pub instance: instance::VulkanInstance,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub queue_indices: QueueFamilyIndices,
    pub device: ash::Device,
    pub present_queue: Option<vk::Queue>,
    pub graphics_queue: Option<vk::Queue>,
    pub command_pool: vk::CommandPool,
    // this field is used so that we can drop the surface
    pub surface_funcs: ash::extensions::khr::Surface
}
impl Device {
    pub fn new(entry: &Entry, window: Arc<winit::window::Window>) -> Arc<Self> {
        let instance = instance::VulkanInstance::builder().set_version(instance::ApiVersion::Type1_2).enable_debugging().enable_window_extensions(window.raw_display_handle()).build();
        let surface = Self::create_surface_winit(&entry, &instance.instance, &window);
        let (physical_device, surface_funcs) = Self::choose_device(&entry, &instance, &surface);
        let queue_indices = unsafe { Self::queue_family_indices(&physical_device, &instance.instance, &surface_funcs, &surface) };
        let device = Self::create_device(&instance.instance, &physical_device, &queue_indices );
        let present_queue = if let Some(i) = queue_indices.surface {
            let queue = unsafe { device.get_device_queue(i as u32, 0) };
            Some(queue)
        } else {
            None
        };
        // No reason to make graphics queue an option when it is necessary. This should instead be a panic but idk im lazy.
        let graphics_queue = if let Some(i) = queue_indices.graphics {
            let queue = unsafe { device.get_device_queue(i as u32, 0) };
            Some(queue)
        } else {
            None
        };
        
        let command_pool = Self::create_commandpool(&device, queue_indices.graphics.unwrap() as u32);

        Arc::new(Self { instance, surface, physical_device, queue_indices, device, surface_funcs, present_queue, graphics_queue, command_pool })
    }
    fn create_surface_winit(entry: &Entry, instance: &ash::Instance, window: &winit::window::Window) -> vk::SurfaceKHR {
        let display = window.raw_display_handle();
        let window_hwnd = window.raw_window_handle();
        unsafe { ash_window::create_surface(entry, instance, display, window_hwnd, None).unwrap() }
    }
    fn query_swapchain_support(surface_funcs: &ash::extensions::khr::Surface, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> SwapchainSupport {
        let support = unsafe { 
            SwapchainSupport {
                    capabilities: surface_funcs.get_physical_device_surface_capabilities(*physical_device, *surface).unwrap(),
                    formats: surface_funcs.get_physical_device_surface_formats(*physical_device, *surface).unwrap(),
                    present_modes: surface_funcs.get_physical_device_surface_present_modes(*physical_device, *surface).unwrap(),
            } 
        };
        support
    }
    pub fn swapchain_support(&self) -> SwapchainSupport {
        Self::query_swapchain_support(&self.surface_funcs, &self.physical_device, &self.surface)
    }
    fn choose_device(entry: &Entry, instance: &instance::VulkanInstance, surface: &vk::SurfaceKHR) -> (vk::PhysicalDevice, ash::extensions::khr::Surface) {
        let devices = unsafe { instance.instance.enumerate_physical_devices().unwrap() };
        let surface_funcs = ash::extensions::khr::Surface::new(entry, &instance.instance);
        let (physical_device) = devices.iter().find_map(|device: &vk::PhysicalDevice| {
            unsafe { 
                instance.instance
                    .get_physical_device_queue_family_properties(*device)
                    .iter().enumerate()
                    .find_map(|(index, info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && surface_funcs
                                .get_physical_device_surface_support(
                                    *device,
                                    index as u32,
                                    *surface,
                                )
                                .unwrap();
                    if supports_graphic_and_surface {
                        Some((*device))
                    } else {
                        None
                    }
                }) 
            }
        }).unwrap();
        (physical_device, surface_funcs)
    }

    fn create_device(instance: &ash::Instance, physical_device: &vk::PhysicalDevice, queue_indices: &QueueFamilyIndices) -> ash::Device {
        // Extensions
        let index_extension = b"VK_EXT_descriptor_indexing\0";

        let vk_features1_2 =  vk::PhysicalDeviceVulkan12Features {
            descriptor_indexing: 1, // VK_TRUE
            runtime_descriptor_array: 1, // VK_TRUE
            shader_sampled_image_array_non_uniform_indexing: 1, // VK_TRUE
            shader_storage_buffer_array_non_uniform_indexing: 1, // VK_TRUE
            descriptor_binding_partially_bound: 1, // VK_TRUE
            ..Default::default()
        };
        let physical_device_features = vk::PhysicalDeviceFeatures2 {
            p_next: &vk_features1_2 as *const _ as _,
            ..Default::default()
        };

        let extensions = [
            ash::extensions::khr::Swapchain::name().as_ptr(),
            index_extension.as_ptr() as _
        ];
        // Creation
		let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![];
		let mut unique_queue_families = std::collections::HashSet::new();
        unique_queue_families.insert(queue_indices.graphics.unwrap());
        for queue_index in unique_queue_families {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                queue_family_index: queue_index,
                queue_count: 1,
                p_queue_priorities: &1.0,
                ..Default::default()
            };
            queue_create_infos.push(queue_create_info);
        }
        let create_info = vk::DeviceCreateInfo {
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_next: &physical_device_features as *const _ as _,

            ..Default::default()
        };
        let device = unsafe { instance.create_device(*physical_device, &create_info, None).unwrap() };
        device
    }
    fn create_commandpool(device: &ash::Device, queue_index: u32) -> vk::CommandPool {
        let create_info = vk::CommandPoolCreateInfo {
            queue_family_index: queue_index,
            flags: vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            ..Default::default()
        };
        unsafe { device.create_command_pool(&create_info, None).unwrap() }
    }
    pub fn create_image(
        &self,
        info: &vk::ImageCreateInfo
    ) -> (vk::Image, vk::DeviceMemory) {
        let image = unsafe { self.device.create_image(info, None).unwrap() };
        // Allocate Memory
        let requirements = unsafe { self.device.get_image_memory_requirements(image) };
        let memory_properties = unsafe { self.instance.instance.get_physical_device_memory_properties(self.physical_device) };
        let mut memory_type_index = 0;
        for i in 0..memory_properties.memory_type_count {
            if requirements.memory_type_bits & (1 << i) == (1 << i)
            && memory_properties.memory_types[i as usize].property_flags & vk::MemoryPropertyFlags::DEVICE_LOCAL == (vk::MemoryPropertyFlags::DEVICE_LOCAL) {
                memory_type_index = i;
                break;
            }
        }

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_type_index,
            ..Default::default()
        };
        let memory = unsafe { self.device.allocate_memory(&alloc_info, None).unwrap() };
       
       unsafe { self.device.bind_image_memory(image, memory, 0).unwrap() };
        (image, memory)
    }
    pub unsafe fn find_supported_format(
        &self, candidates: &Vec<vk::Format> ,  tiling: vk::ImageTiling,  features: vk::FormatFeatureFlags) -> vk::Format {
      for format in candidates {
        let props = self.instance.instance.get_physical_device_format_properties(self.physical_device, *format);
    
        if tiling == vk::ImageTiling::LINEAR && (props.linear_tiling_features & features) == features {
          return *format;
        } else if 
            tiling == vk::ImageTiling::OPTIMAL && (props.optimal_tiling_features & features) == features {
          return *format;
        }
      }
      panic!("failed to find supported format!");
    }
    pub unsafe fn queue_family_indices(physical_device: &vk::PhysicalDevice, instance: &ash::Instance, surface_: &ash::extensions::khr::Surface, surface: &ash::vk::SurfaceKHR) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::default();

        let properties = instance.get_physical_device_queue_family_properties(*physical_device);
        println!("{:#?}", properties);
        let mut i = 0;
        for family in properties {
            if family.queue_count > 0 && (family.queue_flags & vk::QueueFlags::GRAPHICS) == vk::QueueFlags::GRAPHICS {
                indices.graphics = Some(i);
              }
              // instance.
              let present_support = ash::extensions::khr::Surface::get_physical_device_surface_support(&surface_, *physical_device, i, *surface).unwrap();
              if family.queue_count > 0 && present_support {
                indices.surface = Some(i);
                // indices.presentFamilyHasValue = true;
              }
              if indices.graphics.is_some() && indices.surface.is_some() {
                break;
              }
          
              i += 1;
        }
        indices
    }
    pub fn allocate_buffer(&self, size: usize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> vk::Buffer {
        let create_info = vk::BufferCreateInfo {
            size: size as u64,
            usage: usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let buffer = unsafe { self.device.create_buffer(&create_info, None).unwrap() };
        buffer
    }
    pub fn create_command_buffers(&self, level: vk::CommandBufferLevel, count: u32) -> Vec<vk::CommandBuffer> {
        let info = vk::CommandBufferAllocateInfo {
            level,
            command_buffer_count: count,
            command_pool: self.command_pool,
            ..Default::default()
        };
        unsafe { self.device.allocate_command_buffers(&info).unwrap() }
    }
    pub fn single_time_commands(&self) -> vk::CommandBuffer {
        let cmd = self.create_command_buffers(vk::CommandBufferLevel::PRIMARY, 1)[0];

        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        unsafe { self.device.begin_command_buffer(cmd, &begin_info).unwrap() };
        cmd
    }
    pub fn end_single_time_commands_graphics(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.device.end_command_buffer(command_buffer).unwrap() };
        let info = vk::SubmitInfo {
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            ..Default::default()
        };
        unsafe { 
            if let Some(queue) = self.graphics_queue {
                self.device.queue_submit(queue, &[info], vk::Fence::null()).unwrap();
                self.device.queue_wait_idle(queue).unwrap();
            }
            self.device.free_command_buffers(self.command_pool, &[command_buffer]);
        }

    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { 
            self.surface_funcs.destroy_surface(self.surface, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
        };
    }
}