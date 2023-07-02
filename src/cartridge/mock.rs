use crate::cartridge;
use std::any::Any;

#[derive(Debug)]
pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(vec: Vec<u8>) -> Box<Self> {
        Box::new(Cartridge { data: vec })
    }
}

impl cartridge::Interface for Cartridge {
    fn read(&self, addr: usize) -> Option<u8> {
        Some(self.data[addr].clone())
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
