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

    fn reset(&mut self) {
        self.data = vec![0x00; self.data.len()];
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
}

impl interface::Memory for Memory {
    fn reset(&mut self, _cartridge: Box<dyn interface::Cartridge>) {
        self.reset()
    }

    fn read(&self, addr: usize) -> Option<u8> {
        return self.read(addr);
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.write(addr, val);
    }

    fn update_dma_transfer_cycles(&mut self, _cycles: u32) {}

    fn dma_read(&self, addr: usize) -> Option<u8> {
        return self.read(addr);
    }

    fn dma_write(&mut self, addr: usize, val: u8) {
        self.write(addr, val);
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

// Timers
// ----------------------------------------------------

#[derive(Debug)]
pub struct Timer {
    _divider_tick_counter: u32,
    _timer_tick_counter: i32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            _divider_tick_counter: 0,
            _timer_tick_counter: 0,
        }
    }
}

impl interface::Timers for Timer {
    fn update(
        &mut self,
        _cycles: u32,
        _memory: &mut impl interface::Memory,
        _cpu: &mut impl interface::CPU,
    ) {
    }

    fn reset(&mut self) {
        *self = Timer::new();
    }
}
