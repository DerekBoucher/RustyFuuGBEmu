#[path = "memory_test.rs"]
#[cfg(test)]
mod tests;

use crate::memory::Memory;

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [0x00; 0x10000],
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.rom[addr] = value;
    }

    pub fn read(self, addr: usize) -> u8 {
        self.rom[addr]
    }
}
