use crate::cpu::MemoryDriver;

#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new(vec: Vec<u8>) -> Box<Self> {
        Box::new(Memory { memory: vec })
    }
}

impl MemoryDriver for Memory {
    fn read(&self, addr: usize) -> Option<&u8> {
        Some(&self.memory[addr])
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }

    fn dump(&self) -> Vec<u8> {
        self.memory.clone()
    }
}
