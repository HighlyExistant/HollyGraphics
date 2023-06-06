// #![allow(unused)]
mod device;
mod hswapchain;
mod pipelines;
mod holly_types;
mod buffer;
mod app;
mod collision;
mod camera;
mod input;
use std::{time::{Instant}};
use app::{models::{Model3D, create_cube}, PushData3D};

use drowsed_math::{complex::quaternion::Quaternion, linear::FVec3};
mod rendering;
use ash::{Entry, vk::{self}};
use winit::{window::{WindowBuilder}, dpi::{LogicalSize}, event::WindowEvent, event_loop::ControlFlow};
use winit::event_loop::EventLoop;

use crate::holly_types::{vertex::Vertex3DRGB, transform::{TransformQuaternion3D, Transform3D}};
fn main() {
    let mut user_key_w = input::InputKey::new(winit::event::VirtualKeyCode::W);
    let mut user_key_a = input::InputKey::new(winit::event::VirtualKeyCode::A);
    let mut user_key_s = input::InputKey::new(winit::event::VirtualKeyCode::S);
    let mut user_key_d = input::InputKey::new(winit::event::VirtualKeyCode::D);
    let mut user_key_n = input::InputKey::new(winit::event::VirtualKeyCode::N);
    let mut user_key_m = input::InputKey::new(winit::event::VirtualKeyCode::M);
    let mut user_key_k = input::InputKey::new(winit::event::VirtualKeyCode::K);
    let mut user_key_l = input::InputKey::new(winit::event::VirtualKeyCode::L);
    
    let mut camera = camera::Camera::default();
    let mut camera_transform = Transform3D {
        translation: FVec3::from(0.0),
        rotation: FVec3::new(0.0, 0.0, 0.0),
        scale: FVec3::from(1.0),
    };
    camera.set_direction(camera_transform.translation, camera_transform.rotation, FVec3::new(0.0, -1.0, 0.0));
    
    let face1 = create_cube();
    
    let mut transform = TransformQuaternion3D {
        translation: FVec3 { x: 0.0, y: 0.0, z: 0.0 },
        scale: FVec3 { x: 1.0, y: 1.0, z: 1.0 },
        rotation: Quaternion { vector: FVec3 { x: 0.0, y: 0.0, z: 0.0 }, scalar: 1.0 },
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
    window.set_cursor_visible(false);

    let outer_pos = window.inner_position().unwrap();
    
    let entry = Entry::linked();
    let mut application = app::App::new(&entry, app::WindowOption::Winit(window.clone()));
    let (vertex, index) = face1.create(application.device.clone());
    
    let mut constant = PushData3D {
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
        if window.has_focus() {
        window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
            // println!("focused");
        } else {
            // println!("not focused");
            window.set_cursor_grab(winit::window::CursorGrabMode::None);
        }
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
                match input.virtual_keycode {
                    Some(key) => match key {
                        winit::event::VirtualKeyCode::W => {
                           user_key_w.poll(input.state);
                        }
                        winit::event::VirtualKeyCode::A => {
                           user_key_a.poll(input.state);
                        }
                        winit::event::VirtualKeyCode::S => {
                           user_key_s.poll(input.state);

                        }
                        winit::event::VirtualKeyCode::D => {
                           user_key_d.poll(input.state);

                        }
                        winit::event::VirtualKeyCode::N => {
                           user_key_n.poll(input.state);
                        }
                        winit::event::VirtualKeyCode::M => {
                           user_key_m.poll(input.state);
                        }
                        winit::event::VirtualKeyCode::K => {
                           user_key_k.poll(input.state);
                        }
                        winit::event::VirtualKeyCode::L => {
                           user_key_l.poll(input.state);
                        }
                        _ => {}
                    }
                    None => {}
                }
                println!("{:?} {}", input.state, delta_time);
                println!("{:?} {}", input.virtual_keycode, delta_time);
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    let aspect = application.renderer.get_aspect_ratio();
                    // camera.set_orthographic_projection(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);
                    camera.set_perspective_projection(0.872665, aspect, 0.1, 10.0);
                    camera.set_direction(camera_transform.translation, FVec3::new(1.0, 1.0, 1.0), FVec3::new(0.0, -1.0, 0.0));
                    new_time = Instant::now();
                    let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

                    application.renderer.begin_render_pass(cmd_buffer);

                    unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
                    unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[vertex.buffer], &[0]) };
                    unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, index.buffer, 0, vk::IndexType::UINT32) };
                    {
                        if delta_outside[0] != 0.0 && delta_outside[1] != 0.0 {
                            // camera_transform.rotation.x += (delta_outside[0] as f32) * delta_time;
                            // camera_transform.rotation.y += (delta_outside[1] as f32) * delta_time;

                            // camera_transform.rotation.x = f32::cos(camera_transform.rotation.x);
                            // camera_transform.rotation.y = f32::sin(camera_transform.rotation.y);
                            println!("rotation: {:?}", camera_transform.rotation);
                        }
                        if user_key_w.pressed {
                            camera_transform.translation.z += 1.0 * delta_time;
                        }
                        if user_key_s.pressed {
                            camera_transform.translation.z -= 1.0 * delta_time;
                        }
                        if user_key_a.pressed {
                            camera_transform.translation.x -= 1.0 * delta_time;
                        }
                        if user_key_d.pressed {
                            camera_transform.translation.x += 1.0 * delta_time;
                        }
                        if user_key_n.pressed {
                            camera_transform.translation.y -= 1.0 * delta_time;
                        }
                        if user_key_m.pressed {
                            camera_transform.translation.y += 1.0 * delta_time;
                        }
                        if user_key_k.pressed {
                            camera_transform.rotation.x += 1.0 * delta_time;
                        }
                        if user_key_l.pressed {
                            camera_transform.rotation.x -= 1.0 * delta_time;
                        }
                        camera.set_view_yxz(camera_transform.translation, camera_transform.rotation);
                        let projection = camera.projection * camera.view;
                        transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                        y += 0.6 * delta_time;
                        constant.rot_mat =  projection * transform.mat4();
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