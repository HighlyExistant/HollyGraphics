// use ash::vk;

// pub struct ImageTexture {
//     image: vk::Image,
//     view: vk::ImageView,
//     sampler: vk::Sampler,
//     memory: vk::DeviceMemory,
// }

// impl ImageTexture {
//     pub fn new(allocator: std::sync::Arc<device::Device>, filepath: &str) {
//         let img = image::open(filepath).unwrap();
        
//         let mut bytes = Vec::from(img.as_bytes());
        

//         let temp = Buffer::from_vec(
//             allocator, 
//             vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | 
//             vk::MemoryPropertyFlags::HOST_COHERENT, bytes);
//     }
// }