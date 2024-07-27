use std::sync;
use std::sync::Arc;

use crate::cartridge;
use crate::cpu;
use crate::cpu::CPU_CYCLES_PER_FRAME;
use crate::interrupt;
use crate::memory;
use crate::ppu;
use crate::timers;
use crossbeam::channel;
use crossbeam::select;

mod orchestrator;

#[derive(Debug)]
enum State {
    INITIALIZING,
    COMPUTING,
    RENDERING,
    EXITING,
}

impl State {
    pub fn transition(&mut self, new_state: State) {
        log::trace!("State transition: {:?} -> {:?}", self, new_state);
        match self {
            State::INITIALIZING => match new_state {
                State::RENDERING => {
                    panic!("Invalid state transition: INITIALIZING -> {:?}", new_state)
                }
                _ => *self = new_state,
            },
            State::COMPUTING => match new_state {
                State::INITIALIZING => {
                    panic!("Invalid state transition: COMPUTING -> {:?}", new_state)
                }
                _ => *self = new_state,
            },
            State::RENDERING => match new_state {
                State::INITIALIZING => {
                    panic!("Invalid state transition: RENDERING -> {:?}", new_state)
                }
                _ => *self = new_state,
            },
            State::EXITING => panic!("Invalid state transition: EXITED -> {:?}", new_state),
        }
    }
}

pub struct Gameboy {
    state: State,
    skip_boot_rom: bool,
    memory: Arc<sync::Mutex<memory::Memory>>,
    cpu: Arc<sync::Mutex<cpu::LR35902>>,
    ppu: ppu::PPU,
    timers: Arc<sync::Mutex<timers::Timers>>,
    interrupt_bus: Arc<sync::Mutex<interrupt::Bus>>,
}

pub struct Orchestrator {
    close_sender: channel::Sender<()>,
    ack_receiver: channel::Receiver<()>,
    rom_data_sender: channel::Sender<Vec<u8>>,
    frame_data_receiver:
        channel::Receiver<[[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT]>,
    skip_boot_rom_sender: channel::Sender<bool>,
}

impl Gameboy {
    pub fn new(skip_boot_rom: bool) -> Self {
        let timers = Arc::new(sync::Mutex::new(timers::Timers::new()));
        let ppu = ppu::PPU::new();

        let interrupt_bus = Arc::new(sync::Mutex::new(interrupt::Bus::new()));
        let memory = Arc::new(sync::Mutex::new(memory::Memory::default(
            timers.clone(),
            interrupt_bus.clone(),
        )));
        let cpu = Arc::new(sync::Mutex::new(cpu::LR35902::new()));

        return Self {
            state: State::INITIALIZING,
            cpu,
            memory,
            ppu,
            skip_boot_rom,
            timers,
            interrupt_bus,
        };
    }

    fn load_rom(&mut self, rom_data: Vec<u8>) {
        self.cpu.lock().unwrap().reset();
        self.ppu.reset();
        self.memory.lock().unwrap().reset(cartridge::new(rom_data));
        self.timers.lock().unwrap().reset();
        self.interrupt_bus.lock().unwrap().reset();

        if self.skip_boot_rom {
            self.cpu.lock().unwrap().set_post_boot_rom_state();
            self.memory.lock().unwrap().set_post_boot_rom_state();
            self.timers.lock().unwrap().set_post_boot_rom_state();
            self.interrupt_bus.lock().unwrap().set_post_boot_rom_state();
        }
    }

    pub fn start(mut self) -> Orchestrator {
        let (
            orchestrator,
            close_receiver,
            ack_sender,
            rom_data_receiver,
            frame_data_sender,
            skip_boot_rom_recv,
        ) = Orchestrator::new();

        std::thread::spawn(move || {
            self.run(
                close_receiver,
                ack_sender,
                rom_data_receiver,
                frame_data_sender,
                skip_boot_rom_recv,
            )
        });

        return orchestrator;
    }

    fn run(
        &mut self,
        close_receiver: channel::Receiver<()>,
        ack_sender: channel::Sender<()>,
        rom_data_receiver: channel::Receiver<Vec<u8>>,
        frame_data_sender: channel::Sender<
            [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
        >,
        skip_boot_rom_recv: channel::Receiver<bool>,
    ) {
        loop {
            match self.state {
                State::INITIALIZING => {
                    self.initialize(&close_receiver, &rom_data_receiver, &skip_boot_rom_recv);
                }
                State::COMPUTING => {
                    self.compute(&close_receiver, &rom_data_receiver, &skip_boot_rom_recv);
                }
                State::RENDERING => {
                    // Ask main thread to render the frame.
                    // Since the frame_data channel is bounded with a capacity of 1, we can
                    // also rely on this thread blocking until the main thread renders, which in
                    // turn allows the main thread to control the FPS of the emulation.
                    frame_data_sender.send(self.ppu.get_frame_data()).unwrap();
                    self.state.transition(State::COMPUTING);
                }
                State::EXITING => {
                    log::debug!("gb thread exited");
                    match ack_sender.send(()) {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!(
                                "ack_sender channel closed before sending ack: {:?}",
                                err.to_string()
                            );
                        }
                    }
                    return;
                }
            }
        }
    }

    fn initialize(
        &mut self,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
        skip_boot_rom_receiver: &channel::Receiver<bool>,
    ) {
        select! {
            recv(close_receiver) -> signal => {
                match signal {
                    Ok(_) => {
                        self.state.transition(State::EXITING);
                        return;
                    }
                    _ => {}
                }
            }

            recv(skip_boot_rom_receiver) -> signal => {
                match signal {
                    Ok(skip_boot_rom) => {
                        self.skip_boot_rom = skip_boot_rom;
                    }
                    Err(err) => {
                        log::error!("Failed to receive skip_boot_rom signal: {:?}", err.to_string());
                    }
                }
            }

            recv(rom_data_receiver) -> signal => {
                match signal {
                    Ok(rom_data) => {
                        self.load_rom(rom_data);
                        log::debug!("ROM cartridge loaded!");
                        self.state.transition(State::COMPUTING);
                    }
                    Err(err) => {
                        log::error!("Failed to receive ROM data: {:?}", err.to_string());
                    }
                }
            }
        }
    }

    fn compute(
        &mut self,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
        skip_boot_rom_receiver: &channel::Receiver<bool>,
    ) {
        let mut cycles_this_frame_so_far: u32 = 0;
        while cycles_this_frame_so_far < CPU_CYCLES_PER_FRAME {
            select! {
                recv(close_receiver) -> signal => {
                    match signal {
                        Ok(_) => {
                            self.state.transition(State::EXITING);
                            return;
                        }
                        _ => {}
                    }
                }

                recv(skip_boot_rom_receiver) -> signal => {
                    match signal {
                        Ok(skip_boot_rom) => {
                            self.skip_boot_rom = skip_boot_rom;
                        }
                        Err(err) => {
                            log::error!("Failed to receive skip_boot_rom signal: {:?}", err.to_string());
                        }
                    }
                }

                recv(rom_data_receiver) -> signal => {
                    match signal {
                        Ok(rom_data) => {
                            self.load_rom(rom_data);
                            log::debug!("ROM cartridge loaded!");
                        }
                        Err(err) => {
                            log::error!("Failed to receive ROM data: {:?}", err.to_string());
                        }
                    }
                }

                default => {
                    let step_fn = &mut || {
                        self.timers.lock().unwrap().step(&self.interrupt_bus);
                        self.memory.lock().unwrap().step_dma();
                        self.ppu.step_graphics(&self.memory, &self.interrupt_bus);
                    };

                    if self.cpu.lock().unwrap().is_stopped() {
                        // todo
                    }

                    // let cycles: u32;

                    if self.cpu.lock().unwrap().is_halted() {
                        // cycles = 4;
                        step_fn();
                        self.cpu.lock().unwrap().handle_halt(&self.interrupt_bus);
                    } else {
                        _ = self.
                            cpu.
                            lock().
                            unwrap().
                            execute_next_opcode(&self.memory, step_fn);

                            self.
                                cpu.
                                lock().
                                unwrap().
                                process_interrupts(&self.memory,
                                    &self.interrupt_bus,
                                    step_fn,
                                );
                    }


                    cycles_this_frame_so_far += self.timers.lock().unwrap().get_elapsed_cycles();
                }
            }
        }
        self.state.transition(State::RENDERING);
    }
}
