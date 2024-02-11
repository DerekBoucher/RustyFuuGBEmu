#[path = "no_mbc_test.rs"]
#[cfg(test)]
mod test;

use crate::interface;
use std::any::Any;

/// Rom only type of cartridge has no memory bank
/// controller. Simplest form of the gameboy cart.
#[derive(Debug)]
pub struct NoMBC {
    data: Vec<u8>,
    ram_bank: [u8; 0x2000],
}

/// Cartridge implementation for the Rom Only type.
impl interface::Cartridge for NoMBC {
    fn as_any(&self) -> &dyn Any {
        self.as_any()
    }

    fn read(&self, addr: usize) -> Option<u8> {
        return self.read(addr);
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.write(addr, val);
    }
}

impl NoMBC {
    /// Constructor for NoMBC
    pub fn new(data: Vec<u8>) -> Box<NoMBC> {
        Box::new(NoMBC {
            data,
            ram_bank: [0x0; 0x2000],
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn read(&self, addr: usize) -> Option<u8> {
        if addr < 0x8000 {
            let byte = match self.data.get(addr) {
                Some(byte) => *byte,
                None => panic!("tried to access cartridge data out of bounds"),
            };
            return Some(byte.clone());
        }

        // Ram bank
        if addr >= 0xA000 && addr < 0xC000 {
            return Some(self.ram_bank[addr - 0xA000].clone());
        }

        None
    }

    fn write(&mut self, addr: usize, val: u8) {
        if addr >= 0xA000 && addr < 0xC000 {
            self.ram_bank[addr - 0xA000] = val;
        }
    }
}
