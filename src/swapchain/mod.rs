#![allow(unused)]
use crate::device;
use ash::{vk::{self, SurfaceFormatKHR, PresentModeKHR, Extent2D, SharingMode, CompositeAlphaFlagsKHR, SwapchainKHR, ImageSubresourceRange, ImageViewType, SampleCountFlags, AttachmentLoadOp, AccessFlags, Extent3D, FenceCreateFlags}, extensions::khr};
#[derive(Default, Clone, Copy)]
pub struct ImageResource {
    pub image: vk::Image,
    pub view: vk::ImageView,
    pub memory: vk::DeviceMemory,
}
pub struct Swapchain<'a> {
    pub device: &'a device::Device,
    swapchain_funcs: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub old: vk::SwapchainKHR,
    pub format: vk::SurfaceFormatKHR,
    pub extent: Extent2D,
    pub image_count: u32,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub renderpass: vk::RenderPass,
    pub depth_format: vk::Format,
    depth_resources: Vec<ImageResource>,
    frambuffers: Vec<vk::Framebuffer>,
    image_available: Vec<vk::Semaphore>, 
    rendering_done: Vec<vk::Semaphore>, 
    in_flight_fence: Vec<vk::Fence>, 
    in_flight_images: Vec<vk::Fence>,
}

impl<'a> Swapchain<'a> {
    pub fn new(device: &'a device::Device, extent: Extent2D) -> Self {
        let swapchain_funcs: khr::Swapchain = khr::Swapchain::new(&device.instance.instance, &device.device);
        let old = SwapchainKHR::default();
        let (swapchain, format, extent, image_count) = Self::create_swapchain(&swapchain_funcs, device, extent, old);
        let images = unsafe { swapchain_funcs.get_swapchain_images(swapchain).unwrap() };
        let image_views = Self::create_image_views(image_count, &images, format.format, &device.device);
        let (renderpass, depth_format) = Self::create_renderpass(&device, format.format);
        let depth_resources = Self::create_depth_resources(&device, image_count, extent, depth_format);
        let frambuffers = Self::create_framebuffers(&device, image_count, extent, &image_views, &depth_resources, &renderpass);
        let (image_available, rendering_done, in_flight_fence, in_flight_images) = Self::create_sync_resources(&device, image_count);

        Self { 
            device, 
            swapchain_funcs, 
            swapchain, 
            old, 
            format, 
            extent, 
            image_count, 
            images, 
            image_views, 
            renderpass, 
            depth_format, 
            depth_resources, 
            frambuffers, 
            image_available, 
            rendering_done, 
            in_flight_fence, 
            in_flight_images 
        }
    }
    fn create_swapchain(swapchain_funcs: &ash::extensions::khr::Swapchain, device: &device::Device, extent: Extent2D, old: vk::SwapchainKHR) -> (vk::SwapchainKHR, SurfaceFormatKHR, Extent2D, u32) {
        let details = device.swapchain_support();

        let format = Self::choose_format(&details.formats);
        let present_mode = Self::choose_present_mode(&details.present_modes);
        let window_extent = Self::choose_extent(&details.capabilities, extent);

        let mut image_count = details.capabilities.min_image_count + 1;
        if (details.capabilities.max_image_count > 0 &&
			image_count > details.capabilities.max_image_count) {
            image_count = details.capabilities.max_image_count;
		}

        let create_info = vk::SwapchainCreateInfoKHR {
            surface: device.surface,
            min_image_count: image_count,
            image_format: format.format,
            image_color_space: format.color_space,
            image_extent: window_extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            pre_transform: details.capabilities.current_transform,
            composite_alpha: CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: present_mode,
            clipped: 1, // VK_TRUE
            old_swapchain: old,
            // The following entries depend on whether the indices for 
            // surface and graphics are the same and can change depending on device
            // TODO: Make it compatible for more devices by using multiple queue indices in Device and adding conditional for these values
            image_sharing_mode: SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: [].as_ptr(),
            ..Default::default()
        };
        
        let swapchain = unsafe { swapchain_funcs.create_swapchain(&create_info, None).unwrap() };
        (swapchain, format, window_extent, image_count)
    }
    fn create_image_views(image_count: u32, images: &Vec<vk::Image>, format: vk::Format, device: &ash::Device) -> Vec<vk::ImageView> {
        let views: Vec<vk::ImageView> = images.iter().map(|image| {
            let create_info = vk::ImageViewCreateInfo {
                image: *image,
                format: format,
                view_type: ImageViewType::TYPE_2D,
                subresource_range: ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1
                },
                ..Default::default()
            };

            let view = unsafe { device.create_image_view(&create_info, None).unwrap() };
            view
        }).collect();
        views
    }
    fn create_renderpass(device: &device::Device, format: vk::Format) -> (vk::RenderPass, vk::Format) {
        // FIND DEPTH FORMAT
        let tiling = vk::ImageTiling::OPTIMAL;
        let features = vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT;

        let candidates = vec![vk::Format::D32_SFLOAT, vk::Format::D32_SFLOAT_S8_UINT, vk::Format::D24_UNORM_S8_UINT];
        let mut depth_format = vk::Format::UNDEFINED;
        for candidate in candidates {
            let properties = unsafe { device.instance.instance.get_physical_device_format_properties(device.physical_device, candidate) };
            let anded = (properties.optimal_tiling_features & features);
            if tiling == vk::ImageTiling::OPTIMAL && anded == features {
                depth_format = candidate;
                break;
            } else {
                continue;
            }
        }
        if depth_format == vk::Format::UNDEFINED {
            panic!("Failed to find suitable format");
        }

        // Color Attachment
        let color_attachment = vk::AttachmentDescription {
            format: format,
            samples: SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,

            ..Default::default()
        };
        
        let color_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };
        // Depth Attachment
        let depth_attachment = vk::AttachmentDescription {
            format: depth_format,
            samples: SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };
        let depth_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };
        // Subpass
        let subpass = vk::SubpassDescription {
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            color_attachment_count: 1,
            p_color_attachments: &color_ref,
            p_depth_stencil_attachment: &depth_ref,
            ..Default::default()
        };

        let dependency = vk::SubpassDependency {
            src_subpass: u32::MAX, // VK_SUBPASS_EXTERNAL
            src_access_mask: AccessFlags::COLOR_ATTACHMENT_WRITE,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            dst_subpass: 0,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            dst_access_mask: AccessFlags::COLOR_ATTACHMENT_WRITE | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ..Default::default()
        };
        let attachments = [color_attachment, depth_attachment];
        let create_info = vk::RenderPassCreateInfo {
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 1,
            p_dependencies: &dependency,
            ..Default::default()
        };
        let renderpass = unsafe { device.device.create_render_pass(&create_info, None).unwrap() };
        (renderpass, depth_format)
    }
    fn create_depth_resources(device: &'a device::Device,image_count: u32, extent: Extent2D, depth_format: vk::Format) -> Vec<ImageResource> {
        let mut resources: Vec<ImageResource> = vec![ImageResource::default(); image_count as usize];
        for i in 0..resources.len() {
            // Create Image
            let image_info = vk::ImageCreateInfo {
                extent: Extent3D {
                    width: extent.width,
                    height: extent.height,
                    depth: 1,
                },
                mip_levels: 1,
                format: depth_format,
                tiling: vk::ImageTiling::OPTIMAL,
                image_type: vk::ImageType::TYPE_2D,
                usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                array_layers: 1,
                samples: SampleCountFlags::TYPE_1,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            };
            resources[i].image = unsafe { device.device.create_image(&image_info, None).unwrap() };
            // Allocate Memory
            let requirements = unsafe { device.device.get_image_memory_requirements(resources[i].image) };
            let memory_properties = unsafe { device.instance.instance.get_physical_device_memory_properties(device.physical_device) };
            let mut memory_type_index = 0;
            for i in 0..memory_properties.memory_type_count {
                if (requirements.memory_type_bits & (1 << i) == (1 << i)) 
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
            resources[i].memory = unsafe { device.device.allocate_memory(&alloc_info, None).unwrap() };
           
           unsafe { device.device.bind_image_memory(resources[i].image, resources[i].memory, 0).unwrap() };
            // Create Image View
            let view_info = vk::ImageViewCreateInfo {
                image: resources[i].image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: depth_format,
                subresource_range: ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::DEPTH,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            resources[i].view = unsafe { device.device.create_image_view(&view_info, None).unwrap() };
        }
        resources
    }
    fn create_framebuffers(device: &'a device::Device, image_count: u32, extent: Extent2D, color: &Vec<vk::ImageView>, depth: &Vec<ImageResource>, renderpass: &vk::RenderPass) -> Vec<vk::Framebuffer> {
        let mut frambuffers = vec![vk::Framebuffer::default(); image_count as usize];

        for i in 0..image_count as usize {
            let attachments = [color[i], depth[i].view];
            let create_info = vk::FramebufferCreateInfo {
                render_pass: *renderpass,
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: extent.width,
                height: extent.height,
                layers: 1,
                ..Default::default()
            };
            frambuffers[i] = unsafe { device.device.create_framebuffer(&create_info, None).unwrap() };
        }
        frambuffers
    }
    fn create_sync_resources(device: &'a device::Device,image_count: u32) -> (Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>, Vec<vk::Fence>) {
        let mut image_available = vec![vk::Semaphore::default(); 2];
        let mut rendering_done = vec![vk::Semaphore::default(); 2];
        let mut in_flight_fence = vec![vk::Fence::default(); 2];
        let in_flight_image = vec![vk::Fence::default(); image_count as usize];

        let semaphore_info = vk::SemaphoreCreateInfo {
            ..Default::default()
        };
        let fence_info = vk::FenceCreateInfo {
            flags: FenceCreateFlags::SIGNALED,
            ..Default::default()
        };
        for i in 0..2 {
            image_available[i] = unsafe { device.device.create_semaphore(&semaphore_info, None).unwrap() };
            rendering_done[i] = unsafe { device.device.create_semaphore(&semaphore_info, None).unwrap() };
            in_flight_fence[i] = unsafe { device.device.create_fence(&fence_info, None).unwrap() };
        }
        (image_available, rendering_done, in_flight_fence, in_flight_image)
    }
    fn choose_format(formats: &Vec<SurfaceFormatKHR>) -> SurfaceFormatKHR {
        
		for format in formats {
			if (format.format == ash::vk::Format::B8G8R8A8_SRGB
				&& format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR)
			{
				return *format;
			}
        }
        formats[0]
    }
    fn choose_present_mode(present_modes: &Vec<PresentModeKHR>) -> PresentModeKHR {
        for mode in present_modes {
            if *mode == PresentModeKHR::MAILBOX {
                return *mode;
            }
        }
        return PresentModeKHR::FIFO;
    }
    fn choose_extent(capabilities: &vk::SurfaceCapabilitiesKHR, extent: Extent2D) -> Extent2D {
        if capabilities.current_extent.width != std::u32::MAX {
            return capabilities.current_extent;
        }
        
        let mut actual_extent = extent;
        actual_extent.width = std::cmp::max(
			capabilities.min_image_extent.width,
			std::cmp::min(capabilities.min_image_extent.width, actual_extent.width));
            actual_extent.height = std::cmp::max(
			capabilities.min_image_extent.height,
			std::cmp::min(capabilities.min_image_extent.height, actual_extent.height));

        actual_extent
    }
}

impl<'a> Drop for Swapchain<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_render_pass(self.renderpass, None);
            self.swapchain_funcs.destroy_swapchain(self.swapchain, None); 
        }
    }
}