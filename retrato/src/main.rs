mod device;
mod swapchain;
mod pipelines;
mod holly_types;
mod buffer;
mod app;
mod collision;
use std::{cell::RefCell, time::Instant};
use winarabica::window;
use puraexpr::{self, linear::f32::FVec2};

use app::{PushData, models::Model2D, basic::ColliderOptions};
use collision::oriented::OrientedSquareCollider;
use holly_types::{vertex::Vertex2D, transform::Transform2D};
mod rendering;
use ash::{Entry, vk::{self}};
fn main() {
    let vertices = vec![
        Vertex2D {coords: [-0.4, 0.4] },
        Vertex2D {coords: [0.4, 0.4] },
        Vertex2D {coords: [0.4, -0.4] },
        Vertex2D {coords: [-0.4, -0.4] },
    ];
    let transform = Transform2D::default();

    let mut game_obj = app::basic::BasicObject2D::new(vertices.clone(), vec![0, 2, 1, 0, 3, 2], RefCell::new(transform), app::basic::ColliderOptions::None);
    let oriented = OrientedSquareCollider::new(1.0, 1.0, game_obj.transform.clone());
    game_obj.collider = ColliderOptions::Oriented(oriented);
    // add a way to not need a batcher to create game objects nad
    let mut batcher = holly_types::batch::BatchRenderer::<Vertex2D, u32, Model2D>::default();
    

    // let event_loop = EventLoop::new();
    let mut resized = false;
    // let window = std::sync::Arc::new(WindowBuilder::new()
    //     .with_title("Holly Tree")
    //     .with_inner_size(LogicalSize::new(1024, 768))
    //     .build(&event_loop).unwrap()
    // );
    let window = std::sync::Arc::new(winarabica::window::Window::new());
    let entry = Entry::linked();
    let mut application = app::App::new(&entry, app::WindowOption::Winarabica(window.clone()));
    
    let mut constant = PushData {
        rot_mat: game_obj.matrix_2(),
        pos: FVec2::ZERO,
        rotation: 0.0
    };
    let raw = unsafe { std::mem::transmute::<&PushData, &[u8; std::mem::size_of::<PushData>()]>(&constant) };
    batcher.push_constants(Some(raw));
    batcher.push(&mut game_obj.model);
    
    let batch = batcher.create(&mut application.allocator);
    let mut current_time = Instant::now();
    let mut delta_time = 0.0;
    let mut suboptimal: Result<bool, vk::Result> = Ok(false);
    let mut main_loop = || {
        let new_time = Instant::now();
        let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

        application.renderer.begin_render_pass(cmd_buffer);

        unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
        unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[batch.vertex.buffer], &[0]) };
        unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, batch.index.buffer, 0, vk::IndexType::UINT32) };
        {
            let mut interior = game_obj.transform.borrow_mut();
            interior.rotation += 4.0 * delta_time;
            constant.rot_mat = interior.mat2();
            constant.pos = interior.translation;
        }
        
        let data = unsafe { std::mem::transmute::<&PushData, &[u8; std::mem::size_of::<PushData>()]>(&constant) };
        unsafe { application.device.device.cmd_push_constants(cmd_buffer, application.layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
        application.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };
        unsafe { application.device.device.cmd_draw_indexed(cmd_buffer, batch.index_count.unwrap(), 1, 0, 0, 0) };
        
        application.renderer.end(cmd_buffer);
        (application.renderer.image_index, suboptimal) = application.renderer.swapchain.submit(vec![cmd_buffer], application.renderer.image_index as usize);
        unsafe { application.device.device.device_wait_idle().unwrap() };
        
        delta_time = (new_time - current_time).as_secs_f32();
        current_time = new_time;
        if (suboptimal == Err(vk::Result::ERROR_OUT_OF_DATE_KHR) || suboptimal == Ok(true) || resized)
        {
            resized = false;
            application.renderer.recreate_swapchain();
        }
    };
    // loop {
    //     main_loop();
    //     println!("pass");
    //     window.get_messege();
    // }
    loop {
        main_loop();
        window.get_messege(window::ProgramState::Nonblocking, None);
    }
    // event_loop.run(move |event, _, control_flow| {
    //     *control_flow = ControlFlow::Poll;
    //     let new_time = Instant::now();
    //     println!("looped {}", delta_time);
    //     match event {
    //         Event::WindowEvent {
    //             event: WindowEvent::CloseRequested,
    //             window_id,
    //         } 
    //         if window_id == window.id() => *control_flow = ControlFlow::Exit,
    //         Event::WindowEvent { window_id, event: WindowEvent::Resized(new_size) } => {
    //             resized = true;
    //         }
    //         Event::WindowEvent { 
    //             window_id, 
    //             event: WindowEvent::KeyboardInput { device_id, input, is_synthetic },
// 
    //         } => {
    //             let mut interior = game_obj.transform.borrow_mut();
    //             println!("{:?} {}", input.virtual_keycode, delta_time);
    //         }
    //         Event::RedrawRequested(window_id) if window_id == window.id() => {
    //             
    //         }
    //         Event::MainEventsCleared => {
    //             window.request_redraw();
    //         }
    //         _ => (),
    //     }
    // });
}