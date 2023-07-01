use crate::cpu::IMemory;

#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new(vec: Vec<u8>) -> Self {
        Memory { memory: vec }
    }
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.dump() == other.dump()
    }
}

impl IMemory for Memory {
    fn read(&self, addr: usize) -> Option<u8> {
        Some(self.memory[addr].clone())
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }

    fn dump(&self) -> Vec<u8> {
        self.memory.clone()
    }
}
