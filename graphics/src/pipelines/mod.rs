use std::io::Read;

use ash::vk;

pub mod graphics;

pub fn create_shader_module(device: &ash::Device, path: String) -> vk::ShaderModule {
    let file = std::fs::File::options().read(true).open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf).unwrap();

    let create_info = vk::ShaderModuleCreateInfo {
        code_size: buf.len(),
        p_code: buf.as_ptr() as *const u32,
        ..Default::default()
    };
    let shader_module = unsafe { device.create_shader_module(&create_info, None).unwrap() };
    shader_module
}