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
    cartridge_inserted: bool,
}

/// Not to be confused with the Gameboy's player controller, this struct is a
/// mechanism for controlling the behaviour of the asynchronous gameboy thread.
pub struct Controller {
    close_sender: mpsc::Sender<()>,
    pause_sender: mpsc::SyncSender<()>,
    ack_receiver: mpsc::Receiver<()>,
    rom_data_sender: mpsc::Sender<Vec<u8>>,

    paused: bool,
}

impl Gameboy {
    pub fn new() -> Self {
        return Self {
            cpu: LR35902::new(),
            memory: Memory::new(cartridge::default()),
            ppu: Ppu::new(),
            require_render: false,
            cartridge_inserted: false,
        };
    }

    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        *self = Gameboy::new();
        self.memory = Memory::new(cartridge::new(rom_data));
        self.cartridge_inserted = true;
    }

    pub fn skip_boot_rom(&mut self) {
        self.cpu.set_post_boot_rom_state();
        self.memory.set_post_boot_rom_state();
    }

    pub fn start(mut self) -> Controller {
        let (controller, close_receiver, pause_receiver, ack_sender, rom_data_receiver) =
            Controller::new();

        std::thread::spawn(move || {
            self.run(
                close_receiver,
                pause_receiver,
                ack_sender,
                rom_data_receiver,
            )
        });
        return controller;
    }

    fn run(
        &mut self,
        close_receiver: Receiver<()>,
        pause_receiver: Receiver<()>,
        ack_sender: Sender<()>,
        rom_data_receiver: Receiver<Vec<u8>>,
    ) {
        // if !self.cartridge_inserted {
        //     log::debug!("Waiting for rom to be loaded...");
        //     let rom_data = Gameboy::wait_for_cartridge(&rom_data_receiver);
        //     self.load_rom(rom_data);
        //     log::debug!("Rom loaded!");
        // }

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
                        self.load_rom(rom_data);
                        continue 'main;
                    }
                    None => {}
                }

                // let cycles = self.cpu.execute_next_opcode(&mut self.memory);
            }
        }

        log::info!("gb thread closed");
        ack_sender.send(()).unwrap();
    }

    fn should_pause(pause_receiver: &Receiver<()>) -> bool {
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

    fn should_close(close_receiver: &Receiver<()>) -> bool {
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

    fn should_load_rom(rom_data_receiver: &Receiver<Vec<u8>>) -> Option<Vec<u8>> {
        match rom_data_receiver.try_recv() {
            Ok(rom_data) => {
                return Some(rom_data);
            }
            Err(err) => {
                if err == mpsc::TryRecvError::Disconnected {
                    log::error!("gb thread channel disconnected unexpectedly");
                    panic!("gb thread channel disconnected unexpectedly");
                }
            }
        }

        return None;
    }

    // fn wait_for_cartridge(rom_data_receiver: &Receiver<Vec<u8>>) -> Vec<u8> {
    //     match rom_data_receiver.recv() {
    //         Ok(rom_data) => {
    //             return rom_data;
    //         }
    //         Err(err) => {
    //             log::error!("gb thread channel disconnected unexpectedly: {:?}", err);
    //             panic!("gb thread channel disconnected unexpectedly");
    //         }
    //     }
    // }
}
