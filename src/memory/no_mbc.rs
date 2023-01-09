#[path = "no_mbc_test.rs"]
#[cfg(test)]
mod no_mbc_test;

use crate::memory::Cartridge;
use std::any::Any;

/// Rom only type of cartridge has no memory bank
/// controller. Simplest form of the gameboy cart.
pub struct NoMBC {
    data: Vec<u8>,
    ram_bank: [u8; 0x2000],
}

/// Cartridge implementation for the Rom Only type.
impl Cartridge for NoMBC {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn read(&self, addr: usize) -> Option<&u8> {
        if addr < 0x8000 {
            return self.data.get(addr);
        }

        // Ram bank
        if addr >= 0xA000 && addr < 0xC000 {
            return Some(&self.ram_bank[addr - 0xA000]);
        }

        None
    }

    fn write(&mut self, addr: usize, val: u8) {
        if addr >= 0xA000 && addr < 0xC000 {
            self.ram_bank[addr - 0xA000] = val;
        }
    }
}

/// Constructor for NoMBC
impl NoMBC {
    pub fn new(data: Vec<u8>) -> Box<NoMBC> {
        Box::new(NoMBC {
            data,
            ram_bank: [0x0; 0x2000],
        })
    }
}
