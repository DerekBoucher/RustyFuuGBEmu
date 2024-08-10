use egui::{Context, RichText};
use glium::glutin::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::{
    gameboy::channel::front_end::Frontend,
    joypad::{ActionButton, DirectionButton},
};

pub struct Ui {
    show: bool,
    show_key_bind_window: bool,
    gb_control_png: &'static [u8],

    // Mapped input keys
    key_a: (VirtualKeyCode, ElementState),
    key_b: (VirtualKeyCode, ElementState),
    key_start: (VirtualKeyCode, ElementState),
    key_select: (VirtualKeyCode, ElementState),
    key_up: (VirtualKeyCode, ElementState),
    key_down: (VirtualKeyCode, ElementState),
    key_left: (VirtualKeyCode, ElementState),
    key_right: (VirtualKeyCode, ElementState),

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

            key_a: (VirtualKeyCode::A, ElementState::Released),
            key_b: (VirtualKeyCode::S, ElementState::Released),
            key_start: (VirtualKeyCode::D, ElementState::Released),
            key_select: (VirtualKeyCode::F, ElementState::Released),
            key_up: (VirtualKeyCode::Up, ElementState::Released),
            key_down: (VirtualKeyCode::Down, ElementState::Released),
            key_left: (VirtualKeyCode::Left, ElementState::Released),
            key_right: (VirtualKeyCode::Right, ElementState::Released),

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
                let image_data = image::load_from_memory(&self.gb_control_png).unwrap();
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

                ui.image(
                    &gb_controls_texture,
                    egui::vec2(image_size[0] as f32, image_size[1] as f32),
                );

                ui.horizontal_centered(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Up: ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &self.key_up).to_string())
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
                                    RichText::new(format_args!("{:?}", &self.key_down).to_string())
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
                                    RichText::new(format_args!("{:?}", &self.key_left).to_string())
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
                                        format_args!("{:?}", &self.key_right).to_string(),
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
                                    RichText::new(format_args!("{:?}", &self.key_a).to_string())
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
                                    RichText::new(format_args!("{:?}", &self.key_b).to_string())
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
                                        format_args!("{:?}", &self.key_start).to_string(),
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
                                        format_args!("{:?}", &self.key_select).to_string(),
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
        if !self.show_key_bind_window {}

        egui::Window::new("")
            .open(&mut self.show_key_bind_window)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.label(egui::RichText::new("Press a key to bind").size(16.0));
            });
    }

    pub fn process_window_event(&mut self, event: WindowEvent<'_>, frontend: &Frontend) {
        if self.show_key_bind_window {
            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state != ElementState::Pressed {
                        return;
                    }

                    if let Some(key) = input.virtual_keycode {
                        self.show_key_bind_window = false;

                        match self.key_to_modify {
                            (Some(direction), None) => match direction {
                                DirectionButton::Up => self.key_up = (key, ElementState::Released),
                                DirectionButton::Down => {
                                    self.key_down = (key, ElementState::Released)
                                }
                                DirectionButton::Left => {
                                    self.key_left = (key, ElementState::Released)
                                }
                                DirectionButton::Right => {
                                    self.key_right = (key, ElementState::Released)
                                }
                            },
                            (None, Some(action)) => match action {
                                ActionButton::A => self.key_a = (key, ElementState::Released),
                                ActionButton::B => self.key_b = (key, ElementState::Released),
                                ActionButton::Start => {
                                    self.key_start = (key, ElementState::Released)
                                }
                                ActionButton::Select => {
                                    self.key_select = (key, ElementState::Released)
                                }
                            },
                            _ => {}
                        }

                        self.key_to_modify = (None, None);
                    }
                }
                _ => {}
            }
            return;
        }

        // Else we are processing actual gameplay inputs
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let key_pressed = input.virtual_keycode.unwrap();

                if key_pressed == self.key_a.0 {
                    if input.state != self.key_a.1 {
                        self.key_a.1 = input.state;
                        frontend.send_joypad_data(None, Some(ActionButton::A), input.state);
                    }
                }

                if key_pressed == self.key_b.0 {
                    if input.state != self.key_b.1 {
                        self.key_b.1 = input.state;
                        frontend.send_joypad_data(None, Some(ActionButton::B), input.state);
                    }
                }

                if key_pressed == self.key_start.0 {
                    if input.state != self.key_start.1 {
                        self.key_start.1 = input.state;
                        frontend.send_joypad_data(None, Some(ActionButton::Start), input.state);
                    }
                }

                if key_pressed == self.key_select.0 {
                    if input.state != self.key_select.1 {
                        self.key_select.1 = input.state;
                        frontend.send_joypad_data(None, Some(ActionButton::Select), input.state);
                    }
                }

                if key_pressed == self.key_up.0 {
                    if input.state != self.key_up.1 {
                        self.key_up.1 = input.state;
                        frontend.send_joypad_data(Some(DirectionButton::Up), None, input.state);
                    }
                }

                if key_pressed == self.key_down.0 {
                    if input.state != self.key_down.1 {
                        self.key_down.1 = input.state;
                        frontend.send_joypad_data(Some(DirectionButton::Down), None, input.state);
                    }
                }

                if key_pressed == self.key_left.0 {
                    if input.state != self.key_left.1 {
                        self.key_left.1 = input.state;
                        frontend.send_joypad_data(Some(DirectionButton::Left), None, input.state);
                    }
                }

                if key_pressed == self.key_right.0 {
                    if input.state != self.key_right.1 {
                        self.key_right.1 = input.state;
                        frontend.send_joypad_data(Some(DirectionButton::Right), None, input.state);
                    }
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
    }
}
