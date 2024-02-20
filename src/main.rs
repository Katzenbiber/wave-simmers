use clap::Parser;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

mod sim;
mod texture;
mod vis;

#[derive(Parser, Debug)]
struct Args {
    /// Number of grid points in each dimension
    #[arg(short, long, default_value_t = 1000)]
    discretization: u32,
    /// Height of simulation in m
    #[arg(short, long, default_value_t = 10.0)]
    x: f64,
    /// Width of simulation in m
    #[arg(short, long, default_value_t = 10.0)]
    y: f64,
    /// Speed of wave in m/s
    #[arg(short, long, default_value_t = 1.0)]
    c: f64,
    /// Time step in seconds
    #[arg(long, default_value_t = 1e-3)]
    dt: f64,
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

    let vis_settings = vis::Settings {
        colors: (wgpu::Color::BLUE, wgpu::Color::RED),
        clamp: 1.0,
    };

    log::info!("Creating Visualizer");
    let vis = vis::Visualizer::new(
        &window,
        (args.discretization, args.discretization),
        vis_settings,
    )
    .await;
    log::info!("Created Visualizer");

    let mut steps_per_frame = 1;

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            log::info!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::AboutToWait => {
            log::debug!(
                "sim time: {:.4e} | energy: {:.4e}",
                &sim.time(),
                &sim.energy()
            );
            let field = sim.multi_step(steps_per_frame, args.dt);
            vis.render(field);
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {}
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(code),
                            ..
                        },
                    ..
                },
            ..
        } => match code {
            KeyCode::ArrowUp => {
                steps_per_frame += 1;
                log::info!("steps_per_frame: {}", steps_per_frame);
            }
            KeyCode::ArrowDown => {
                steps_per_frame -= 1;
                log::info!("steps_per_frame: {}", steps_per_frame);
            }
            _ => (),
        },
        _ => (),
    });
}
