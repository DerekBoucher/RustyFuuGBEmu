use crate::{
    cpu::{
        events::{Event, RegisterAccess, Trace},
        register,
    },
    gameboy,
};

pub struct Tool {
    pub open: bool,
    logs: String,
    is_tracing: bool,
    options: TraceOptions,
    address: u16,
    register: register::ID,
    stop_condition: StopCondition,

    stop_address_written: u16,
    stop_address_read: u16,
    stop_register_written: register::ID,
    stop_register_read: register::ID,

    trace_rx: Option<(uuid::Uuid, crossbeam::channel::Receiver<Trace>)>,
}

struct TraceOptions {
    address_tracing: bool,
    register_tracing: bool,
}

pub enum Signal {
    Start,
    Stop,
}

pub enum MemoryOperation {
    Read,
    Write,
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
    pub fn new() -> Self {
        Self {
            open: false,
            address: 0x0000,
            register: register::ID::A,
            logs: String::new(),
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
            trace_rx: None,
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, gb_orchestrator: &gameboy::Orchestrator) {
        egui::Window::new("Trace Tool")
            .default_size(egui::vec2(500.0, 500.0))
            .resizable(true)
            .constrain(true)
            .vscroll(false)
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
                                self.is_tracing = !self.is_tracing;

                                if self.is_tracing && self.logs != "" {
                                    self.logs.clear();
                                }

                                let sub_event =
                                    Event::Register(RegisterAccess::Read(self.register.clone()));

                                if self.is_tracing {
                                    let (tx, rx) = crossbeam::channel::unbounded::<Trace>();

                                    let sub_id = gb_orchestrator.subscribe(tx, sub_event);
                                    self.trace_rx = Some((sub_id, rx));
                                } else {
                                    match &self.trace_rx {
                                        Some(sub) => {
                                            gb_orchestrator.unsubscribe(sub_event, sub.0.clone());
                                        }
                                        None => {}
                                    }
                                    self.trace_rx = None;
                                }
                            }
                        },
                    );
                });

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .vscroll(true)
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.logs)
                                .desired_width(f32::INFINITY)
                                .text_color(egui::Color32::WHITE)
                                .interactive(false)
                                .desired_rows(25),
                        );
                    });

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
