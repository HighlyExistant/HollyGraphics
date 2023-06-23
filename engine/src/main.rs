use std::{time::Instant, sync::Arc, rc::Rc, cell::RefCell};

use ash::{vk, Entry};
use drowsed_math::{linear::{FVec3, TransformQuaternion3D, FMat4, Transform}, complex::quaternion::Quaternion};
use components::{rendersystem::RenderSystem, models::{Mesh3D, FromFBX}, object::BasicObject, scene::Scene, physics::rigidbody::RigidBody};
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, dpi::LogicalSize, event::WindowEvent};
use yum_mocha::{self, input::input_state::GlobalInputState, debug::DebugMovement, camera, vk_obj::{device::WindowOption, buffer::{self, img::ImageTexture}, self, rendering::mesh::Renderable}, model::vertex::{GlobalDebugVertex, Vertex3DNormalUV}};
use crate::{app::PushData3D};
mod app;
mod object;
mod components;
use components::physics;
fn main() {
    let global_input = GlobalInputState::new();
    let mut debug_movement = DebugMovement::new(global_input.clone());
    
    let mut camera = camera::Camera::default();
    camera.set_direction(debug_movement.transform.translation, debug_movement.transform.rotation, FVec3::new(0.0, -1.0, 0.0));
    
    let mut monke = Rc::new(Mesh3D::<GlobalDebugVertex>::from_fbx("monke.fbx")[0].clone());
    let mut cube = Rc::new(Mesh3D::<GlobalDebugVertex>::from_fbx("untitled.fbx")[0].clone());
    let mut scene = Scene::new(vec![camera]);
    
    scene.push_object(0, BasicObject::new(TransformQuaternion3D::default()));
    scene.push_object(1, BasicObject::new(TransformQuaternion3D::default()));

    let event_loop = EventLoop::new();
    let mut resized = false;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    window.set_cursor_visible(false);

    let entry = Entry::linked();
    let mut application = app::App::new(&entry, WindowOption::Winit(window.clone()));
    let texture = ImageTexture::new(application.device.clone(), "Miles.JPG");
    for i in 0..2 {
        let info = texture.get_info(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let writer = vk_obj::descriptors::DescriptorWriter::new().add_image_buffer(application.sets[i], 1, 0, 0, &info);
        writer.write(application.device.clone());
    }
    let mut current_time = Instant::now();
    let mut delta_time = 0.0;

    let mut y= 0.0;
    let mut render_queue = RenderSystem::<GlobalDebugVertex, u32>::default();
    let mut physics_system = physics::physics_system::PhysicsSystem::new();
    render_queue.push(application.device.clone(), 0, monke.clone());
    render_queue.push(application.device.clone(), 1, cube.clone());
    physics_system.push(0, RigidBody::new(0.6));
    physics_system.set_gravity(FVec3::new(0.0, 5.0, 0.0));
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        #[allow(unused_assignments)]
        let mut new_time = Instant::now();
        
        match event {
            winit::event::Event::DeviceEvent { device_id: _, event: _ } => {}
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
                let mut inputlock = global_input.lock().unwrap();
                inputlock.poll(input);
                
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                let aspect = application.renderer.get_aspect_ratio();
                let get_camera = scene.get_camera_mut();
                get_camera.set_perspective_projection(0.872665, aspect, 0.1, 150.0);
                new_time = Instant::now();
                let cmd_buffer = application.renderer.begin_command_buffer().unwrap();

                application.renderer.begin_render_pass(cmd_buffer);

                unsafe { application.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.graphics.pipelines[0]) };
                
                unsafe { application.device.device.cmd_bind_descriptor_sets(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, application.layout, 0, &[application.sets[application.renderer.swapchain.current_frame]], &[]) };
                {
                    debug_movement.transform = debug_movement.movement(global_input.clone(), delta_time);
                    
                    get_camera.set_view_yxz(debug_movement.transform.translation, debug_movement.transform.rotation);
                    
                    // Model Rotation Code
                    if let Some(entity) = scene.get_object_by_id_mut(0) {
                        // entity.transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, 0.0, y));
                    }
                    if let Some(entity) = scene.get_object_by_id_mut(1) {
                        // entity.transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                        // entity.transform.translation.z -= 1.0 * delta_time;
                    }
                    if let Some(entity) = scene.get_object_by_id_mut(2) {
                        entity.transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                        entity.transform.translation.y -= 1.0 * delta_time;
                    }
                    y += 0.1  * delta_time;
                }
                let lock = global_input.lock().unwrap();
                let body = physics_system.get_rigidbody_by_id_mut(0).unwrap();
                
                if lock.is_pressed(winit::event::VirtualKeyCode::F) {
                    //body.velocity = FVec3::new(0.0, -10.0, 0.0);
                    body.apply_force(FVec3::new(0.0, -5.0, 0.0), FVec3::new(0.0, 0.2, 0.01));
                }
                if lock.is_pressed(winit::event::VirtualKeyCode::G) {
                    body.apply_force(FVec3::new(0.0, -0.7, 0.0), FVec3::new(0.0, -0.8, 0.0));
                }
                drop(lock);
                physics_system.render_all(application.device.clone(), delta_time, &mut scene);
                render_queue.render_all(application.device.clone(), cmd_buffer, application.layout, &scene);
                
                application.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };
                
                application.renderer.end(cmd_buffer);
                
                let suboptimal = application.renderer.draw(vec![cmd_buffer]);
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
