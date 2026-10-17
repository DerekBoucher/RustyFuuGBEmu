use crate::gameboy;
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
use glium::glutin::surface::WindowSurface;
use glium::Display;
use glium::Frame;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

pub const TOP_MENUBAR_HEIGHT: f32 = 20.0;
pub const SCALE_FACTOR: i32 = 5;

pub struct Ui {
    egui_glium_client: egui_glium::EguiGlium,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
    skip_boot_rom: bool,
    controls: controls::Ui,
    vram_viewer: vram_viewer::Ui,
    is_paused: bool,
    memory_ref: Arc<Mutex<Memory>>,
}

impl Ui {
    pub fn new(
        egui_glium_client: egui_glium::EguiGlium,
        event_loop_proxy: EventLoopProxy<events::UiEvent>,
        skip_boot_rom: bool,
        memory_ref: Arc<Mutex<Memory>>,
    ) -> Self {
        Self {
            egui_glium_client,
            ui_event_loop_proxy: event_loop_proxy,
            skip_boot_rom,
            controls: controls::Ui::new(),
            vram_viewer: vram_viewer::Ui::new(),
            is_paused: false,
            memory_ref,
        }
    }

    pub fn draw(
        &mut self,
        _control_flow: &mut ControlFlow,
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        frontend: &mut Frontend,
        window: &Window,
    ) {
        self.egui_glium_client.run(window, |ctx| {
            let mut visuals = Visuals::default();
            visuals.window_shadow = Shadow::NONE;
            visuals.popup_shadow = Shadow::NONE;
            visuals.override_text_color = Some(Color32::WHITE);
            ctx.set_visuals(visuals);

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

                                ui.close_menu();
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
                                ui.close_menu();
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
                                ui.close_menu();
                            }
                        });

                        ui.menu_button("Debug", |ui| {
                            if ui.button("VRAM Viewer").clicked() {
                                self.vram_viewer.show(true);
                                ui.close_menu();
                            }
                        });
                    });
                });

            // Controls window
            self.controls.render(ctx);

            // VRAM Viewer window
            self.vram_viewer.render(ctx, &self.memory_ref);
        });

        self.egui_glium_client.paint(display, frame);
    }

    pub fn process_window_event(
        &mut self,
        event: WindowEvent,
        _display: &Display<WindowSurface>,
        window: &Window,
        frontend: &Frontend,
    ) {
        let _ = self.egui_glium_client.on_event(window, &event);

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
