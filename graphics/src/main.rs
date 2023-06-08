#![allow(unused)]
mod device;
mod hswapchain;
mod pipelines;
mod model;
mod buffer;
mod app;
mod collision;
mod camera;
mod input;
mod debug;
mod descriptors;
use std::{time::{Instant, Duration}};
use app::{models::{Model3D, create_cube}, PushData3D};

use drowsed_math::{complex::quaternion::Quaternion, linear::FVec3};
mod rendering;
use ash::{Entry, vk::{self}};
use winit::{window::{WindowBuilder}, dpi::{LogicalSize}, event::WindowEvent, event_loop::ControlFlow};
use winit::event_loop::EventLoop;

use crate::{model::{vertex::{Vertex3DRGB, GlobalDebugVertex}, transform::{TransformQuaternion3D, Transform3D}}, app::models::create_cube_textured, debug::DebugMovement};
fn main() {
    let mut debug_movement = DebugMovement::new();
    
    let mut camera = camera::Camera::default();
    camera.set_direction(debug_movement.transform.translation, debug_movement.transform.rotation, FVec3::new(0.0, -1.0, 0.0));
    // camera.set_direction(debug_movement.transform.translation, FVec3::new(1.0, -1.0, 1.0), FVec3::new(0.0, -1.0, 0.0));
    
    let face1 = create_cube_textured(0);
    
    let mut transform = TransformQuaternion3D {
        translation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
        scale: FVec3 { x: 1.0, y: 1.0, z: 1.0 },
        rotation: Quaternion { vector: FVec3 { x: 0.0, y: 0.0, z: 0.0 }, scalar: 1.0 },
    };
    let mut transform_euler = Transform3D {
        translation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
        scale: FVec3 { x: 1.0, y: 1.0, z: 1.0 },
        rotation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
    };
    let mut quaternion = Quaternion::<f32>::from_euler(FVec3::new(0.0, 1.0, 0.0));
    transform.scale = FVec3::new(0.5, 0.5, 0.5);
    let mut batcher = rendering::batch::BatchRenderer::<GlobalDebugVertex, u32, Model3D<GlobalDebugVertex>>::default();

    let event_loop = EventLoop::new();
    let mut resized = false;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    window.set_cursor_visible(false);

    let outer_pos = window.inner_position().unwrap();
    
    let entry = Entry::linked();
    let mut application = app::App::new(&entry, app::WindowOption::Winit(window.clone()));
    let texture = buffer::img::ImageTexture::new(application.device.clone(), "LovelyCat-mini_3.JPG");
    for i in 0..2 {
        let info = texture.get_info(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let writer = descriptors::DescriptorWriter::new().add_image_buffer(application.sets[i], 1, 0, 0, &info);
        writer.write(application.device.clone());
    }
    let (vertex, index) = face1.create(application.device.clone());
    
    let mut constant = PushData3D {
        index: 0,
        rot_mat: transform.mat4(),
    };

    let mut current_time = Instant::now();
    let mut delta_time = 0.0;
    let mut suboptimal: Result<bool, vk::Result> = Ok(false);
    let mut y= 0.0;
    transform.translation = FVec3::new(0.0, 0.0, 1.0);
    transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, 0.0, 0.0));
    transform.scale = FVec3::new(0.3, 0.3, 0.3);
    let mut delta_outside = [0.0f64, 0.0];
    println!("{:#?}", constant.rot_mat);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let mut new_time = Instant::now();
        
        match event {
            winit::event::Event::DeviceEvent { device_id, event } => {
                match event {
                    winit::event::DeviceEvent::MouseMotion { delta } => {
                        delta_outside[0] = delta.0;
                        delta_outside[1] = delta.1;
                    }
                    
                    _ => {}
                }
            }
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
                // TODO: A better approach would probably be to move the input variable into another variable that can be accessed by winit::event::Event::RedrawRequested
                debug_movement.poll(input);
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    let aspect = application.renderer.get_aspect_ratio();
                    // camera.set_orthographic_projection(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);
                    camera.set_perspective_projection(0.872665, aspect, 0.1, 10.0);
                    new_time = Instant::now();
                    let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

                    application.renderer.begin_render_pass(cmd_buffer);

                    unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
                    
                    unsafe { application.device.device.cmd_bind_descriptor_sets(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.layout, 0, &[application.sets[application.renderer.swapchain.current_frame]], &[]) };
                    
                    unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[vertex.buffer], &[0]) };
                    unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, index.buffer, 0, vk::IndexType::UINT32) };
                    {
                        debug_movement.transform = debug_movement.movement(delta_time);
                        
                        camera.set_view_yxz(debug_movement.transform.translation, debug_movement.transform.rotation);
                        
                        // Cube Rotation Code
                        transform_euler.rotation = FVec3::new(0.0, 0.0, y);
                        let projection = camera.projection * camera.view;

                        transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                        y += 0.6 * delta_time;

                        constant.rot_mat =  projection * transform_euler.mat4();
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
                    if suboptimal == Err(vk::Result::ERROR_OUT_OF_DATE_KHR) || suboptimal == Ok(true) || resized
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