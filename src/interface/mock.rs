use crate::interface;
use std::any::Any;

// Memory
// ----------------------------------------------------

#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(vec: Vec<u8>) -> Self {
        Memory { data: vec }
    }

    fn read(&self, addr: usize) -> Option<u8> {
        Some(self.data[addr].clone())
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    pub fn dump(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn update_timers(&mut self, _cycles: u32) {}
}

impl interface::Memory for Memory {
    fn read(&self, addr: usize) -> Option<u8> {
        return self.read(addr);
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.write(addr, val);
    }
    fn dump(&self) -> Vec<u8> {
        return self.dump();
    }

    fn update_timers(&mut self, cycles: u32) {
        self.update_timers(cycles);
    }
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.dump() == other.dump()
    }
}

// Cartridge
// ----------------------------------------------------

#[derive(Debug)]
pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(vec: Vec<u8>) -> Box<Self> {
        Box::new(Cartridge { data: vec })
    }
}

impl interface::Cartridge for Cartridge {
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
