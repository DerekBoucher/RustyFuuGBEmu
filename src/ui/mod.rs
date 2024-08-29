use crate::gameboy;

use egui::epaint::Shadow;
use egui::Visuals;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event_loop::EventLoopProxy;
use glium::Display;
use glium::Frame;
use std::fs;

mod controls;
pub mod events;
mod vram_viewer;
use gameboy::channel::front_end::Frontend;

pub const TOP_MENUBAR_HEIGHT: f32 = 20.0;
pub const SCALE_FACTOR: i32 = 5;

pub struct Ui {
    egui_glium_client: egui_glium::EguiGlium,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
    skip_boot_rom: bool,
    controls: controls::Ui,
    vram_viewer: vram_viewer::Ui,
    is_paused: bool,
}

impl Ui {
    pub fn new(
        egui_glium_client: egui_glium::EguiGlium,
        event_loop_proxy: EventLoopProxy<events::UiEvent>,
        skip_boot_rom: bool,
    ) -> Self {
        Self {
            egui_glium_client,
            ui_event_loop_proxy: event_loop_proxy,
            skip_boot_rom,
            controls: controls::Ui::new(),
            vram_viewer: vram_viewer::Ui::new(),
            is_paused: false,
        }
    }

    pub fn draw(
        &mut self,
        control_flow: &mut ControlFlow,
        display: &Display,
        frame: &mut Frame,
        frontend: &mut Frontend,
    ) {
        let egui_redraw_timer = self.egui_glium_client.run(display, |egui_ctx| {
            let mut visuals = Visuals::default();
            visuals.window_shadow = Shadow::NONE;
            visuals.popup_shadow = Shadow::NONE;
            egui_ctx.set_visuals(visuals);

            // Top menubar
            egui::TopBottomPanel::top("top_panel")
                .exact_height(TOP_MENUBAR_HEIGHT)
                .show(egui_ctx, |ui| {
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

                                self.ui_event_loop_proxy
                                    .send_event(events::UiEvent::CloseWindow)
                                    .unwrap();
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
            self.controls.render(egui_ctx);

            // VRAM Viewer window
            self.vram_viewer.render(egui_ctx);
        });

        let time_until_next_redraw = std::time::Instant::now().checked_add(egui_redraw_timer);

        if egui_redraw_timer.is_zero() {
            display.gl_window().window().request_redraw();
            *control_flow = ControlFlow::Poll;
        } else if time_until_next_redraw.is_some() {
            *control_flow = ControlFlow::WaitUntil(time_until_next_redraw.unwrap());
        } else {
            *control_flow = ControlFlow::Wait;
        }

        self.egui_glium_client.paint(display, frame);
    }

    pub fn process_window_event(
        &mut self,
        event: WindowEvent<'_>,
        display: &Display,
        frontend: &Frontend,
    ) {
        match event {
            _ => {}
        }

        let event_response = self.egui_glium_client.on_event(&event);

        if event_response.repaint {
            display.gl_window().window().request_redraw();
        }

        self.controls.process_window_event(event, frontend);
    }

    fn load_rom_from_file_dialog(frontend: &mut Frontend) {
        let selected_rom = rfd::FileDialog::new()
            .add_filter("Gameboy ROM", &["gb"])
            .pick_file();

        match selected_rom {
            Some(rom_path) => match fs::read(rom_path.as_path()) {
                Ok(rom_data) => {
                    log::info!("Loaded ROM: {}", rom_path.display());
                    frontend.send_rom_data_back_end(rom_data);
                }
                Err(err) => {
                    // TODO: Add UI dialog indicating error
                    log::error!("Failed to load ROM: {}", err);
                }
            },
            None => {}
        }
    }
}
