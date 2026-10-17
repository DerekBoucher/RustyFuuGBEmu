mod cartridge;
mod cpu;
mod gameboy;
mod gpu;
mod interrupt;
mod joypad;
mod memory;
mod ppu;
mod renderer;
mod timers;
mod ui;

use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use clap::Parser;
use env_logger::Env;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::Window;

use crate::gameboy::channel::front_end::Frontend;
use crate::gpu::{Backend, GpuContext};
use crate::memory::Memory;
use crate::renderer::Renderer;
use crate::ui::Ui;

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

/// Window, GPU and UI resources.
///
/// Created in `resumed`, as windows may only be created while the event loop
/// is running.
struct AppState {
    // Fields are dropped in declaration order. The UI's clipboard borrows the
    // window's display connection, so `ui` goes before `window`; `gpu` holds
    // the surface, which keeps the window alive through its own `Arc` until
    // the reference count reaches zero.
    ui: Ui,
    renderer: Box<dyn Renderer>,
    gpu: GpuContext,
    window: Arc<Window>,
}

struct App {
    front_end: Frontend,
    // Kept until `resumed`, where they are used to construct the UI.
    ui_event_loop_proxy: EventLoopProxy<ui::events::UiEvent>,
    skip_boot_rom: bool,
    /// Backend wgpu is restricted to, from `--renderer`.
    backend: Backend,
    memory_ref: Arc<Mutex<Memory>>,
    state: Option<AppState>,
    last_frame_update: Instant,
    is_closing: bool,
}

impl App {
    fn close(&mut self, event_loop: &ActiveEventLoop) {
        if self.is_closing {
            return;
        }
        self.is_closing = true;

        self.front_end.send_close_back_end();

        // Drain the frame data channel to avoid backend from blocking.
        // Not ideal, need to refactor this
        let _ = self.front_end.should_render_screen();

        if let Err(err) = self.front_end.join_back_end() {
            log::warn!("error occurred when joining back end thread: {:?}", err);
        }

        event_loop.exit();
    }
}

impl ApplicationHandler<ui::events::UiEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            // `resumed` fires again when a suspended application comes back;
            // everything is already set up at that point.
            return;
        }

        let width = (ppu::NATIVE_SCREEN_WIDTH as u32) * ui::SCALE_FACTOR as u32;
        let height = (ppu::NATIVE_SCREEN_HEIGHT as u32 * (ui::SCALE_FACTOR as u32))
            + ui::TOP_MENUBAR_HEIGHT as u32;

        let window = match event_loop.create_window(
            Window::default_attributes()
                .with_title("RustyFuuGBemu")
                .with_inner_size(PhysicalSize::new(width, height)),
        ) {
            Ok(window) => Arc::new(window),
            Err(err) => {
                log::error!("failed to create the window: {err}");
                std::process::exit(1);
            }
        };

        let gpu = match GpuContext::new(window.clone(), self.backend) {
            Ok(gpu) => gpu,
            Err(err) => {
                log::error!("failed to initialize wgpu: {err}");
                std::process::exit(1);
            }
        };

        let renderer: Box<dyn Renderer> = Box::new(renderer::WgpuRenderer::new(&gpu));

        let ui = Ui::new(
            &gpu,
            &window,
            self.ui_event_loop_proxy.clone(),
            self.skip_boot_rom,
            self.memory_ref.clone(),
        );

        self.state = Some(AppState { ui, renderer, gpu, window });

        // The frame timer in `new_events` runs off `StartCause::Poll`, so poll
        // continuously from the start.
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.close(event_loop);
            }

            WindowEvent::RedrawRequested => {
                let Some(state) = &mut self.state else {
                    return;
                };

                // Drain the frame produced by the emulation thread, if any,
                // and upload it to the renderer. The back end blocks on the
                // bounded channel until the front end consumes a frame, so
                // this also paces the emulation.
                if let Some(frame_data) = self.front_end.should_render_screen() {
                    state.renderer.update_frame(&state.gpu.queue, frame_data);
                }

                let size = state.window.inner_size();
                if size.width == 0 || size.height == 0 {
                    // Nothing drawable (e.g. a minimized window).
                    return;
                }

                let mut encoder =
                    state
                        .gpu
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("frame encoder"),
                        });

                let mut ui_frame = state.ui.prepare(
                    &state.gpu,
                    &state.window,
                    &mut encoder,
                    &mut self.front_end,
                );

                let surface_texture = match state.gpu.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        // The swapchain no longer matches the window; rebuild
                        // it and let the next frame redraw.
                        state.gpu.reconfigure();
                        return;
                    }
                    Err(err) => {
                        // Transient (e.g. `Timeout`); skip this frame and try
                        // again on the next redraw.
                        log::warn!("dropped frame while acquiring the surface texture: {err}");
                        return;
                    }
                };

                let view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut pass = encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("frame render pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 1.0,
                                    g: 1.0,
                                    b: 1.0,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    })
                    .forget_lifetime();

                // The Game Boy screen is drawn first; the UI overlays it,
                // e.g. with the top menubar.
                state.renderer.render(&mut pass);
                state.ui.paint(&mut pass, &ui_frame);
                drop(pass);

                let mut command_buffers = ui_frame.take_command_buffers();
                command_buffers.push(encoder.finish());
                state.gpu.queue.submit(command_buffers);

                surface_texture.present();

                state.ui.release_textures(&ui_frame);
            }

            WindowEvent::Resized(size) => {
                let Some(state) = &mut self.state else {
                    return;
                };

                // Keep the swapchain in sync with the window's physical size.
                // This is critical when the DPI scale factor changes (e.g.
                // Wayland fractional scaling): egui maps pointer positions
                // using pixels_per_point and derives the drawing coordinate
                // space from the swapchain size. If these disagree, clicks are
                // offset from the drawn UI.
                state.gpu.resize(size);
                state.ui.process_window_event(
                    WindowEvent::Resized(size),
                    &state.window,
                    &self.front_end,
                );
            }

            _ => {
                let Some(state) = &mut self.state else {
                    return;
                };

                state
                    .ui
                    .process_window_event(event, &state.window, &self.front_end);
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ui::events::UiEvent) {
        match event {
            ui::events::UiEvent::CloseWindow => {
                self.close(event_loop);
            }
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        // Request a redraw once per frame so the emulator/UI update at ~60 FPS.
        // The back end blocks on the bounded frame data channel until the front
        // end drains it, so this also paces the emulation.
        let Some(state) = &self.state else {
            return;
        };

        let now = Instant::now();
        if now - self.last_frame_update >= FRAME_DURATION {
            self.last_frame_update = now;
            state.window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
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

    /// Rendering backend to use.
    #[arg(long, value_enum, default_value_t = Backend::Auto)]
    renderer: Backend,
}

fn main() -> Result<(), EventLoopError> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    log::info!("Starting RustyFuuGBemu");
    let args = Args::parse();

    let program_loop = EventLoop::<ui::events::UiEvent>::with_user_event().build()?;

    let (gameboy, memory_ref) = gameboy::Gameboy::new(args.skip_boot_rom);
    let front_end = gameboy.start();

    let mut app = App {
        front_end,
        ui_event_loop_proxy: program_loop.create_proxy(),
        skip_boot_rom: args.skip_boot_rom,
        backend: args.renderer,
        memory_ref,
        state: None,
        last_frame_update: Instant::now(),
        is_closing: false,
    };

    program_loop.run_app(&mut app)
}
