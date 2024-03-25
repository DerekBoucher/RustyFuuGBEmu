use egui::FontFamily;

use crate::cpu::register;

pub struct Tool {
    pub open: bool,
    logs: String,
    is_tracing: bool,
    trace_recv: crossbeam::channel::Receiver<Trace>,
    options: TraceOptions,
    address: u16,
    register: register::ID,
    stop_condition: StopCondition,

    stop_address_written: u16,
    stop_address_read: u16,
    stop_register_written: register::ID,
    stop_register_read: register::ID,
}

struct TraceOptions {
    address_tracing: bool,
    register_tracing: bool,
}

pub struct Trace {
    pub op_code: u8,
    pub op_code_name: String,
    pub address: u16,
    pub value: Option<u8>,
}

#[derive(Debug, PartialEq)]
enum StopCondition {
    None,
    AddressWritten,
    AddressRead,
    RegisterWritten,
    RegisterRead,
}

impl std::fmt::Display for StopCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {
                write!(f, "None")
            }
            Self::AddressRead => {
                write!(f, "Address read")
            }
            Self::AddressWritten => {
                write!(f, "Address written")
            }
            Self::RegisterRead => {
                write!(f, "Register read")
            }
            Self::RegisterWritten => {
                write!(f, "Register written")
            }
        }
    }
}

impl Tool {
    pub fn new() -> (Self, crossbeam::channel::Sender<Trace>) {
        let (tx, rx) = crossbeam::channel::unbounded::<Trace>();
        (
            Self {
                open: false,
                address: 0x0000,
                register: register::ID::A,
                logs: String::new(),
                trace_recv: rx,
                is_tracing: false,
                options: TraceOptions {
                    address_tracing: false,
                    register_tracing: false,
                },
                stop_condition: StopCondition::None,
                stop_address_written: 0x0000,
                stop_address_read: 0x0000,
                stop_register_written: register::ID::A,
                stop_register_read: register::ID::A,
            },
            tx,
        )
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        egui::Window::new("Trace Tool")
            .default_size(egui::vec2(500.0, 500.0))
            .resizable(true)
            .constrain(true)
            .scroll2([true, true])
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.address_tracing, "Trace Address");
                    ui.set_enabled(self.options.address_tracing);
                    Tool::address_picker(ui, &mut self.address);
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.register_tracing, "Trace Register");
                    ui.set_enabled(self.options.register_tracing);
                    Tool::register_picker("reg_picker_1".to_string(), ui, &mut self.register)
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Stop condition: ");
                    egui::ComboBox::from_id_source("Hash2")
                        .selected_text(format!("{}", self.stop_condition))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.stop_condition,
                                StopCondition::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut self.stop_condition,
                                StopCondition::AddressRead,
                                "Address Read",
                            );
                            ui.selectable_value(
                                &mut self.stop_condition,
                                StopCondition::AddressWritten,
                                "Address Written",
                            );
                            ui.selectable_value(
                                &mut self.stop_condition,
                                StopCondition::RegisterRead,
                                "Register Read",
                            );
                            ui.selectable_value(
                                &mut self.stop_condition,
                                StopCondition::RegisterWritten,
                                "Register Written",
                            );
                        });

                    ui.add_enabled_ui(self.stop_condition != StopCondition::None, |ui| {
                        ui.separator();

                        match self.stop_condition {
                            StopCondition::AddressRead => {
                                ui.horizontal(|ui| {
                                    ui.label("Address: ");
                                    Tool::address_picker(ui, &mut self.stop_address_read);
                                });
                            }
                            StopCondition::AddressWritten => {
                                ui.horizontal(|ui| {
                                    ui.label("Address: ");
                                    Tool::address_picker(ui, &mut self.stop_address_written)
                                });
                            }
                            StopCondition::RegisterRead => {
                                ui.horizontal(|ui| {
                                    ui.label("Register: ");
                                    Tool::register_picker(
                                        "id".to_string(),
                                        ui,
                                        &mut self.stop_register_read,
                                    );
                                });
                            }
                            StopCondition::RegisterWritten => {
                                ui.horizontal(|ui| {
                                    ui.label("Register: ");
                                    Tool::register_picker(
                                        "id".to_string(),
                                        ui,
                                        &mut self.stop_register_written,
                                    );
                                });
                            }
                            _ => {}
                        }
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(
                        self.options.address_tracing || self.options.register_tracing,
                        |ui| {
                            let trace_button_label = match self.is_tracing {
                                true => "Stop".to_string(),
                                false => "Trace".to_string(),
                            };

                            if ui.button(trace_button_label).clicked() {
                                self.is_tracing = true;
                            }
                        },
                    );
                });

                ui.add(
                    egui::TextEdit::multiline(&mut self.logs)
                        .desired_width(f32::INFINITY)
                        .text_color(egui::Color32::WHITE)
                        .interactive(false)
                        .desired_rows(30),
                );

                ui.add_space(5.0);
            });
    }

    fn register_picker(id: String, ui: &mut egui::Ui, reg: &mut register::ID) {
        egui::ComboBox::from_id_source(id)
            .selected_text(format!("{}", reg))
            .show_ui(ui, |ui| {
                ui.selectable_value(reg, register::ID::A, "A");
                ui.selectable_value(reg, register::ID::F, "F");
                ui.selectable_value(reg, register::ID::B, "B");
                ui.selectable_value(reg, register::ID::C, "C");
                ui.selectable_value(reg, register::ID::D, "D");
                ui.selectable_value(reg, register::ID::E, "E");
                ui.selectable_value(reg, register::ID::H, "H");
                ui.selectable_value(reg, register::ID::L, "L");
            });
    }

    fn address_picker(ui: &mut egui::Ui, address: &mut u16) {
        ui.add(
            egui::Slider::new(address, 0..=0xFFFF)
                .hexadecimal(4, false, false)
                .prefix("0x"),
        );
    }

    pub fn open(&mut self) {
        self.open = true;
    }
}
