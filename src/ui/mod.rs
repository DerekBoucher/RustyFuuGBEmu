use crate::gameboy;

use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event_loop::EventLoopProxy;
use glium::Display;
use glium::Frame;
use std::fs;

pub mod events;

pub const TOP_MENUBAR_HEIGHT: f32 = 20.0;
pub const SCALE_FACTOR: i32 = 5;

pub struct Ui {
    egui_glium_client: egui_glium::EguiGlium,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
    skip_boot_rom: bool,
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
        }
    }

    pub fn draw(
        &mut self,
        control_flow: &mut ControlFlow,
        display: &Display,
        frame: &mut Frame,
        gb_controller: &mut gameboy::Orchestrator,
    ) {
        let egui_redraw_timer = self.egui_glium_client.run(display, |egui_ctx| {
            egui::TopBottomPanel::top("top_panel")
                .exact_height(TOP_MENUBAR_HEIGHT)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Load ROM").clicked() {
                                Ui::load_rom_from_file_dialog(gb_controller);
                                ui.close_menu();
                            }

                            if ui.button("Exit").clicked() {
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
                                gb_controller.set_skip_boot_rom(self.skip_boot_rom);
                            }
                        })
                    });
                });
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

    pub fn process_window_event(&mut self, event: WindowEvent<'_>, display: &Display) {
        let event_response = self.egui_glium_client.on_event(&event);

        if event_response.repaint {
            display.gl_window().window().request_redraw();
        }
    }

    fn load_rom_from_file_dialog(gb_controller: &mut gameboy::Orchestrator) {
        let selected_rom = rfd::FileDialog::new()
            .add_filter("Gameboy ROM", &["gb"])
            .pick_file();

        match selected_rom {
            Some(rom_path) => match fs::read(rom_path.as_path()) {
                Ok(rom_data) => {
                    gb_controller.load_rom(rom_data);
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
