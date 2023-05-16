use ash::vk::{self, Extent2D, Rect2D, Offset2D};

use crate::{device, swapchain};

pub struct Renderer {
    pub swapchain: swapchain::Swapchain,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub image_index: u32,
    device: std::sync::Arc<device::Device>,
    pub window: std::sync::Arc<winit::window::Window>,
}
impl Renderer {
    pub fn new(device: &std::sync::Arc<device::Device>, window: std::sync::Arc<winit::window::Window>) -> Self {
        let extent = window.inner_size();
        let swapchain = swapchain::Swapchain::new(device.clone(), vk::Extent2D {
            width: extent.width,
            height: extent.height
        });
        let command_buffers = Self::create_command_buffers(device.clone());
        
        Self { swapchain, command_buffers, image_index: 0, device: device.clone(), window }
    }
    fn create_command_buffers(device: std::sync::Arc<device::Device>) -> Vec<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo {
            level: vk::CommandBufferLevel::PRIMARY,
            command_pool: device.command_pool,
            command_buffer_count: 2, // Max Frames
            ..Default::default()
        };
        let command_buffers = unsafe { device.device.allocate_command_buffers(&alloc_info).unwrap() };
        command_buffers
    }
    pub fn recreate_swapchain(&self) {
        
    }
    pub fn begin_command_buffer(&mut self) -> Result<vk::CommandBuffer, vk::Result> {
        let result = self.swapchain.next_image();
        match result {
            Ok((o, _)) => {
                self.image_index = o;
                let command_buffer = self.command_buffers[self.swapchain.current_frame];
                let begin_info = vk::CommandBufferBeginInfo::default();
                unsafe { self.device.device.begin_command_buffer(command_buffer, &begin_info).unwrap() };
                return Ok(command_buffer);
            }
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain();
                return Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    pub fn begin_render_pass(&self, command_buffer: vk::CommandBuffer) {
        let clear_value = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                }
            }
        ];
        let begin_info = vk::RenderPassBeginInfo {
            clear_value_count: 2,
            p_clear_values: clear_value.as_ptr(),
            render_pass: self.swapchain.renderpass,
            framebuffer: self.swapchain.frambuffers[self.image_index as usize],
            render_area: Rect2D {
                offset: Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent
            },
            ..Default::default()
        };
        unsafe { self.device.device.cmd_begin_render_pass(command_buffer, &begin_info, vk::SubpassContents::INLINE) };
        let viewport = vk::Viewport {
            min_depth: 0.0,
            max_depth: 1.0,
            x: 0.0,
            y: 0.0,
            width: self.swapchain.extent.width as f32,
            height: self.swapchain.extent.height as f32,
            ..Default::default()
        };
        let scissor = vk::Rect2D {
            extent: self.swapchain.extent,
            offset: Offset2D { x: 0, y: 0 },
        };

        unsafe { self.device.device.cmd_set_viewport(command_buffer, 0, &[viewport]); };
        unsafe { self.device.device.cmd_set_scissor(command_buffer, 0, &[scissor]) };
    }
    pub fn end(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.device.device.cmd_end_render_pass(command_buffer) };
        unsafe { self.device.device.end_command_buffer(command_buffer).unwrap() };
    }
}
