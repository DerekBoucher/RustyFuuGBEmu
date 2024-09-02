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
use env_logger::Env;
use gameboy::channel::front_end::Frontend;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::platform::unix::WindowBuilderExtUnix;
use glium::glutin::window::Theme;
use glium::Display;
use glium::{glutin, Surface};

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

fn main() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    log::info!("Starting RustyFuuGBemu");
    let args = Args::parse();

    let (program_loop, display) = init_glium();
    let egui_glium_client = egui_glium::EguiGlium::new(&display, &program_loop);
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

    program_loop.run(move |program_event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match program_event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => {
                    handle_app_close(control_flow, &mut frontend);
                }

                _ => ui.process_window_event(window_event, &display, &frontend),
            },
            Event::UserEvent(custom_event) => match custom_event {
                ui::events::UiEvent::CloseWindow => {
                    handle_app_close(control_flow, &mut frontend);
                }
            },
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(1.0, 1.0, 1.0, 1.0);
                opengl_renderer.render(&mut frame);
                ui.draw(control_flow, &display, &mut frame, &mut frontend);
                frame.finish().unwrap();
            }
            _ => {}
        }

        if Instant::now() >= next_frame_time {
            next_frame_time = Instant::now() + FRAME_INTERVAL;
            match frontend.should_render_screen() {
                Some(frame_data) => {
                    opengl_renderer.update_frame(&display, frame_data);
                }
                _ => {}
            }
            display.gl_window().window().request_redraw();
        }
    });
}

fn init_glium() -> (EventLoop<ui::events::UiEvent>, Display) {
    let events_loop =
        glium::glutin::event_loop::EventLoopBuilder::<ui::events::UiEvent>::with_user_event()
            .build();

    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            (ppu::NATIVE_SCREEN_WIDTH as i32) * ui::SCALE_FACTOR,
            ((ppu::NATIVE_SCREEN_HEIGHT as i32) * ui::SCALE_FACTOR) + ui::TOP_MENUBAR_HEIGHT as i32,
        ))
        .with_title("RustyFuuGBemu")
        .with_wayland_csd_theme(Theme::Dark)
        .with_resizable(true);

    let context = glium::glutin::ContextBuilder::new().with_hardware_acceleration(Some(true));

    let display = glium::Display::new(window_builder, context, &events_loop).unwrap();

    return (events_loop, display);
}

fn handle_app_close(
    control_flow: &mut glutin::event_loop::ControlFlow,
    gb_frontend: &mut Frontend,
) {
    *control_flow = glutin::event_loop::ControlFlow::Exit;
    gb_frontend.send_close_back_end();

    // Drain the frame data channel to avoid backend from blocking.
    // Not ideal, need to refactor this
    let _ = gb_frontend.should_render_screen();

    match gb_frontend.join_back_end() {
        Ok(_) => (),
        Err(err) => panic!("error occurred when joining back end thread: {:?}", err),
    }
}
