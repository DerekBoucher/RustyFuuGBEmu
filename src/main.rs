mod cartridge;
mod cpu;
mod gameboy;
mod interface;
mod memory;
mod ppu;
mod renderer;
mod timers;
mod ui;

use clap::Parser;
use env_logger::Env;
use gameboy::Orchestrator;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::platform::unix::WindowBuilderExtUnix;
use glium::glutin::window::Theme;
use glium::Display;
use glium::{glutin, Surface};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_path: Option<String>,

    #[arg(short, long, default_value_t = false)]
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

    let cpu = cpu::LR35902::new();
    let memory = memory::Memory::default();
    let ppu = ppu::PPU::new();
    let timers = timers::Timers::new();
    let gameboy = gameboy::Gameboy::new(cpu, memory, ppu, timers, args.skip_boot_rom);

    let (trace_tool, trace_tx) = ui::trace::Tool::new();

    let mut ui = ui::Ui::new(
        egui_glium_client,
        program_loop.create_proxy(),
        args.skip_boot_rom,
        trace_tool,
    );
    let mut gb_orchestrator = gameboy.start();

    program_loop.run(move |program_event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667); // Achieves 60fps

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match program_event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => {
                    handle_app_close(control_flow, &mut gb_orchestrator);
                }

                _ => ui.process_window_event(window_event, &display),
            },
            Event::UserEvent(custom_event) => match custom_event {
                ui::events::UiEvent::CloseWindow => {
                    handle_app_close(control_flow, &mut gb_orchestrator);
                }
            },
            Event::NewEvents(_) => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(1.0, 1.0, 1.0, 1.0);
                opengl_renderer.render(&mut frame);
                ui.draw(control_flow, &display, &mut frame, &mut gb_orchestrator);
                frame.finish().unwrap();
            }
            _ => {}
        }

        match gb_orchestrator.render_requested() {
            Some(frame_data) => {
                opengl_renderer.update_frame(&display, frame_data);
                display.gl_window().window().request_redraw();
            }
            None => (),
        }
    });
}

fn init_glium() -> (EventLoop<ui::events::UiEvent>, Display) {
    let events_loop =
        glium::glutin::event_loop::EventLoopBuilder::<ui::events::UiEvent>::with_user_event()
            .build();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            (interface::NATIVE_SCREEN_WIDTH as i32) * ui::SCALE_FACTOR,
            ((interface::NATIVE_SCREEN_HEIGHT as i32) * ui::SCALE_FACTOR)
                + ui::TOP_MENUBAR_HEIGHT as i32,
        ))
        .with_title("RustyFuuGBemu")
        .with_wayland_csd_theme(Theme::Dark)
        .with_resizable(true);
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    return (events_loop, display);
}

fn handle_app_close(
    control_flow: &mut glutin::event_loop::ControlFlow,
    gb_controller: &mut Orchestrator,
) {
    *control_flow = glutin::event_loop::ControlFlow::Exit;
    gb_controller.close();
    match gb_controller.join() {
        Ok(_) => (),
        Err(err) => match err {
            crossbeam::channel::RecvTimeoutError::Timeout => {
                log::error!("gb thread deadlocked -> join operation did not complete on time")
            }
            crossbeam::channel::RecvTimeoutError::Disconnected => {
                log::error!("lost connection to gb thread while waiting for a join")
            }
        },
    }
}
