#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge;
use crate::cpu::{CPU_CYCLES_PER_FRAME, LR35902};
use crate::memory::Memory;
use crate::ppu::Ppu;
use std::sync::mpsc::{self, Receiver, Sender};

mod controller;

pub struct Gameboy {
    cpu: LR35902,
    memory: Memory,
    ppu: Ppu,

    require_render: bool,
}

impl Gameboy {
    pub fn new(rom_data: Vec<u8>) -> Self {
        return Self {
            cpu: LR35902::new(),
            memory: Memory::new(cartridge::new(rom_data)),
            ppu: Ppu::new(),

            require_render: false,
        };
    }

    pub fn skip_boot_rom(&mut self) {
        self.cpu.set_post_boot_rom_state();
        self.memory.set_post_boot_rom_state();
    }

    pub fn start(mut self) -> controller::Controller {
        let (controller, close_receiver, pause_receiver, ack_sender) =
            controller::Controller::new();

        std::thread::spawn(move || self.run(close_receiver, pause_receiver, ack_sender));
        return controller;
    }

    fn run(
        &mut self,
        close_receiver: Receiver<()>,
        pause_receiver: Receiver<()>,
        ack_sender: Sender<()>,
    ) {
        'main: loop {
            let cycles_this_update: u32 = 0;

            while cycles_this_update < CPU_CYCLES_PER_FRAME {
                if self.should_close(&close_receiver) {
                    break 'main;
                }

                if self.should_pause(&pause_receiver) {
                    log::debug!("gb emulation paused");
                    pause_receiver.recv().unwrap();
                    log::debug!("gb emulation resumed");
                }

                // let cycles = self.cpu.execute_next_opcode(&mut self.memory);
            }
        }

        log::info!("gb thread closed");
        ack_sender.send(()).unwrap();
    }

    fn should_pause(&self, pause_receiver: &Receiver<()>) -> bool {
        match pause_receiver.try_recv() {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                if err == mpsc::TryRecvError::Disconnected {
                    log::error!("gb thread cannot pause, channel disconnected unexpectedly");
                    panic!("gb thread cannot pause, channel disconnected unexpectedly");
                }
            }
        }

        return false;
    }

    fn should_close(&self, close_receiver: &Receiver<()>) -> bool {
        match close_receiver.try_recv() {
            Ok(_) => {
                log::info!("closing gb thread...");
                return true;
            }
            Err(err) => {
                if err == mpsc::TryRecvError::Disconnected {
                    log::error!("gb thread channel disconnected unexpectedly");
                    panic!("gb thread channel disconnected unexpectedly");
                }
            }
        }

        return false;
    }
}
