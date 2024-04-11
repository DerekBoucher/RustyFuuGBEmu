use crate::{
    cpu::{
        events::{Event, MemoryAccess, RegisterAccess, Trace},
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

    register_trace_rx: Option<(uuid::Uuid, crossbeam::channel::Receiver<Trace>)>,
    memory_trace_rx: Option<(uuid::Uuid, crossbeam::channel::Receiver<Trace>)>,
}

struct TraceOptions {
    address_tracing: bool,
    register_tracing: bool,
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
            register_trace_rx: None,
            memory_trace_rx: None,
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
                    ui.set_enabled(!self.is_tracing);
                    ui.checkbox(&mut self.options.address_tracing, "Trace Address");
                    ui.set_enabled(self.options.address_tracing);
                    Tool::address_picker(ui, &mut self.address);
                });

                ui.horizontal(|ui| {
                    ui.set_enabled(!self.is_tracing);
                    ui.checkbox(&mut self.options.register_tracing, "Trace Register");
                    ui.set_enabled(self.options.register_tracing);
                    Tool::register_picker("reg_picker_1".to_string(), ui, &mut self.register)
                });

                ui.separator();

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

                                let register_sub_event =
                                    Event::Register(RegisterAccess::Read(self.register.clone()));

                                let memory_sub_event =
                                    Event::Memory(MemoryAccess::Read(self.address));

                                if self.is_tracing {
                                    if self.options.register_tracing {
                                        let (reg_tx, reg_rx) =
                                            crossbeam::channel::unbounded::<Trace>();

                                        let sub_id =
                                            gb_orchestrator.subscribe(reg_tx, register_sub_event);
                                        self.register_trace_rx = Some((sub_id, reg_rx));
                                    }

                                    if self.options.address_tracing {
                                        let (mem_tx, mem_rx) =
                                            crossbeam::channel::unbounded::<Trace>();

                                        let sub_id =
                                            gb_orchestrator.subscribe(mem_tx, memory_sub_event);
                                        self.memory_trace_rx = Some((sub_id, mem_rx));
                                    }
                                } else {
                                    match &self.register_trace_rx {
                                        Some(sub) => {
                                            gb_orchestrator
                                                .unsubscribe(register_sub_event, sub.0.clone());
                                        }
                                        None => {}
                                    }
                                    self.register_trace_rx = None;

                                    match &self.memory_trace_rx {
                                        Some(sub) => {
                                            gb_orchestrator
                                                .unsubscribe(memory_sub_event, sub.0.clone());
                                        }
                                        None => {}
                                    }
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

        match &self.register_trace_rx {
            Some(sub) => {
                while let Ok(trace) = sub.1.try_recv() {
                    self.logs.push_str(&format!("pc: {}, sp: {}, a: {}, f: {}, b: {}, c: {}, d: {}, e: {}, h: {}, l: {}, op_code: {}, cpu_cycles: {}", trace.pc, trace.sp,
                trace.a, trace.f, trace.b, trace.c, trace.d, trace.e, trace.h, trace.l, trace.opcode, trace.cpu_cycles));
                }
            }
            None => {}
        }

        match &self.memory_trace_rx {
            Some(sub) => {
                while let Ok(trace) = sub.1.try_recv() {
                    self.logs.push_str(&format!("pc: {}, sp: {}, a: {}, f: {}, b: {}, c: {}, d: {}, e: {}, h: {}, l: {}, op_code: {}, cpu_cycles: {}", trace.pc, trace.sp,
                trace.a, trace.f, trace.b, trace.c, trace.d, trace.e, trace.h, trace.l, trace.opcode, trace.cpu_cycles));
                }
            }
            None => {}
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
