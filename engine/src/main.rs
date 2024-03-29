#![allow(unused)]
use std::{time::Instant, rc::Rc, cell::RefCell};

use ash::{vk, Entry};
use drowsed_math::{{FVec3, TransformQuaternion3D}, complex::quaternion::Quaternion, LinearSegment};
use components::{object::BasicObject, scene::Scene, rendering::models::{Model, FromFBX}};
use motor::{SchonMotor, system_manager::SystemManagerInfo, SolidMotor};
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, dpi::LogicalSize, event::WindowEvent};
use yum_mocha::{self, input::input_state::GlobalInputState, debug::DebugMovement, camera, vk_obj::{device::WindowOption, buffer::img::ImageTexture, self}, model::vertex::GlobalDebugVertex};
use mofongo::{bodies::RigidBody, solid::collisions::gjk::GJKColliderSolid};
mod components;
mod motor;
use drowsed_math::Segment;
fn main() {
    let global_input = GlobalInputState::new();
    let mut debug_movement = DebugMovement::new(global_input.clone());
    
    let mut camera = camera::Camera::default();
    camera.set_direction(debug_movement.transform.translation, debug_movement.transform.rotation.to_euler(), FVec3::new(0.0, -1.0, 0.0));
    
    let monke = Rc::new(Model::<GlobalDebugVertex>::from_fbx("untitled.fbx")[0].clone());
    let cube = Rc::new(Model::<GlobalDebugVertex>::from_fbx("untitled.fbx")[0].clone());
    let mut scene = Scene::new(vec![camera]);
    
    scene.push_object(0, BasicObject::new(TransformQuaternion3D { translation: FVec3::new(0.0, -10.0, 0.0), ..Default::default() }));
    scene.push_object(1, BasicObject::new(TransformQuaternion3D::default()));

    let vertices1: Vec<_> = cube.vertices.clone().iter().map(|v| {
        v.pos
    }).collect();
    let vertices2: Vec<_> = cube.vertices.clone().iter().map(|v| {
        v.pos
    }).collect();
    let collider1 = Rc::new(RefCell::new(GJKColliderSolid::new(vertices1.clone())));
    let collider2 = Rc::new(RefCell::new(GJKColliderSolid::new(vertices2.clone())));

    let event_loop = EventLoop::new();
    let mut resized = false;
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    window.set_cursor_visible(false);

    let entry = Entry::linked();
    let info = SystemManagerInfo {
        global_gravity: FVec3::new(0.0, 0.0, 0.0),
    };
    let mut schonmotor = SchonMotor::<SolidMotor>::new(&entry, WindowOption::Winit(window.clone()), &info);
    schonmotor.push_scene(scene);
    let texture = ImageTexture::new(schonmotor.device_manager.device.clone(), "Miles.JPG");
    for i in 0..2 {
        let info = texture.get_info(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let writer = vk_obj::descriptors::DescriptorWriter::new().add_image_buffer(schonmotor.device_manager.sets[i], 1, 0, 0, &info);
        writer.write(schonmotor.device_manager.device.clone());
    }
    let mut current_time = Instant::now();
    let mut delta_time = 0.0;
    let mut y= 0.0;
    {
        schonmotor.system_manager.collisions.push(0, collider1.clone());
        schonmotor.system_manager.collisions.push(1, collider2.clone());
        schonmotor.system_manager.rendering.push(schonmotor.device_manager.device.clone(), 0, monke.clone());
        schonmotor.system_manager.rendering.push(schonmotor.device_manager.device.clone(), 1, cube.clone());
        schonmotor.system_manager.physics.push(0, mofongo::solid::physics::rigidbody::RigidBody3D::new(0.6));
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
                    let aspect = schonmotor.device_manager.renderer.get_aspect_ratio();
                    let scene = schonmotor.system_manager.scene_manager.get_selected_scene_mut();
                    let get_camera = scene.get_camera_mut();
                    get_camera.set_perspective_projection(0.872665, aspect, 0.1, 150.0);
                    new_time = Instant::now();
                    let cmd_buffer = schonmotor.device_manager.renderer.begin_command_buffer().unwrap();

                    schonmotor.device_manager.renderer.begin_render_pass(cmd_buffer);

                    unsafe { schonmotor.device_manager.device.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, schonmotor.device_manager.graphics.pipelines[0]) };
                    let body = schonmotor.system_manager.physics.get_rigidbody_by_id_mut(0).unwrap();

                    unsafe { schonmotor.device_manager.device.device.cmd_bind_descriptor_sets(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, schonmotor.device_manager.layout, 0, &[schonmotor.device_manager.sets[schonmotor.device_manager.renderer.swapchain.current_frame]], &[]) };
                    {
                        debug_movement.transform = debug_movement.movement(global_input.clone(), delta_time);

                        get_camera.set_view_yxz(debug_movement.transform.translation, debug_movement.transform.rotation.to_euler());

                        // Model Rotation Code
                        if let Some(entity) = scene.get_object_by_id_mut(0) {}
                        if let Some(entity) = scene.get_object_by_id_mut(1) {}
                        if let Some(entity) = scene.get_object_by_id_mut(2) {
                            entity.transform.rotation = Quaternion::<f32>::from_euler(FVec3::new(0.0, y, y));
                            entity.transform.translation.y -= 1.0 * delta_time;
                        }
                        if let Some((collider, info)) = schonmotor.system_manager.collisions.get_collider_by_id(0) {
                            let cell = collider.borrow();
                            if let Some(i) = info.get() {
                                println!("info {:?}", i);
                            }
                        } else {
                        }
                        y += 0.1  * delta_time;
                    }
                    let lock = global_input.lock().unwrap();

                    if lock.is_pressed(winit::event::VirtualKeyCode::F) {
                        body.apply_force(FVec3::new(0.006, 10.0, 0.0), FVec3::new(0.006, 0.0, 0.0));
                    }
                    if lock.is_pressed(winit::event::VirtualKeyCode::H) {
                        body.apply_force(FVec3::new(0.5, 0.0, 0.0), FVec3::new(0.0, 0.0, 0.0));
                    }
                    if lock.is_pressed(winit::event::VirtualKeyCode::T) {
                        body.apply_force(FVec3::new(-0.5, 0.0, 0.0), FVec3::new(0.0, 0.0, 0.0));
                    }
                    drop(lock);
                    schonmotor.system_manager.collisions.render(schonmotor.device_manager.device.clone(), &mut schonmotor.system_manager.scene_manager);
                    schonmotor.system_manager.physics.render(schonmotor.device_manager.device.clone(), delta_time, &mut schonmotor.system_manager.scene_manager);
                    schonmotor.system_manager.rendering.render(schonmotor.device_manager.device.clone(), cmd_buffer, schonmotor.device_manager.layout, &schonmotor.system_manager.scene_manager);
                    schonmotor.device_manager.renderer.clear_value = vk::ClearColorValue {float32: [0.0, 0.0, 0.0, 1.0] };

                    schonmotor.device_manager.renderer.end(cmd_buffer);

                    let suboptimal = schonmotor.device_manager.renderer.draw(vec![cmd_buffer]);
                    unsafe { schonmotor.device_manager.device.device.device_wait_idle().unwrap() };

                    delta_time = (new_time - current_time).as_secs_f32();
                    current_time = new_time;
                    if suboptimal == Err(vk::Result::ERROR_OUT_OF_DATE_KHR) || suboptimal == Ok(true) || resized
                    {
                        resized = false;
                        schonmotor.device_manager.renderer.recreate_swapchain();
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
}
