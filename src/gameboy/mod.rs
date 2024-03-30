use crate::cartridge;
use crate::cpu;
use crate::cpu::CPU_CYCLES_PER_FRAME;
use crate::interface;
use crate::interface::Memory;
use crate::interface::Timers;
use crate::interface::CPU;
use crate::interface::PPU;
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
    cpu: cpu::LR35902,
    memory: memory::Memory,
    ppu: ppu::PPU,
    timers: timers::Timers,
}

pub struct Orchestrator {
    close_sender: channel::Sender<()>,
    ack_receiver: channel::Receiver<()>,
    rom_data_sender: channel::Sender<Vec<u8>>,
    frame_data_receiver: channel::Receiver<
        [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
    >,
    skip_boot_rom_sender: channel::Sender<bool>,
}

impl Gameboy {
    pub fn new(
        cpu: cpu::LR35902,
        memory: memory::Memory,
        ppu: ppu::PPU,
        timers: timers::Timers,
        skip_boot_rom: bool,
    ) -> Self {
        return Self {
            state: State::INITIALIZING,
            cpu,
            memory,
            ppu,
            timers,
            skip_boot_rom,
        };
    }

    fn load_rom(&mut self, rom_data: Vec<u8>) {
        self.cpu.reset();
        self.ppu = ppu::PPU::new();
        self.timers = timers::Timers::new();
        self.memory = memory::Memory::new(cartridge::new(rom_data));

        if self.skip_boot_rom {
            self.cpu.set_post_boot_rom_state();
            self.memory.set_post_boot_rom_state();
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
            [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
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
                    self.render(&frame_data_sender);
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

    fn initialize(
        &mut self,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
        skip_boot_rom_receiver: &channel::Receiver<bool>,
    ) {
        select! {
            recv(close_receiver) -> _ => {
                self.state.transition(State::EXITING);
            }

            recv(skip_boot_rom_receiver) -> skip_boot_rom => {
                self.skip_boot_rom = skip_boot_rom.unwrap();
            }

            recv(rom_data_receiver) -> rom_data => {
                self.load_rom(rom_data.unwrap());
                log::debug!("ROM cartridge loaded!");
                self.state.transition(State::COMPUTING);
            }
        }
    }

    fn compute(
        &mut self,
        close_receiver: &channel::Receiver<()>,
        rom_data_receiver: &channel::Receiver<Vec<u8>>,
        skip_boot_rom_receiver: &channel::Receiver<bool>,
    ) {
        let mut cycles_this_update: u32 = 0;
        while cycles_this_update < CPU_CYCLES_PER_FRAME {
            select! {
                recv(close_receiver) -> _ => {
                    self.state.transition(State::EXITING);
                    return;
                }

                recv(skip_boot_rom_receiver) -> skip_boot_rom => {
                    self.skip_boot_rom = skip_boot_rom.unwrap();
                }

                recv(rom_data_receiver) -> rom_data => {
                    self.load_rom(rom_data.unwrap());
                    log::debug!("ROM cartridge loaded!");
                    continue;
                }

                default => {
                    let cycles: u32;
                    if self.cpu.is_halted() {
                        cycles = 4;
                        self.cpu.halt(&mut self.memory);
                    } else {
                        cycles = self.cpu.execute_next_opcode(&mut self.memory);
                    }

                    cycles_this_update += cycles;

                    self.memory.update_dma_transfer_cycles(cycles);
                    self.timers.update(cycles, &mut self.memory, &mut self.cpu);
                    self.ppu.update_graphics(cycles, &mut self.memory, &mut self.cpu);
                    self.cpu.process_interrupts(&mut self.memory,&mut self.timers);
                }
            }
        }
        self.state.transition(State::RENDERING);
    }

    fn render(
        &mut self,
        frame_data_sender: &channel::Sender<
            [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
        >,
    ) {
        // Ask main thread to render the frame.
        // Since the frame_data channel is bounded with a capacity of 1, we can
        // also rely on this thread blocking until the main thread renders, which in
        // turn allows the main thread to control the FPS of the emulation.
        frame_data_sender.send(self.ppu.get_frame_data()).unwrap();
        self.state.transition(State::COMPUTING);
    }
}
