mod model;
mod app;
mod camera;
mod input;
mod debug;
mod vk_obj;
mod components;
use std::{time::{Instant}};
use drowsed_math::{complex::quaternion::Quaternion, linear::{FVec3, FMat4}};
use ash::{Entry, vk::{self}};
use winit::{window::{WindowBuilder}, dpi::{LogicalSize}, event::WindowEvent, event_loop::ControlFlow};
use winit::event_loop::EventLoop;
use drowsed_math::linear::{Transform3D, TransformQuaternion3D, Transform};

use crate::{debug::DebugMovement, model::vertex::GlobalDebugVertex, vk_obj::{buffer}, app::{PushData3D, models::{Mesh3D, FromFBX}}, input::input_state::GlobalInputState};

fn main() {
    let global_input = GlobalInputState::new();
    let mut debug_movement = DebugMovement::new(global_input.clone());
    
    let mut camera = camera::Camera::default();
    camera.set_direction(debug_movement.transform.translation, debug_movement.transform.rotation, FVec3::new(0.0, -1.0, 0.0));
    
    let face1 = Mesh3D::<GlobalDebugVertex>::from_fbx("bad.fbx");
    
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
    transform.scale = FVec3::new(0.5, 0.5, 0.5);

    let event_loop = EventLoop::new();
    let mut resized = false;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    window.set_cursor_visible(false);

    let entry = Entry::linked();
    let mut application = app::App::new(&entry, app::WindowOption::Winit(window.clone()));
    let texture = buffer::img::ImageTexture::new(application.device.clone(), "Miles.JPG");
    for i in 0..2 {
        let info = texture.get_info(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let writer = vk_obj::descriptors::DescriptorWriter::new().add_image_buffer(application.sets[i], 1, 0, 0, &info);
        writer.write(application.device.clone());
    }
    let (vertex, index) = face1[0].create(application.device.clone());
    
    let mut constant = PushData3D {
        model: transform.matrix4(),
        transform: FMat4::identity(0.0),
    };

    let mut current_time = Instant::now();
    let mut delta_time = 0.0;

    let mut y= 0.0;
    transform.translation = FVec3::new(0.0, 0.0, 1.0);
    transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, 0.0, 0.0));
    transform.scale = FVec3::new(0.3, 0.3, 0.3);
    let mut delta_outside = [0.0f64, 0.0];
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        #[allow(unused_assignments)]
        let mut new_time = Instant::now();
        
        match event {
            winit::event::Event::DeviceEvent { device_id: _, event } => {
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
            winit::event::Event::WindowEvent { window_id: _, event: WindowEvent::Resized(_) } => {
                resized = true;
            }
            winit::event::Event::WindowEvent { 
                window_id: _, 
                event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ },
            } => {
                // TODO: A better approach would probably be to move the input variable into another variable that can be accessed by winit::event::Event::RedrawRequested
                let mut inputlock = global_input.lock().unwrap();
                
                inputlock.poll(input);
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                let aspect = application.renderer.get_aspect_ratio();
                camera.set_perspective_projection(0.872665, aspect, 0.1, 50.0);
                new_time = Instant::now();
                let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

                application.renderer.begin_render_pass(cmd_buffer);

                unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipeline) };
                
                unsafe { application.device.device.cmd_bind_descriptor_sets(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.layout, 0, &[application.sets[application.renderer.swapchain.current_frame]], &[]) };
                
                unsafe { application.device.device.cmd_bind_vertex_buffers(cmd_buffer, 0, &[vertex.buffer], &[0]) };
                unsafe { application.device.device.cmd_bind_index_buffer(cmd_buffer, index.buffer, 0, vk::IndexType::UINT32) };
                {
                    debug_movement.transform = debug_movement.movement(global_input.clone(), delta_time);
                    
                    camera.set_view_yxz(debug_movement.transform.translation, debug_movement.transform.rotation);
                    
                    // Cube Rotation Code
                    transform_euler.rotation = FVec3::new(0.0, 0.0, y);
                    let projection = camera.projection * camera.view;

                    transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                    y += 0.6 * delta_time;
                    let model = transform_euler.matrix4();
                    let normal_mat = transform_euler.set_scaling(FVec3::from(1.0) / transform_euler.scale).matrix3();
                    constant.transform = projection * model;
                    constant.model = normal_mat.into();
                }
                let data = unsafe { std::mem::transmute::<&PushData3D, &[u8; std::mem::size_of::<PushData3D>()]>(&constant) };
                unsafe { application.device.device.cmd_push_constants(cmd_buffer, application.layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, data) };
                application.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };
                unsafe { application.device.device.cmd_draw_indexed(cmd_buffer, face1[0].indices.len() as u32, 1, 0, 0, 0) };
                
                application.renderer.end(cmd_buffer);
                
                let (image_index, suboptimal) = application.renderer.swapchain.submit(vec![cmd_buffer], application.renderer.image_index as usize);
                application.renderer.image_index = image_index;
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