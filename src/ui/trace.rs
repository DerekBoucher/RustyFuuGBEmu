use crate::cpu::register;
use crossbeam::channel;

pub struct Tool {
    pub open: bool,
    logs: String,
    is_tracing: bool,
    cpu_rx: channel::Receiver<CPUTrace>,
    mem_rx: channel::Receiver<MemoryTrace>,
    trace_sig_tx: channel::Sender<TraceSignal>,
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

pub struct TraceSignal {
    pub trace_id: uuid::Uuid,
    pub signal: Signal,
}

pub struct CPUTrace {
    pub trace_id: uuid::Uuid,
    pub op_code: u8,
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

pub enum Signal {
    Start,
    Stop,
}

pub struct MemoryTrace {
    pub trace_id: uuid::Uuid,
    pub address: u16,
    pub data: u8,
    pub operation: MemoryOperation,
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
    pub fn new() -> (
        Self,
        channel::Sender<CPUTrace>,
        channel::Sender<MemoryTrace>,
        channel::Receiver<TraceSignal>,
    ) {
        let (cpu_tx, cpu_rx) = channel::unbounded::<CPUTrace>();
        let (mem_tx, mem_rx) = channel::unbounded::<MemoryTrace>();
        let (trace_sig_tx, trace_sig_rx) = channel::unbounded::<TraceSignal>();
        (
            Self {
                open: false,
                address: 0x0000,
                register: register::ID::A,
                logs: String::new(),
                cpu_rx,
                mem_rx,
                trace_sig_tx,
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
            cpu_tx,
            mem_tx,
            trace_sig_rx,
        )
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
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

                                match self.trace_sig_tx.send(TraceSignal {
                                    trace_id: uuid::Uuid::new_v4(),
                                    signal: match self.is_tracing {
                                        true => Signal::Start,
                                        false => Signal::Stop,
                                    },
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        log::error!("Error sending trace signal: {:?}", err);
                                    }
                                }

                                if self.is_tracing && self.logs != "" {
                                    self.logs.clear();
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

        if self.is_tracing {
            match self.cpu_rx.try_recv() {
                Ok(trace) => {
                    self.logs.push_str(format!("CPU: [a: 0x{:02X}, b: 0x{:02X}, c: 0x{:02X}, d: 0x{:02X}, e: 0x{:02X}, h: 0x{:02X}, l: 0x{:02X}, f: 0x{:02X}, pc: 0x{:04X}, sp: 0x{:04X}, op_code: 0x{:02X}]\n",
                        trace.a, trace.b, trace.c, trace.d, trace.e, trace.h, trace.l, trace.f, trace.pc, trace.sp, trace.op_code).as_str());
                }
                Err(_) => {}
            }
        }
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
