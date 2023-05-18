pub mod allocator;
pub mod raw;

trait Buffer {
    fn get_raw_buffer() -> ash::vk::Buffer;
}