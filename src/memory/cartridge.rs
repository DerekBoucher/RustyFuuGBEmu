#[path = "cartridge_test.rs"]
#[cfg(test)]
mod cartridge_test;
use crate::memory::Cartridge;

pub fn new(data: Vec<u8>) -> Box<dyn Cartridge> {
    return Box::new(MBC1 {});
}

struct MBC1 {}

impl Cartridge for MBC1 {
    fn read(&self, addr: usize) -> u8 {
        0x00
    }

    fn write(&mut self, addr: usize, val: u8) {}
}
