mod device;
mod swapchain;
mod pipelines;
mod holly_types;
mod buffer;
mod app;
use app::PushData;
use holly_types::{vertex::Vertex2D, transform::Transform2D};
mod rendering;
mod lin_alg;
use ash::{Entry, vk::{self}};
use lin_alg::f32::{FMat2, FVec2};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, event::{WindowEvent, Event}};
fn main() {
     let vertices1 = vec![
        Vertex2D {coords: [1.0, -1.0] },
        Vertex2D {coords: [0.0, 1.0] },
        Vertex2D {coords: [1.0, 0.0] },
    ];
    let vertices2 = vec![
        Vertex2D {coords: [1.0, -1.0] },
        Vertex2D {coords: [1.0, 1.0] },
        Vertex2D {coords: [-1.0, 0.0] },
    ];
    let mut game_obj = app::basic::BasicObject2D::new(vertices2, vec![0, 2, 1], Transform2D {..Default::default()});
    
    let mut batcher = holly_types::batch::BatchRenderer::<Vertex2D, u32>::default();
    
    batcher.push(&mut game_obj.model);

    let event_loop = EventLoop::new();
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    let entry = Entry::linked();
    let mut application = app::App::new(&entry, window.clone());
    
    let batch = batcher.create(&mut application.allocator);
    
    
    let mut constant = PushData {
        rot_mat: game_obj.transform.mat2(),
        pos: FVec2::ZERO,
        rotation: 0.0
    };
    
    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } 
            if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

                application.renderer.begin_render_pass(cmd_buffer);

                unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
                unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[batch.vertex.buffer], &[0]) };
                unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, batch.index.buffer, 0, vk::IndexType::UINT32) };
                game_obj.transform.rotation += 0.001;
                constant.rot_mat = game_obj.transform.mat2();
                let data = unsafe { std::mem::transmute::<&PushData, &[u8; std::mem::size_of::<PushData>()]>(&constant) };
                unsafe { application.device.device.cmd_push_constants(cmd_buffer, application.layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
                application.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };
                unsafe { application.device.device.cmd_draw_indexed(cmd_buffer, batch.index_count.unwrap(), 1, 0, 0, 0) };
                
                application.renderer.end(cmd_buffer);
                application.renderer.image_index = application.renderer.swapchain.submit(vec![cmd_buffer], application.renderer.image_index as usize);
                unsafe { application.device.device.device_wait_idle().unwrap() };
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}