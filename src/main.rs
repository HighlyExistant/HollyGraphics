mod device;
mod swapchain;
mod pipelines;
mod holly_types;
mod buffer;
use holly_types::Vertex2D;
mod rendering;
use std::println;

use ash::{Entry, vk::{Extent2D, self}};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, event::{WindowEvent, Event}};

use crate::pipelines::graphics;
// This code is ripped out of stack overflow: https://stackoverflow.com/questions/59707349/cast-vector-of-i8-to-vector-of-u8-in-rust
// for temporary fix to a debugging problem
fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
    // ideally we'd use Vec::into_raw_parts, but it's unstable,
    // so we have to do it manually:

    // first, make sure v's destructor doesn't free the data
    // it thinks it owns when it goes out of scope
    let mut v = std::mem::ManuallyDrop::new(v);

    // then, pick apart the existing Vec
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    
    // finally, adopt the data into a new Vec
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}
fn main() {
    let vertices = vec![
        Vertex2D {coords: [1.0, -1.0] },
        Vertex2D {coords: [0.0, 1.0] },
        Vertex2D {coords: [1.0, 0.0] },
    ];

    let event_loop = EventLoop::new();
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    let entry = Entry::linked();
    let engine =  device::Device::new(&entry, &window);
    
    let mut renderer = rendering::Renderer::new(&engine, window.clone());
    let layout_info = vk::PipelineLayoutCreateInfo {
        ..Default::default()
    };
    let layout = unsafe { engine.device.create_pipeline_layout(&layout_info, None).unwrap() };

    let graphics_info = graphics::GraphicsPipelineInfo {
        culling: vk::CullModeFlags::NONE,
        vertex_entry: String::from("main\0"),
        fragment_entry: String::from("main\0"),
        vertex_filepath: String::from("./shaders/vertex.vert.spv"),
        fragment_filepath: String::from("./shaders/vertex.frag.spv"),
        layout: layout,
        renderpass: renderer.swapchain.renderpass,
        ..Default::default()
    };
    let graphics = graphics::GraphicsPipeline::new::<holly_types::Vertex2D>(engine.clone(), &graphics_info);
    let vertex_size = (vertices.len() * std::mem::size_of::<Vertex2D>()) as u64;
    let mut vertex_buffer = buffer::Buffer::<Vertex2D>::new(
        engine.clone(), 
        (vertices.len() * std::mem::size_of::<Vertex2D>()) as u64, 
        vk::BufferUsageFlags::VERTEX_BUFFER, 
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
    vertex_buffer.map(engine.clone(), vertex_size, 0);
    vertex_buffer.write_vec(vertices);
    vertex_buffer.unmap(engine.clone());
    
    println!("queue index {}", engine.queue_index);
    let properties = unsafe { engine.instance.instance.get_physical_device_properties(engine.physical_device) };
    println!("{}", String::from_utf8(vec_i8_into_u8(properties.device_name.to_vec())).unwrap());
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            },
            Event::MainEventsCleared => {
                let cmd_buffer = renderer.begin_command_buffer().unwrap();

                renderer.begin_render_pass(cmd_buffer);

                unsafe { engine.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, graphics.pipeline) };
                unsafe { engine.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[vertex_buffer.buffer], &[0]) };
                unsafe { engine.device.cmd_draw(cmd_buffer, 3, 1, 0, 0) };
                renderer.end(cmd_buffer);
                
                renderer.image_index = renderer.swapchain.submit(vec![cmd_buffer], renderer.image_index as usize);
                unsafe { engine.device.device_wait_idle().unwrap() };
            }
            _ => (),
        }
    });
}