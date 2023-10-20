#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge;
use crate::cpu::LR35902;
use crate::memory::Memory;
use crate::ppu::Ppu;
use std::sync::mpsc;

pub struct Gameboy {
    cpu: LR35902,
    memory: Memory,
    ppu: Ppu,

    require_render: bool,
}

impl Gameboy {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: LR35902::new(),
            memory: Memory::new(cartridge::new(rom_data)),
            ppu: Ppu::new(),

            require_render: false,
        }
    }

    pub fn skip_boot_rom(&mut self) {
        self.cpu.set_post_boot_rom_state();
        self.memory.set_post_boot_rom_state();
    }

    pub fn start(mut self) -> (mpsc::Sender<()>, mpsc::Receiver<()>) {
        let (close_signal_writer, close_signal_receiver) = mpsc::channel();
        let (acker, joiner) = mpsc::channel();
        std::thread::spawn(move || self.run(close_signal_receiver, acker));
        return (close_signal_writer, joiner);
    }

    fn run(&mut self, close_signal_receiver: mpsc::Receiver<()>, ack: mpsc::Sender<()>) {
        loop {
            match close_signal_receiver.try_recv() {
                Ok(_) => {
                    println!("closing gb thread...");
                    break;
                }
                Err(err) => {
                    if err == mpsc::TryRecvError::Disconnected {
                        println!("channel disconnected");
                        break;
                    }
                }
            }
        }

        // Signals to the initiator that the thread has closed.
        ack.send(()).unwrap();
        println!("cleanly terminated gb thread...");
    }
}
