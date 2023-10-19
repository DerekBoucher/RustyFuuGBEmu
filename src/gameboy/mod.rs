#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge;
use crate::cpu::LR35902;
use crate::memory::Memory;
use crate::ppu::Ppu;

pub struct Gameboy {
    cpu: LR35902,
    memory: Memory,
    ppu: Ppu,

    running: bool,
    require_render: bool,
}

impl Gameboy {
    pub fn new(cartridge: Box<dyn cartridge::Interface>) -> Self {
        Self {
            cpu: LR35902::new(),
            memory: Memory::new(cartridge),
            ppu: Ppu::new(),

            running: false,
            require_render: false,
        }
    }

    pub fn skip_boot_rom(&mut self) {
        self.cpu.set_post_boot_rom_state();
        self.memory.set_post_boot_rom_state();
    }
}
