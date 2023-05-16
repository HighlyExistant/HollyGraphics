use ash::{vk::{self}, Entry};
mod instance;
use ash_window;
use raw_window_handle::{ HasRawDisplayHandle, HasRawWindowHandle};
use std::sync::Arc;
#[derive(Default)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>
}
pub struct Device {
    pub instance: instance::Instance,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub queue_index: usize,
    pub device: ash::Device,
    pub present_queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    // this field is used so that we can drop the surface
    pub surface_funcs: ash::extensions::khr::Surface
}
impl Device {
    pub fn new(entry: &Entry, window: &winit::window::Window) -> Arc<Self> {
        let instance = instance::Instance::new(&entry, &window);
        let surface = Self::create_surface(&entry, &instance.instance, &window);
        let (physical_device, queue_index, surface_funcs) = Self::choose_device(&entry, &instance, &surface);
        let device = Self::create_device(&instance.instance, &physical_device, queue_index as u32);
        let present_queue = unsafe { device.get_device_queue(queue_index as u32, 0) };
        let command_pool = Self::create_commandpool(&device, queue_index as u32);

        Arc::new(Self { instance, surface, physical_device, queue_index, device, surface_funcs, present_queue, command_pool })
    }
    fn create_surface(entry: &Entry, instance: &ash::Instance, window: &winit::window::Window) -> vk::SurfaceKHR {
        let display = window.raw_display_handle();
        let window_hwnd = window.raw_window_handle();
        unsafe { ash_window::create_surface(entry, instance, display, window_hwnd, None).unwrap() }
    }
    fn query_swapchain_support(surface_funcs: &ash::extensions::khr::Surface, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> SwapchainSupport {
        let support = unsafe { 
            SwapchainSupport {
                    capabilities: surface_funcs.get_physical_device_surface_capabilities(*physical_device, *surface).unwrap(),
                    formats: surface_funcs.get_physical_device_surface_formats(*physical_device, *surface).unwrap(),
                    present_modes: surface_funcs.get_physical_device_surface_present_modes(*physical_device, *surface).unwrap()
            } 
        };
        support
    }
    pub fn swapchain_support(&self) -> SwapchainSupport {
        Self::query_swapchain_support(&self.surface_funcs, &self.physical_device, &self.surface)
    }
    fn choose_device(entry: &Entry, instance: &instance::Instance, surface: &vk::SurfaceKHR) -> (vk::PhysicalDevice, usize, ash::extensions::khr::Surface) {
        let devices = unsafe { instance.instance.enumerate_physical_devices().unwrap() };
        let surface_funcs = ash::extensions::khr::Surface::new(entry, &instance.instance);

        let (physical_device, queue_family_index) = devices.iter().find_map(|device| {
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
                        Some((*device, index))
                    } else {
                        None
                    }
                }) 
            }
        }).unwrap();
        (physical_device, queue_family_index, surface_funcs)
    }

    fn create_device(instance: &ash::Instance, physical_device: &vk::PhysicalDevice, queue_index: u32) -> ash::Device {
        // Extensions
        let extensions = [
            ash::extensions::khr::Swapchain::name().as_ptr(),
        ];
        // Creation

        let queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: queue_index,
            queue_count: 1,
            p_queue_priorities: &1.0,
            ..Default::default()
        };

        let create_info = vk::DeviceCreateInfo {
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
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
}

impl Drop for crate::device::Device {
    fn drop(&mut self) {
        unsafe { 
            self.surface_funcs.destroy_surface(self.surface, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
        };
    }
}