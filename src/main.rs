mod cartridge;
mod cpu;
mod gameboy;
mod memory;
mod ppu;

extern crate egui;

use clap::Parser;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{self, EventLoop};
use glium::Display;
use std::{fs, path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_path: String,

    #[arg(short, long, default_value_t = false)]
    skip_boot_rom: bool,
}

fn main() {
    env_logger::init();
    log::info!("Starting RustyFuuGBemu");
    let args = Args::parse();
    let rom_data = fs::read(path::Path::new(args.rom_path.as_str())).unwrap();

    let (events_loop, display) = init_glium();
    let mut egui_glium_impl = egui_glium::EguiGlium::new(&display, &events_loop);

    let gameboy = gameboy::Gameboy::new(rom_data);
    let gb_controller = gameboy.start();

    events_loop.run(move |ev, _, control_flow| {
        let mut redraw = |control_flow: &mut event_loop::ControlFlow| {
            let repaint_after = egui_glium_impl.run(&display, |_egui_ctx| {});

            *control_flow = if repaint_after.is_zero() {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui::Rgba::from_rgb(1.0, 1.0, 1.0);
                target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here

                egui_glium_impl.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

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

                _ => {
                    let event_response = egui_glium_impl.on_event(&event);

                    if event_response.repaint {
                        display.gl_window().window().request_redraw();
                    }
                }
            },
            Event::NewEvents(_) => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                redraw(control_flow);
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
