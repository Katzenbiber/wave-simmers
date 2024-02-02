use clap::Parser;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

mod sim;
mod texture;
mod vis;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 0.0001)]
    timestep: f64,
    #[arg(short, long, default_value_t = 100)]
    x: u32,
    #[arg(short, long, default_value_t = 100)]
    y: u32,
    #[arg(short, long, default_value_t = 0.01)]
    c: f64,
    #[arg(short, long, default_value_t = 25.0)]
    init: f64,
}

#[pollster::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let window = Window::new(&event_loop).unwrap();
    let _ = window.request_inner_size(winit::dpi::PhysicalSize {
        width: 1000,
        height: 1000,
    });

    log::info!("Creating Simulation");
    let mut sim = sim::Simulation::new(&args);
    log::info!("Created Simulation");

    log::info!("Creating Visualizer");
    let vis = vis::Visualizer::new(&window, (args.x, args.y)).await;
    log::info!("Created Visualizer");

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            log::info!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::AboutToWait => {
            let mut casted = vec![0; (sim.x * sim.y) as usize];
            let field = sim.multi_step(50);
            for (n, node) in field.iter().enumerate() {
                casted[n] = (node.abs() * 100.0) as u8;
            }
            let casted = casted.into_boxed_slice();
            log::debug!("energy is: {}", sim.energy());
            vis.render(&casted);
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {}
        _ => (),
    });
}
