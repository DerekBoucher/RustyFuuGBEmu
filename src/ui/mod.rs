use crate::gameboy;
use crate::gpu::GpuContext;
use crate::memory::Memory;

use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

mod controls;
pub mod events;
mod vram_viewer;
use egui::Color32;
use egui::Shadow;
use egui::Visuals;
use gameboy::channel::front_end::Frontend;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

pub const TOP_MENUBAR_HEIGHT: f32 = 20.0;
pub const SCALE_FACTOR: i32 = 5;

/// A frame's worth of egui draw data, produced by [`Ui::prepare`].
pub struct UiFrame {
    paint_jobs: Vec<egui::ClippedPrimitive>,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
    textures_to_free: Vec<egui::TextureId>,
    command_buffers: Vec<wgpu::CommandBuffer>,
}

impl UiFrame {
    /// Command buffers that stage this frame's egui buffers; submit them
    /// together with the frame's own command buffer.
    pub fn take_command_buffers(&mut self) -> Vec<wgpu::CommandBuffer> {
        std::mem::take(&mut self.command_buffers)
    }
}

pub struct Ui {
    egui_context: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
    skip_boot_rom: bool,
    controls: controls::Ui,
    vram_viewer: vram_viewer::Ui,
    is_paused: bool,
    memory_ref: Arc<Mutex<Memory>>,
}

impl Ui {
    pub fn new(
        gpu: &GpuContext,
        window: &Window,
        event_loop_proxy: EventLoopProxy<events::UiEvent>,
        skip_boot_rom: bool,
        memory_ref: Arc<Mutex<Memory>>,
    ) -> Self {
        let egui_context = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_context.clone(),
            egui::ViewportId::ROOT,
            window,
            None,
            None,
            None,
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            &gpu.device,
            gpu.config.format,
            egui_wgpu::RendererOptions::default(),
        );

        Self {
            egui_context,
            egui_state,
            egui_renderer,
            ui_event_loop_proxy: event_loop_proxy,
            skip_boot_rom,
            controls: controls::Ui::new(),
            vram_viewer: vram_viewer::Ui::new(),
            is_paused: false,
            memory_ref,
        }
    }

    /// Runs egui for this frame and stages its buffers.
    ///
    /// The returned [`UiFrame`] is painted by [`Ui::paint`] once the frame's
    /// render pass is recording.
    pub fn prepare(
        &mut self,
        gpu: &GpuContext,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        frontend: &mut Frontend,
    ) -> UiFrame {
        let raw_input = self.egui_state.take_egui_input(window);
        let full_output = self.egui_context.run(raw_input, |ctx| {
            ctx.set_visuals(Visuals {
                window_shadow: Shadow::NONE,
                popup_shadow: Shadow::NONE,
                override_text_color: Some(Color32::WHITE),
                ..Default::default()
            });

            // Top menubar
            egui::TopBottomPanel::top("top_panel")
                .exact_height(TOP_MENUBAR_HEIGHT)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Load ROM").clicked() {
                                Ui::load_rom_from_file_dialog(frontend);

                                // When loading a rom, un-pause the emulator to avoid weirdness
                                self.is_paused = false;
                                frontend.send_pause(self.is_paused);

                                ui.close();
                            }

                            if ui.button("Exit").clicked() {
                                self.is_paused = false;
                                frontend.send_pause(self.is_paused);

                                if let Err(err) = self
                                    .ui_event_loop_proxy
                                    .send_event(events::UiEvent::CloseWindow)
                                {
                                    log::warn!(
                                        "error occurred sending close window event to event loop: {:?}",
                                        err
                                    );
                                }
                                ui.close();
                            }
                        });

                        ui.menu_button("Settings", |ui| {
                            if ui
                                .checkbox(&mut self.skip_boot_rom, "Skip Boot ROM")
                                .clicked()
                            {
                                frontend.send_set_skip_boot_rom_back_end(self.skip_boot_rom);
                            }

                            if ui.checkbox(&mut self.is_paused, "Pause").clicked() {
                                frontend.send_pause(self.is_paused);
                            }

                            ui.separator();
                            if ui.button("Controls").clicked() {
                                self.controls.show(true);
                                ui.close();
                            }
                        });

                        ui.menu_button("Debug", |ui| {
                            if ui.button("VRAM Viewer").clicked() {
                                self.vram_viewer.show(true);
                                ui.close();
                            }
                        });
                    });
                });

            // Controls window
            self.controls.render(ctx);

            // VRAM Viewer window
            self.vram_viewer.render(ctx, &self.memory_ref);
        });

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            ..
        } = full_output;
        self.egui_state
            .handle_platform_output(window, platform_output);

        let paint_jobs = self.egui_context.tessellate(shapes, pixels_per_point);

        let size = window.inner_size();
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point,
        };

        for (texture_id, image_delta) in &textures_delta.set {
            self.egui_renderer
                .update_texture(&gpu.device, &gpu.queue, *texture_id, image_delta);
        }

        let command_buffers = self.egui_renderer.update_buffers(
            &gpu.device,
            &gpu.queue,
            encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        UiFrame {
            paint_jobs,
            screen_descriptor,
            textures_to_free: textures_delta.free,
            command_buffers,
        }
    }

    /// Records the frame's egui draw calls.
    pub fn paint(&mut self, pass: &mut wgpu::RenderPass<'static>, frame: &UiFrame) {
        self.egui_renderer
            .render(pass, &frame.paint_jobs, &frame.screen_descriptor);
    }

    /// Releases the textures egui dropped while preparing `frame`.
    ///
    /// Must run after the frame's command buffers were submitted: destroying
    /// a texture that still has unsubmitted commands referencing it
    /// invalidates those commands.
    pub fn release_textures(&mut self, frame: &UiFrame) {
        for texture_id in &frame.textures_to_free {
            self.egui_renderer.free_texture(texture_id);
        }
    }

    pub fn process_window_event(
        &mut self,
        event: WindowEvent,
        window: &Window,
        frontend: &Frontend,
    ) {
        let _ = self.egui_state.on_window_event(window, &event);

        self.controls.process_window_event(event, frontend);
    }

    fn load_rom_from_file_dialog(frontend: &mut Frontend) {
        let selected_rom = rfd::FileDialog::new()
            .add_filter("Gameboy ROM", &[String::from("gb")])
            .pick_file();

        if let Some(rom_path) = selected_rom {
            match fs::read(rom_path.as_path()) {
                Ok(rom_data) => {
                    log::info!("Loaded ROM: {}", rom_path.display());
                    frontend.send_rom_data_back_end(rom_data);
                }
                Err(err) => {
                    // TODO: Add UI dialog indicating error
                    log::error!("Failed to load ROM: {}", err);
                }
            }
        }
    }
}
