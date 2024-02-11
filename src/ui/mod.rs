use crate::gameboy;
use egui::Context;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event_loop::EventLoopProxy;
use glium::Display;
use glium::Surface as _;
use std::fs;

pub mod events;

pub struct Ui {
    egui_glium_client: egui_glium::EguiGlium,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
}

impl Ui {
    pub fn new(
        egui_glium_client: egui_glium::EguiGlium,
        event_loop_proxy: EventLoopProxy<events::UiEvent>,
    ) -> Self {
        Self {
            egui_glium_client,
            ui_event_loop_proxy: event_loop_proxy,
        }
    }

    pub fn render(
        &mut self,
        control_flow: &mut ControlFlow,
        display: &Display,
        gb_controller: &mut gameboy::Orchestrator,
    ) {
        let egui_redraw_timer = self.egui_glium_client.run(&display, |egui_ctx| {
            Ui::draw(egui_ctx, gb_controller, &self.ui_event_loop_proxy);
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

        let mut target = display.draw();
        let color = egui::Rgba::from_rgb(1.0, 1.0, 1.0);
        target.clear_color(color[0], color[1], color[2], color[3]);

        // draw things behind egui here

        self.egui_glium_client.paint(&display, &mut target);

        // draw things on top of egui here

        target.finish().unwrap();
    }

    pub fn process_events(&mut self, event: WindowEvent<'_>, display: &Display) {
        let event_response = self.egui_glium_client.on_event(&event);

        if event_response.repaint {
            display.gl_window().window().request_redraw();
        }
    }

    fn draw(
        egui_ctx: &Context,
        gb_controller: &mut gameboy::Orchestrator,
        ui_event_loop_proxy: &EventLoopProxy<events::UiEvent>,
    ) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load ROM").clicked() {
                    gb_controller.pause();
                    Ui::load_rom_from_file_dialog(gb_controller);
                    ui.close_menu();
                    gb_controller.resume();
                }

                if ui.button("Exit").clicked() {
                    ui_event_loop_proxy
                        .send_event(events::UiEvent::CloseWindow)
                        .unwrap();
                    ui.close_menu();
                }
            })
        });
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
