mod cartridge;
mod cpu;
mod gameboy;
mod memory;
mod ppu;

extern crate glium;

use clap::Parser;
use glium::{glutin, Surface};
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
    let args = Args::parse();
    let rom_data = fs::read(path::Path::new(args.rom_path.as_str())).unwrap();

    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("RustyFuuGBemu");
    let cb = glium::glutin::ContextBuilder::new();
    let _display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let gameboy = gameboy::Gameboy::new(rom_data);
    let (gb_closer, gb_joiner) = gameboy.start();

    events_loop.run(move |ev, _, control_flow| {
        let mut target = _display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    gb_closer.send(()).unwrap();
                    gb_joiner
                        .recv_timeout(std::time::Duration::from_secs(5))
                        .unwrap();
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
