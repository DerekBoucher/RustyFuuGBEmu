mod cartridge;
mod cpu;
mod gameboy;
mod memory;
mod ppu;
mod ui;

use clap::Parser;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::EventLoop;
use glium::Display;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_path: Option<String>,

    #[arg(short, long, default_value_t = false)]
    skip_boot_rom: bool,
}

fn main() {
    env_logger::init();
    log::info!("Starting RustyFuuGBemu");
    let args = Args::parse();

    let (events_loop, display) = init_glium();
    let egui_glium_client = egui_glium::EguiGlium::new(&display, &events_loop);

    let mut gameboy = gameboy::Gameboy::new();
    if args.skip_boot_rom {
        gameboy.skip_boot_rom();
    }

    let mut ui = ui::Ui::new(egui_glium_client);
    let gb_controller = gameboy.start();

    events_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    gb_controller.close();
                    gb_controller.join();
                    return;
                }

                _ => ui.process_events(event, &display),
            },
            Event::NewEvents(_) => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                ui.render(control_flow, &display, &gb_controller);
            }
            _ => {}
        }
    });
}

fn init_glium() -> (EventLoop<()>, Display) {
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("RustyFuuGBemu");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    return (events_loop, display);
}
