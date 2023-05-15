use ash::{vk::{InstanceCreateInfo, ApplicationInfo}, Entry};
use ash_window;
use raw_window_handle::{self, HasRawDisplayHandle};

pub struct Instance {
    pub instance: ash::Instance,
}

impl Instance {
    pub fn new(entry: &Entry, window: &winit::window::Window) -> Self {
        let app_info = ApplicationInfo {
            ..Default::default()
        };
        let display = window.raw_display_handle();
        let required_extensions = ash_window::enumerate_required_extensions(
            display
        )
        .unwrap();
        ash::extensions::khr::Swapchain::name();
        let create_info = InstanceCreateInfo {
            enabled_extension_count: required_extensions.len() as u32,
            pp_enabled_extension_names: required_extensions.as_ptr(),
            enabled_layer_count: if cfg!(debug_assertions) {
                1
            } else {
                0
            },
            pp_enabled_layer_names: if cfg!(debug_assertions) {
                let validation_layers = b"VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8;
                [validation_layers].as_ptr()
            } else {
                InstanceCreateInfo::default().pp_enabled_layer_names
            },
            p_application_info: &app_info,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() }; 
        Self { instance }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}