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
use glium::Display;
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::gameboy::channel::front_end::Frontend;
use crate::renderer::OpenGL;
use crate::ui::Ui;

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

struct App {
    front_end: Frontend,
    display: Display<WindowSurface>,
    window: Window,
    // TODO: Abstract renderer behind an interface to support multiple rendering technologies
    opengl_renderer: OpenGL,
    ui: Ui,
    last_frame_update: Instant,
}

impl ApplicationHandler<ui::events::UiEvent> for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let mut control_flow = event_loop.control_flow();

        match event {
            WindowEvent::CloseRequested => {
                handle_app_close(&mut control_flow, &mut self.front_end);
                event_loop.set_control_flow(control_flow);
                return;
            }

            WindowEvent::RedrawRequested => {
                // Drain the latest frame produced by the emulation thread and
                // upload the pixels to the GPU. The frame channel is bounded, so
                // this also unblocks the back end after it finishes a frame.
                if let Some(frame_data) = self.front_end.should_render_screen() {
                    self.opengl_renderer.update_frame(&self.display, frame_data);
                }

                let mut frame = self.display.draw();
                frame.clear_color(1.0, 1.0, 1.0, 1.0);
                self.opengl_renderer.render(&mut frame);
                self.ui.draw(
                    &mut control_flow,
                    &self.display,
                    &mut frame,
                    &mut self.front_end,
                    &self.window,
                );
                frame.finish().unwrap();
            }
            WindowEvent::Resized(size) => {
                // Keep the GL surface (framebuffer) in sync with the window's
                // physical size. This is critical when the DPI scale factor
                // changes (e.g. Wayland fractional scaling): egui maps pointer
                // positions from physical pixels using pixels_per_point, while
                // its painter derives the drawing coordinate space from the
                // framebuffer size. If these disagree, clicks are offset from
                // the drawn UI.
                self.display.resize((size.width, size.height));
                self.ui.process_window_event(
                    WindowEvent::Resized(size),
                    &self.display,
                    &self.window,
                    &self.front_end,
                );
            }
            _ => self
                .ui
                .process_window_event(event, &self.display, &self.window, &self.front_end),
        }

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: ui::events::UiEvent,
    ) {
        let mut control_flow = event_loop.control_flow();

        match event {
            ui::events::UiEvent::CloseWindow => {
                handle_app_close(&mut control_flow, &mut self.front_end);
                event_loop.exit();
            }
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        let _ = (event_loop, cause);

        // Request a redraw once per frame so the emulator/UI update at ~60 FPS.
        // The back end blocks on the bounded frame data channel until the front
        // end drains it, so this also paces the emulation.
        let now = Instant::now();
        if now - self.last_frame_update >= FRAME_DURATION {
            self.last_frame_update = now;
            self.window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
}

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
    let opengl_renderer = renderer::OpenGL::new(&display);

    let (gameboy, memory_ref) = gameboy::Gameboy::new(args.skip_boot_rom);

    let ui = ui::Ui::new(
        egui_glium_client,
        program_loop.create_proxy(),
        args.skip_boot_rom,
        memory_ref,
    );
    let front_end = gameboy.start();

    let mut app = App {
        front_end,
        display,
        window,
        opengl_renderer,
        ui,
        last_frame_update: Instant::now(),
    };

    program_loop.run_app(&mut app)
}

fn init_glium() -> (
    EventLoop<ui::events::UiEvent>,
    Display<WindowSurface>,
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
