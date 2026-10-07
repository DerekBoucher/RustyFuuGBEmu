mod cartridge;
mod cpu;
mod gameboy;
mod interrupt;
mod joypad;
mod memory;
mod ppu;
mod renderer;
mod timers;
mod ui;

use std::time::{Duration, Instant};

use clap::Parser;
use egui::ViewportId;
use egui_glium::EguiGlium;
use env_logger::Env;
use glium::glutin::surface::WindowSurface;
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::error::EventLoopError;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::gameboy::channel::front_end::Frontend;

const FPS: u64 = 60;
const FRAME_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / FPS);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_path: Option<String>,

    #[arg(short, long, default_value_t = true)]
    skip_boot_rom: bool,
}

fn main() -> Result<(), EventLoopError> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    log::info!("Starting RustyFuuGBemu");
    let args = Args::parse();

    let (program_loop, display, window) = init_glium();
    let egui_glium_client = EguiGlium::new(ViewportId::ROOT, &display, &window, &program_loop);
    let mut opengl_renderer = renderer::OpenGL::new(&display);

    let (gameboy, memory_ref) = gameboy::Gameboy::new(args.skip_boot_rom);

    let mut ui = ui::Ui::new(
        egui_glium_client,
        program_loop.create_proxy(),
        args.skip_boot_rom,
        memory_ref,
    );
    let mut frontend = gameboy.start();
    let mut next_frame_time = Instant::now() + FRAME_INTERVAL;

    program_loop.run(move |window_event, event_loop| {
        let mut control_flow = event_loop.control_flow();
        control_flow = ControlFlow::WaitUntil(next_frame_time);

        match window_event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => {
                    handle_app_close(&mut control_flow, &mut frontend);
                }

                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color(1.0, 1.0, 1.0, 1.0);
                    opengl_renderer.render(&mut frame);
                    ui.draw(
                        &mut control_flow,
                        &display,
                        &mut frame,
                        &mut frontend,
                        &window,
                    );
                    frame.finish().unwrap();
                }
                _ => ui.process_window_event(window_event, &display, &window, &frontend),
            },
            Event::UserEvent(custom_event) => match custom_event {
                ui::events::UiEvent::CloseWindow => {
                    handle_app_close(&mut control_flow, &mut frontend);
                    event_loop.exit();
                }
            },
            _ => {}
        }

        if Instant::now() >= next_frame_time {
            next_frame_time = Instant::now() + FRAME_INTERVAL;
            if let Some(frame_data) = frontend.should_render_screen() {
                opengl_renderer.update_frame(&display, frame_data);
            }
            window.request_redraw();
        }

        event_loop.set_control_flow(control_flow);
    })
}

fn init_glium() -> (
    EventLoop<ui::events::UiEvent>,
    glium::Display<WindowSurface>,
    Window,
) {
    let event_loop = EventLoop::<ui::events::UiEvent>::with_user_event()
        .build()
        .unwrap();

    let width = (ppu::NATIVE_SCREEN_WIDTH as u32) * ui::SCALE_FACTOR as u32;
    let height = (ppu::NATIVE_SCREEN_HEIGHT as u32 * (ui::SCALE_FACTOR as u32))
        + ui::TOP_MENUBAR_HEIGHT as u32;

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("RustyFuuGBEmu")
        .with_inner_size(width, height)
        .build(&event_loop);

    (event_loop, display, window)
}

fn handle_app_close(control_flow: &mut ControlFlow, gb_frontend: &mut Frontend) {
    *control_flow = ControlFlow::Wait;
    gb_frontend.send_close_back_end();

    // Drain the frame data channel to avoid backend from blocking.
    // Not ideal, need to refactor this
    let _ = gb_frontend.should_render_screen();

    match gb_frontend.join_back_end() {
        Ok(_) => (),
        Err(err) => panic!("error occurred when joining back end thread: {:?}", err),
    }
}
