use std::sync;
use std::sync::Arc;

use crate::cartridge;
use crate::cpu;
use crate::cpu::CPU_CYCLES_PER_FRAME;
use crate::interrupt;
use crate::memory;
use crate::ppu;
use crate::timers;

pub mod channel;
use channel::back_end::Backend;
use channel::front_end::Frontend;

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

    pub fn start(mut self) -> Frontend {
        let (frontend, backend) = channel::new();
        let _ = std::thread::spawn(move || self.run(backend));
        return frontend;
    }

    fn run(&mut self, backend: Backend) {
        loop {
            match self.state {
                State::INITIALIZING => {
                    self.initialize(&backend);
                }
                State::COMPUTING => {
                    self.compute(&backend);
                }
                State::RENDERING => {
                    // Ask the front end to render the frame.
                    // Since the frame_data channel is bounded with a capacity of 1, we can
                    // also rely on this thread blocking until the main thread renders, which in
                    // turn allows the main thread to control the FPS of the emulation.
                    backend.send_frame_data_front_end(self.ppu.get_frame_data());
                    self.state.transition(State::COMPUTING);
                }
                State::EXITING => {
                    log::debug!("gb thread exited");
                    backend.ack_front_end();
                    return;
                }
            }
        }
    }

    fn initialize(&mut self, backend: &Backend) {
        if backend.should_close() {
            self.state.transition(State::EXITING);
            return;
        }

        if backend.should_pause() {
            backend.wait_pause_resume();
        }

        match backend.should_set_skip_bootrom() {
            Some(skip_bootrom) => self.skip_boot_rom = skip_bootrom,
            _ => {}
        }

        match backend.should_load_rom() {
            Some(rom_data) => {
                self.load_rom(rom_data);
                log::debug!("rom cartridge loaded!");
                self.state.transition(State::COMPUTING);
            }
            _ => {}
        }
    }

    fn compute(&mut self, backend: &Backend) {
        let mut cycles_this_frame_so_far: u32 = 0;
        while cycles_this_frame_so_far < CPU_CYCLES_PER_FRAME {
            if backend.should_close() {
                self.state.transition(State::EXITING);
                return;
            }

            if backend.should_pause() {
                backend.wait_pause_resume();
            }

            let (direction_press, action_press, input_state) = backend.recv_joypad_data();
            self.memory.lock().unwrap().write_joypad_queue(
                direction_press,
                action_press,
                input_state,
            );

            match backend.should_set_skip_bootrom() {
                Some(skip_bootrom) => self.skip_boot_rom = skip_bootrom,
                None => {}
            }

            match backend.should_load_rom() {
                Some(rom_data) => {
                    self.load_rom(rom_data);
                    log::debug!("rom cartridge loaded!");
                }
                None => {}
            }

            let step_fn = &mut || {
                self.timers.lock().unwrap().step(&self.interrupt_bus);
                self.memory.lock().unwrap().step_dma();
                self.ppu.step_graphics(&self.memory, &self.interrupt_bus);
            };

            if self.cpu.lock().unwrap().is_stopped() {
                // todo
            }

            if self.cpu.lock().unwrap().is_halted() {
                step_fn();
                self.cpu.lock().unwrap().handle_halt(&self.interrupt_bus);
            } else {
                let _ = self
                    .cpu
                    .lock()
                    .unwrap()
                    .execute_next_opcode(&self.memory, step_fn);

                self.cpu.lock().unwrap().process_interrupts(
                    &self.memory,
                    &self.interrupt_bus,
                    step_fn,
                );
            }

            cycles_this_frame_so_far += self.timers.lock().unwrap().get_elapsed_cycles();
        }

        self.state.transition(State::RENDERING);
    }
}
