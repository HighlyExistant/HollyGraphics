use ash::{vk::{InstanceCreateInfo, ApplicationInfo}, Entry, extensions::ext::DebugUtils};
use ash_window;
use raw_window_handle::{self, HasRawDisplayHandle};
pub struct Instance {
    pub instance: ash::Instance,
}

impl Instance {
    pub fn new(entry: &Entry, display: raw_window_handle::RawDisplayHandle) -> Self {
        let app_info = ApplicationInfo {
            api_version: 4194304,
            ..Default::default()
        };
        let mut required_extensions = ash_window::enumerate_required_extensions(
            display
        )
        .unwrap().to_vec();
        required_extensions.push(DebugUtils::name().as_ptr());
        
       let validation_layers = b"VK_LAYER_KHRONOS_validation\0";
       let ptr = validation_layers.as_ptr() as *const i8;
        let enabled_layer_count = if cfg!(debug_assertions) {
            1
        } else {
            0
        };
        let create_info = InstanceCreateInfo {
            enabled_extension_count: required_extensions.len() as u32,
            pp_enabled_extension_names: required_extensions.as_ptr(),
            enabled_layer_count: enabled_layer_count,
            pp_enabled_layer_names: [ptr].as_ptr(),
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