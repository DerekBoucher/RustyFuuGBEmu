#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "memory_test.rs"]
#[cfg(test)]
mod tests;

pub struct Memory {
    rom: [u8; 0x10000],
}

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
