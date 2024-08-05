use crate::gameboy;
use crate::joypad::ActionButton;
use crate::joypad::DirectionButton;

use egui::epaint::Shadow;
use egui::Visuals;
use glium::glutin::event;
use glium::glutin::event::ElementState;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event_loop::EventLoopProxy;
use glium::Display;
use glium::Frame;
use std::fs;

mod controls;
pub mod events;
use gameboy::channel::front_end::Frontend;

pub const TOP_MENUBAR_HEIGHT: f32 = 20.0;
pub const SCALE_FACTOR: i32 = 5;

pub struct Ui {
    egui_glium_client: egui_glium::EguiGlium,
    ui_event_loop_proxy: EventLoopProxy<events::UiEvent>,
    skip_boot_rom: bool,
    controls: controls::Ui,

    // Mapped input keys
    key_a: event::VirtualKeyCode,
    key_b: event::VirtualKeyCode,
    key_start: event::VirtualKeyCode,
    key_select: event::VirtualKeyCode,
    key_up: event::VirtualKeyCode,
    key_down: event::VirtualKeyCode,
    key_left: event::VirtualKeyCode,
    key_right: event::VirtualKeyCode,
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
            key_a: event::VirtualKeyCode::A,
            key_b: event::VirtualKeyCode::S,
            key_start: event::VirtualKeyCode::D,
            key_select: event::VirtualKeyCode::F,
            key_up: event::VirtualKeyCode::Up,
            key_down: event::VirtualKeyCode::Down,
            key_left: event::VirtualKeyCode::Left,
            key_right: event::VirtualKeyCode::Right,
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

            // Preferences window
            self.controls.render(
                egui_ctx,
                &mut self.key_a,
                &mut self.key_b,
                &mut self.key_start,
                &mut self.key_select,
                &mut self.key_up,
                &mut self.key_down,
                &mut self.key_left,
                &mut self.key_right,
            );

            // Top menubar
            egui::TopBottomPanel::top("top_panel")
                .exact_height(TOP_MENUBAR_HEIGHT)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Load ROM").clicked() {
                                Ui::load_rom_from_file_dialog(frontend);
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
                                frontend.send_set_skip_boot_rom_back_end(self.skip_boot_rom);
                            }

                            ui.separator();
                            if ui.button("Controls").clicked() {
                                self.controls.show(true);
                                ui.close_menu();
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

    pub fn process_window_event(
        &mut self,
        event: WindowEvent<'_>,
        display: &Display,
        frontend: &Frontend,
    ) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state != ElementState::Pressed {
                    return;
                }

                let key_pressed = input.virtual_keycode.unwrap();

                if key_pressed == self.key_a {
                    frontend.send_joypad_data(None, Some(ActionButton::A));
                }

                if key_pressed == self.key_b {
                    frontend.send_joypad_data(None, Some(ActionButton::B));
                }

                if key_pressed == self.key_start {
                    frontend.send_joypad_data(None, Some(ActionButton::Start));
                }

                if key_pressed == self.key_select {
                    frontend.send_joypad_data(None, Some(ActionButton::Select));
                }

                if key_pressed == self.key_up {
                    frontend.send_joypad_data(Some(DirectionButton::Up), None);
                }

                if key_pressed == self.key_down {
                    frontend.send_joypad_data(Some(DirectionButton::Down), None);
                }

                if key_pressed == self.key_left {
                    frontend.send_joypad_data(Some(DirectionButton::Left), None);
                }

                if key_pressed == self.key_right {
                    frontend.send_joypad_data(Some(DirectionButton::Right), None);
                }

                log::trace!(
                    "key scancode: {:?}, state: {:?}, virt: {:?}",
                    input.scancode,
                    input.state,
                    input.virtual_keycode.unwrap()
                );
            }
            _ => {}
        }

        let event_response = self.egui_glium_client.on_event(&event);

        if event_response.repaint {
            display.gl_window().window().request_redraw();
        }

        self.controls.process_window_event(event);
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
