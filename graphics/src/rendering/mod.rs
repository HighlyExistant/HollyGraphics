use ash::{vk::{self, Rect2D, Offset2D, SwapchainKHR, Extent2D}, extensions::khr::Swapchain};
use windows_sys::Win32::Foundation::*;
use crate::{device, hswapchain, app::WindowOption};

pub struct Renderer {
    pub swapchain: hswapchain::Swapchain,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub image_index: u32,
    device: std::sync::Arc<device::Device>,
    pub window: WindowOption,
    pub clear_value: vk::ClearColorValue,
}
impl Renderer {
    pub fn new(device: &std::sync::Arc<device::Device>, window: WindowOption) -> Self {
        let extent = window.get_extent2d();

        let swapchain = hswapchain::Swapchain::new(device.clone(), extent, SwapchainKHR::null());// , SwapchainKHR::null());
        
        let command_buffers = Self::create_command_buffers(device.clone());
        
        Self { swapchain, command_buffers, image_index: 0, device: device.clone(), window, clear_value: vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] } }
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
    pub fn recreate_swapchain(&mut self) {
        let mut window_extent = self.window.get_extent2d();
        while window_extent.width == 0 || window_extent.height == 0 {
            window_extent = self.window.get_extent2d();
        }
        unsafe { self.device.device.device_wait_idle().unwrap() };

        if (self.swapchain.swapchain ) == SwapchainKHR::null() {
            self.swapchain = hswapchain::Swapchain::new(self.device.clone(), window_extent, SwapchainKHR::null());
        } else {
            // Add other things here later
            self.swapchain  = hswapchain::Swapchain::new(self.device.clone(), window_extent, self.swapchain.swapchain);
        }
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
                let window_extent = self.window.get_extent2d();
                let extent = Extent2D {
                    width: window_extent.width,
                    height: window_extent.height,
                };
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
                color: self.clear_value,
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
    pub fn get_aspect_ratio(&self) -> f32 {
        (self.swapchain.extent.width as f32) / (self.swapchain.extent.height as f32)
    }
}
