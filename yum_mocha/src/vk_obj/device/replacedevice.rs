// This file will replace the device code just not yet

use ash::{vk, Entry};
use raw_window_handle::{ HasRawDisplayHandle, HasRawWindowHandle};

use super::instance;
pub struct LogicalDeviceQueueIndices {
    pub graphics: Option<u32>,
    pub compute_only: Option<u32>,
    pub transfer_only: Option<u32>,
}

pub struct LogicalDeviceBuilder {
    window: Option<&'static winit::window::Window>,
    queue_support: &'static [vk::QueueFlags],
}
pub struct LogicalDevice {
    pub instance: instance::VulkanInstance,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub present_queue: Option<vk::Queue>,
    pub graphics_queue: Option<vk::Queue>,
    pub command_pool: vk::CommandPool,
    // this field is used so that we can drop the surface
    pub surface_funcs: ash::extensions::khr::Surface
}

impl LogicalDeviceBuilder {
    /// by using this function you are telling Vulkan
    /// you want to use surface extensions for your application.
    pub fn set_window(mut self, window: &'static winit::window::Window) -> Self {
        self.window = Some(window);
        self
    }
    pub fn check_queue_support(mut self, support: &'static [vk::QueueFlags]) -> Self {
        self.queue_support = support;
        self
    }
    pub fn build(mut self) {
        // Instance and Surface Creation
        let entry = Entry::linked();

        let mut surface_extensions = false;
        let mut instancebuilder = instance::VulkanInstance::builder()
            .set_version(instance::ApiVersion::Type1_0)
            .enable_debugging();
        
        let (instance, surface) = if let Some(window) = self.window {
            instancebuilder = instancebuilder.enable_window_extensions((*window).raw_display_handle());
            surface_extensions = true;
            
            let vkinstace = instancebuilder.build();
            let surface: vk::SurfaceKHR = Self::create_surface_winit(&entry, &vkinstace.instance, window);
            (vkinstace, Some(surface))
        } else {
            let vkinstace = instancebuilder.build();
            (vkinstace, None)
        };
        // Choosing a Physical Device
        let (physical_device, surface_functions) =  self.choose_physical_device(&entry, &instance.instance, &surface);
    }

    fn create_surface_winit(entry: &Entry, instance: &ash::Instance, window: &winit::window::Window) -> vk::SurfaceKHR {
        let display = window.raw_display_handle();
        let window_hwnd = window.raw_window_handle();
        unsafe { ash_window::create_surface(entry, instance, display, window_hwnd, None).unwrap() }
    }
    fn choose_physical_device(&self, entry: &Entry, vkinstance: &ash::Instance, surface: &Option<vk::SurfaceKHR>) -> (vk::PhysicalDevice, ash::extensions::khr::Surface) {
        let physical_devices = unsafe { vkinstance.enumerate_physical_devices().unwrap() };
        let surface_funcs = ash::extensions::khr::Surface::new(entry, vkinstance);
        
        unsafe { 
            let physical_device = physical_devices.iter().find_map(|device| {
                vkinstance.get_physical_device_queue_family_properties(*device)
                .iter().enumerate()
                .find_map(|(i, info)| {
                    let mut support = true;
                    for condition in self.queue_support {
                        support = support && info.queue_flags.contains(*condition);
                    }
                    if let Some(s) = surface {
                        support = support && surface_funcs.get_physical_device_surface_support(*device,i as u32,*s,).unwrap();
                    }
                    if support {
                        Some((*device))
                    } else {
                        None
                    }
                })
            }).unwrap();
            (physical_device, surface_funcs)
        }
    }
}

impl LogicalDevice {
    fn builder() -> LogicalDeviceBuilder {
        todo!()
    }

}