use egui::{Context, RichText};
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{
    gameboy::channel::front_end::Frontend,
    joypad::{ActionButton, DirectionButton},
};

pub struct Ui {
    show: bool,
    show_key_bind_window: bool,
    gb_control_png: &'static [u8],

    // Mapped input keys
    key_a: (KeyCode, ElementState),
    key_b: (KeyCode, ElementState),
    key_start: (KeyCode, ElementState),
    key_select: (KeyCode, ElementState),
    key_up: (KeyCode, ElementState),
    key_down: (KeyCode, ElementState),
    key_left: (KeyCode, ElementState),
    key_right: (KeyCode, ElementState),

    // Key to modify
    key_to_modify: (Option<DirectionButton>, Option<ActionButton>),
}

impl Ui {
    pub fn new() -> Self {
        let gb_control_png = include_bytes!("./assets/gb_controls_cropped.png");

        Self {
            show: false,
            show_key_bind_window: false,
            gb_control_png,

            key_a: (KeyCode::KeyA, ElementState::Released),
            key_b: (KeyCode::KeyS, ElementState::Released),
            key_start: (KeyCode::KeyD, ElementState::Released),
            key_select: (KeyCode::KeyF, ElementState::Released),
            key_up: (KeyCode::ArrowUp, ElementState::Released),
            key_down: (KeyCode::ArrowDown, ElementState::Released),
            key_left: (KeyCode::ArrowLeft, ElementState::Released),
            key_right: (KeyCode::ArrowRight, ElementState::Released),

            key_to_modify: (None, None),
        }
    }

    pub fn show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn render(&mut self, egui_ctx: &Context) {
        self.render_key_bind_window(egui_ctx);

        if !self.show {
            self.show_key_bind_window = false;
        }

        egui::Window::new(egui::RichText::new("Controls").size(24.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .open(&mut self.show)
            .show(egui_ctx, |ui| {
                let image_data = image::load_from_memory(self.gb_control_png).unwrap();
                let image_rgba = image_data.to_rgba8();
                let image_size = [image_data.width() as usize, image_data.height() as usize];
                let image_rgba = image_rgba.as_flat_samples();

                let gb_controls_texture =
                    egui::ColorImage::from_rgba_unmultiplied(image_size, image_rgba.as_slice());

                let gb_controls_texture = egui_ctx.load_texture(
                    "gb_controls",
                    gb_controls_texture,
                    egui::TextureOptions::default(),
                );

                ui.image(&gb_controls_texture);

                ui.horizontal_centered(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Up: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", self.key_up.0).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (Some(DirectionButton::Up), None);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Down: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(
                                        format_args!("{:?}", self.key_down.0).to_string(),
                                    )
                                    .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (Some(DirectionButton::Down), None);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Left: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(
                                        format_args!("{:?}", self.key_left.0).to_string(),
                                    )
                                    .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (Some(DirectionButton::Left), None);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Right: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(
                                        format_args!("{:?}", self.key_right.0).to_string(),
                                    )
                                    .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (Some(DirectionButton::Right), None);
                            }
                        });
                    });

                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("A: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", self.key_a.0).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (None, Some(ActionButton::A));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("B: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", self.key_b.0).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (None, Some(ActionButton::B));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Start: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(
                                        format_args!("{:?}", self.key_start.0).to_string(),
                                    )
                                    .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (None, Some(ActionButton::Start));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Select: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(
                                        format_args!("{:?}", self.key_select.0).to_string(),
                                    )
                                    .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.key_to_modify = (None, Some(ActionButton::Select));
                            }
                        });
                    });
                });
            });
    }

    fn render_key_bind_window(&mut self, egui_ctx: &Context) {
        egui::Window::new("")
            .open(&mut self.show_key_bind_window)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.label(egui::RichText::new("Press a key to bind").size(16.0));
            });
    }

    pub fn process_window_event(&mut self, event: WindowEvent, frontend: &Frontend) {
        if self.show_key_bind_window {
            if let WindowEvent::KeyboardInput { event, .. } = event {
                if event.state != ElementState::Pressed {
                    return;
                }

                self.show_key_bind_window = false;

                let key: KeyCode = match event.physical_key {
                    PhysicalKey::Code(key_code) => key_code,
                    PhysicalKey::Unidentified(_) => return,
                };

                match self.key_to_modify {
                    (Some(direction), None) => match direction {
                        DirectionButton::Up => self.key_up = (key, ElementState::Released),
                        DirectionButton::Down => self.key_down = (key, ElementState::Released),
                        DirectionButton::Left => self.key_left = (key, ElementState::Released),
                        DirectionButton::Right => self.key_right = (key, ElementState::Released),
                    },
                    (None, Some(action)) => match action {
                        ActionButton::A => self.key_a = (key, ElementState::Released),
                        ActionButton::B => self.key_b = (key, ElementState::Released),
                        ActionButton::Start => self.key_start = (key, ElementState::Released),
                        ActionButton::Select => self.key_select = (key, ElementState::Released),
                    },
                    _ => {}
                }

                self.key_to_modify = (None, None);
            }
            return;
        }

        // Else we are processing actual gameplay inputs
        if let WindowEvent::KeyboardInput { event, .. } = event {
            let key_pressed = match event.physical_key {
                PhysicalKey::Code(key_code) => key_code,
                PhysicalKey::Unidentified(_) => return,
            };

            if key_pressed == self.key_a.0 && event.state != self.key_a.1 {
                self.key_a.1 = event.state;
                frontend.send_joypad_data(None, Some(ActionButton::A), event.state);
            }

            if key_pressed == self.key_b.0 && event.state != self.key_b.1 {
                self.key_b.1 = event.state;
                frontend.send_joypad_data(None, Some(ActionButton::B), event.state);
            }

            if key_pressed == self.key_start.0 && event.state != self.key_start.1 {
                self.key_start.1 = event.state;
                frontend.send_joypad_data(None, Some(ActionButton::Start), event.state);
            }

            if key_pressed == self.key_select.0 && event.state != self.key_select.1 {
                self.key_select.1 = event.state;
                frontend.send_joypad_data(None, Some(ActionButton::Select), event.state);
            }

            if key_pressed == self.key_up.0 && event.state != self.key_up.1 {
                self.key_up.1 = event.state;
                frontend.send_joypad_data(Some(DirectionButton::Up), None, event.state);
            }

            if key_pressed == self.key_down.0 && event.state != self.key_down.1 {
                self.key_down.1 = event.state;
                frontend.send_joypad_data(Some(DirectionButton::Down), None, event.state);
            }

            if key_pressed == self.key_left.0 && event.state != self.key_left.1 {
                self.key_left.1 = event.state;
                frontend.send_joypad_data(Some(DirectionButton::Left), None, event.state);
            }

            if key_pressed == self.key_right.0 && event.state != self.key_right.1 {
                self.key_right.1 = event.state;
                frontend.send_joypad_data(Some(DirectionButton::Right), None, event.state);
            }

            log::trace!("state: {:?}", event.state);
        }
    }
}
