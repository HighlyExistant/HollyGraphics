mod device;
use std::println;

use ash::{Entry};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, event::{WindowEvent, Event}};
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
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Holly Tree")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap();
    
    let entry = unsafe { Entry::load().unwrap() };
    let engine =  device::Device::new(&entry, &window);
    println!("queue index {}", engine.queue_index);
    let properties = unsafe { engine.instance.instance.get_physical_device_properties(engine.physical_device) };
    println!("{}", String::from_utf8(vec_i8_into_u8(properties.device_name.to_vec())).unwrap());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}