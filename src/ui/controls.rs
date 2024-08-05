use egui::{Context, RichText};
use glium::glutin::event::{ElementState, VirtualKeyCode, WindowEvent};

pub struct Ui {
    show: bool,
    show_key_bind_window: bool,
    gb_control_png: &'static [u8],
    pending_bind: *mut VirtualKeyCode,
}

impl Ui {
    pub fn new() -> Self {
        let gb_control_png = include_bytes!("./assets/gb_controls_cropped.png");

        Self {
            show: false,
            show_key_bind_window: false,
            gb_control_png,
            pending_bind: std::ptr::null_mut(),
        }
    }

    pub fn show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn render(
        &mut self,
        egui_ctx: &Context,
        key_a: &mut VirtualKeyCode,
        key_b: &mut VirtualKeyCode,
        key_start: &mut VirtualKeyCode,
        key_select: &mut VirtualKeyCode,
        key_up: &mut VirtualKeyCode,
        key_down: &mut VirtualKeyCode,
        key_left: &mut VirtualKeyCode,
        key_right: &mut VirtualKeyCode,
    ) {
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
                            ui.label(RichText::new("Up -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_up).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_up;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Down -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_down).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_down;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Left -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_left).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_left;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Right -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_right).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_right;
                            }
                        });
                    });

                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("A -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_a).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_a;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("B -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_b).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_b;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Start -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_start).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_start;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Select -> ").size(15.0));
                            if ui
                                .button(
                                    RichText::new(format_args!("{:?}", &key_select).to_string())
                                        .size(15.0),
                                )
                                .clicked()
                            {
                                self.show_key_bind_window = true;
                                self.pending_bind = key_select;
                            }
                        });
                    });
                });
            });
    }

    fn render_key_bind_window(&mut self, egui_ctx: &Context) {
        if !self.show_key_bind_window {
            self.pending_bind = std::ptr::null_mut();
        }

        egui::Window::new("")
            .open(&mut self.show_key_bind_window)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.label(egui::RichText::new("Press a key to bind").size(16.0));
            });
    }

    pub fn process_window_event(&mut self, event: WindowEvent<'_>) {
        if self.show_key_bind_window {
            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state != ElementState::Pressed {
                        return;
                    }

                    if self.pending_bind.is_null() {
                        return;
                    }

                    if let Some(key) = input.virtual_keycode {
                        unsafe {
                            *self.pending_bind = key;
                        }
                        self.show_key_bind_window = false;
                    }
                }
                _ => {}
            }
        }
    }
}
