use crate::cartridge;
use crate::cpu::CPU_CYCLES_PER_FRAME;
use crate::interface;
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
}

pub struct Orchestrator {
    close_sender: channel::Sender<()>,
    ack_receiver: channel::Receiver<()>,
    rom_data_sender: channel::Sender<Vec<u8>>,
    frame_data_receiver: channel::Receiver<
        [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
    >,
}

impl Gameboy {
    pub fn new() -> Self {
        return Self {
            state: State::INITIALIZING,
        };
    }

    fn load_rom(
        &mut self,
        rom_data: Vec<u8>,
        cpu: &mut impl interface::CPU,
        memory: &mut impl interface::Memory,
        ppu: &mut impl interface::PPU,
        timers: &mut impl interface::Timers,
    ) {
        cpu.reset();
        ppu.reset();
        timers.reset();
        memory.reset(cartridge::new(rom_data));
    }

    pub fn skip_boot_rom(
        &self,
        cpu: &mut impl interface::CPU,
        memory: &mut impl interface::Memory,
    ) {
        cpu.set_post_boot_rom_state();
        memory.set_post_boot_rom_state();
    }

    pub fn start(
        mut self,
        cpu: impl interface::CPU + 'static,
        memory: impl interface::Memory + 'static,
        ppu: impl interface::PPU + 'static,
        timers: impl interface::Timers + 'static,
    ) -> Orchestrator {
        let (orchestrator, close_receiver, ack_sender, rom_data_receiver, frame_data_sender) =
            Orchestrator::new();

        std::thread::spawn(move || {
            self.run(
                close_receiver,
                ack_sender,
                rom_data_receiver,
                frame_data_sender,
                cpu,
                memory,
                ppu,
                timers,
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
            [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
        >,

        mut cpu: impl interface::CPU,
        mut memory: impl interface::Memory,
        mut ppu: impl interface::PPU,
        mut timers: impl interface::Timers,
    ) {
        loop {
            match self.state {
                State::INITIALIZING => {
                    self.handle_initializing(
                        &close_receiver,
                        &rom_data_receiver,
                        &mut cpu,
                        &mut memory,
                        &mut ppu,
                        &mut timers,
                    );
                }
                State::COMPUTING => {
                    self.handle_computing(
                        &mut cpu,
                        &mut memory,
                        &mut ppu,
                        &mut timers,
                        &close_receiver,
                        &rom_data_receiver,
                    );
                }
                State::RENDERING => {
                    // Ask main thread to render the frame.
                    // Since the frame_data channel is bounded with a capacity of 1, we can
                    // also rely on this thread blocking until the main thread renders, which in
                    // turn allows the main thread to control the FPS of the emulator.
                    frame_data_sender.send(ppu.get_frame_data()).unwrap();
                    self.state.transition(State::COMPUTING);
                }
                State::EXITING => {
                    log::debug!("gb thread exited");
                    match ack_sender.send(()) {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("ack_sender channel closed before sending ack: {:?}", err,);
                        }
                    }
                    return;
                }
            }
        }
    }

    fn handle_initializing(
        &mut self,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
        cpu: &mut impl interface::CPU,
        memory: &mut impl interface::Memory,
        ppu: &mut impl interface::PPU,
        timers: &mut impl interface::Timers,
    ) {
        select! {
            recv(close_receiver) -> _ => {
                self.state.transition(State::EXITING);
            }

            recv(rom_data_receiver) -> rom_data => {
                self.load_rom(rom_data.unwrap(), cpu, memory, ppu, timers);
                log::debug!("ROM cartridge loaded!");
                self.state.transition(State::COMPUTING);
            }
        }
    }

    fn handle_computing(
        &mut self,
        cpu: &mut impl interface::CPU,
        memory: &mut impl interface::Memory,
        ppu: &mut impl interface::PPU,
        timers: &mut impl interface::Timers,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
    ) {
        let mut cycles_this_update: u32 = 0;
        while cycles_this_update < CPU_CYCLES_PER_FRAME {
            select! {
                recv(close_receiver) -> _ => {
                    self.state.transition(State::EXITING);
                    return;
                }

                recv(rom_data_receiver) -> rom_data => {
                    self.load_rom(rom_data.unwrap(), cpu, memory, ppu, timers);
                    log::debug!("ROM cartridge loaded!");
                    continue;
                }

                default => {
                    let cycles: u32;
                    if cpu.is_halted() {
                        cycles = 4;
                        cpu.halt(memory);
                    } else {
                        cycles = cpu.execute_next_opcode(memory);
                    }

                    cycles_this_update += cycles;

                    memory.update_dma_transfer_cycles(cycles);
                    timers.update(cycles, memory, cpu);
                    ppu.update_graphics(cycles, memory, cpu);
                    cpu.process_interrupts(memory, timers);
                }
            }
        }
        self.state.transition(State::RENDERING);
    }
}
