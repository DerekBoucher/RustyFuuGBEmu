#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge;
use crate::cpu::CPU_CYCLES_PER_FRAME;
use crate::interface;
use crossbeam::channel::{self};
use crossbeam::select;

mod orchestrator;

pub struct Gameboy {
    require_render: bool,
    cartridge_inserted: bool,
}

pub struct Orchestrator {
    close_sender: channel::Sender<()>,
    pause_sender: channel::Sender<()>,
    ack_receiver: channel::Receiver<()>,
    rom_data_sender: channel::Sender<Vec<u8>>,
    paused: bool,
}

impl Gameboy {
    pub fn new() -> Self {
        return Self {
            require_render: false,
            cartridge_inserted: false,
        };
    }

    fn load_rom(
        &mut self,
        rom_data: Vec<u8>,
        cpu: &mut impl interface::CPU,
        memory: &mut impl interface::Memory,
        ppu: &mut impl interface::PPU,
    ) {
        cpu.reset();
        ppu.reset();
        memory.reset(cartridge::new(rom_data));
        self.cartridge_inserted = true;
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
    ) -> Orchestrator {
        let (orchestrator, close_receiver, pause_receiver, ack_sender, rom_data_receiver) =
            Orchestrator::new();

        std::thread::spawn(move || {
            self.run(
                close_receiver,
                pause_receiver,
                ack_sender,
                rom_data_receiver,
                cpu,
                memory,
                ppu,
            )
        });
        return orchestrator;
    }

    fn run(
        &mut self,
        close_receiver: channel::Receiver<()>,
        pause_receiver: channel::Receiver<()>,
        ack_sender: channel::Sender<()>,
        rom_data_receiver: channel::Receiver<Vec<u8>>,

        mut cpu: impl interface::CPU,
        mut memory: impl interface::Memory,
        mut ppu: impl interface::PPU,
    ) {
        if !self.cartridge_inserted {
            log::debug!("Waiting for ROM cartridge...");

            select! {
                recv(rom_data_receiver) -> rom_data => {
                    self.load_rom(rom_data.unwrap(), &mut cpu, &mut memory, &mut ppu);
                    log::debug!("ROM cartridge loaded!");
                }
                recv(close_receiver) -> _ => {
                    log::debug!("gb thread closed");
                    ack_sender.send(()).unwrap();
                    return;
                }
            }
        }

        'main: loop {
            let cycles_this_update: u32 = 0;

            while cycles_this_update < CPU_CYCLES_PER_FRAME {
                if Gameboy::should_close(&close_receiver) {
                    break 'main;
                }

                if Gameboy::should_pause(&pause_receiver) {
                    log::debug!("gb emulation paused");
                    pause_receiver.recv().unwrap();
                    log::debug!("gb emulation resumed");
                }

                match Gameboy::should_load_rom(&rom_data_receiver) {
                    Some(rom_data) => {
                        self.load_rom(rom_data, &mut cpu, &mut memory, &mut ppu);
                        log::debug!("ROM cartridge loaded!");
                        continue 'main;
                    }
                    None => {}
                }

                let cycles = cpu.execute_next_opcode(&mut memory);
                memory.update_timers(cycles);
            }
        }

        log::debug!("gb thread closed");
        ack_sender.send(()).unwrap();
    }

    fn should_pause(pause_receiver: &channel::Receiver<()>) -> bool {
        match pause_receiver.try_recv() {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                if err == channel::TryRecvError::Disconnected {
                    log::error!("gb thread cannot pause, channel disconnected unexpectedly");
                    panic!("gb thread cannot pause, channel disconnected unexpectedly");
                }
            }
        }

        return false;
    }

    fn should_close(close_receiver: &channel::Receiver<()>) -> bool {
        match close_receiver.try_recv() {
            Ok(_) => {
                log::info!("closing gb thread...");
                return true;
            }
            Err(err) => {
                if err == channel::TryRecvError::Disconnected {
                    log::error!("gb thread channel disconnected unexpectedly");
                    panic!("gb thread channel disconnected unexpectedly");
                }
            }
        }

        return false;
    }

    fn should_load_rom(rom_data_receiver: &channel::Receiver<Vec<u8>>) -> Option<Vec<u8>> {
        match rom_data_receiver.try_recv() {
            Ok(rom_data) => {
                return Some(rom_data);
            }
            Err(err) => {
                if err == channel::TryRecvError::Disconnected {
                    log::error!("gb thread channel disconnected unexpectedly");
                    panic!("gb thread channel disconnected unexpectedly");
                }
            }
        }

        return None;
    }
}
