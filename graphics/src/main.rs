#![allow(unused)]
mod device;
mod hswapchain;
mod pipelines;
mod holly_types;
mod buffer;
mod app;
mod collision;
mod camera;
use std::{cell::RefCell, time::{Instant, Duration}};

use app::{models::{Model2D, Model3D, create_face, create_cube}, basic::ColliderOptions, PushData3D};
use buffer::raw::Buffer;
use drowsed_math::{complex::quaternion::Quaternion, linear::FVec3};
use holly_types::{vertex::{Vertex2D, Vertex3D}, transform::{Transform2D, Transform3D}, model};
mod rendering;
use ash::{Entry, vk::{self}};
use winit::{window::{Window, WindowBuilder}, dpi::LogicalSize, event::WindowEvent, event_loop::ControlFlow};
use winit::event_loop::EventLoop;

use crate::holly_types::vertex::Vertex3DRGB;
fn main() {
    let mut camera = camera::Camera::default();
    
    let face1 = create_cube();
    
    let mut transform = Transform3D {
        translation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
        scale: FVec3 { x: 1.0, y: 1.0, z: 1.0 },
        rotation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
    };
    let mut quaternion = Quaternion::<f32>::from_euler(FVec3::new(0.0, 1.0, 0.0));
    transform.scale = FVec3::new(0.5, 0.5, 0.5);
    
    let mut batcher = holly_types::batch::BatchRenderer::<Vertex3DRGB, u32, Model3D<Vertex3DRGB>>::default();

    let event_loop = EventLoop::new();
    let mut resized = false;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    let entry = Entry::linked();
    let mut application = app::App::new(&entry, app::WindowOption::Winit(window.clone()));
    let (vertex, index) = face1.create(&mut application.allocator);
    
    let mut constant = PushData3D {
        rot_mat: transform.mat4(),
    };

    let mut current_time = Instant::now();
    let mut delta_time = 0.0;
    let mut suboptimal: Result<bool, vk::Result> = Ok(false);
    let mut y= 0.0;
    transform.translation = FVec3::new(0.0, 0.0, 0.5);
    transform.rotation = FVec3::new(0.0, 0.0, 0.0);
    transform.scale = FVec3::new(0.3, 0.3, 0.3);
    // y += 0.1 * delta_time;
    println!("{:#?}", constant.rot_mat);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let mut new_time = Instant::now();
        match event {
            winit::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } 
            if window_id == window.id() => *control_flow = ControlFlow::Exit,
            winit::event::Event::WindowEvent { window_id, event: WindowEvent::Resized(new_size) } => {
                resized = true;
            }
            winit::event::Event::WindowEvent { 
                window_id, 
                event: WindowEvent::KeyboardInput { device_id, input, is_synthetic },
            } => {
                println!("{:?} {}", input.virtual_keycode, delta_time);
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    let aspect = application.renderer.get_aspect_ratio();
                    camera.set_orthographic_projection(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);
                    new_time = Instant::now();
                    let cmd_buffer = application.renderer.begin_command_buffer().unwrap();
            
                    application.renderer.begin_render_pass(cmd_buffer);

                    unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
                    unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[vertex.buffer], &[0]) };
                    unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, index.buffer, 0, vk::IndexType::UINT32) };
                    {
                        transform.rotation.x = y;
                        transform.rotation.y = y;
                        y += 0.3 * delta_time;
                        constant.rot_mat =  camera.projection * transform.mat4();
                    }
                    let data = unsafe { std::mem::transmute::<&PushData3D, &[u8; std::mem::size_of::<PushData3D>()]>(&constant) };
                    unsafe { application.device.device.cmd_push_constants(cmd_buffer, application.layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
                    application.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };
                    unsafe { application.device.device.cmd_draw_indexed(cmd_buffer, face1.indices.len() as u32, 1, 0, 0, 0) };
                    
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
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}