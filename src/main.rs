use wgpu::{Instance, InstanceDescriptor};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop).unwrap();

    let instance = Instance::new(InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            // render stuff
            todo!();
        }
        _ => (),
    });
}
