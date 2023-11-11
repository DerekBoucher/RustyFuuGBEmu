use crate::memory;

#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(vec: Vec<u8>) -> Self {
        Memory { data: vec }
    }

    pub fn dump(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl memory::Interface for Memory {
    fn read(&self, addr: usize) -> Option<u8> {
        Some(self.data[addr].clone())
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    fn dump(&self) -> Vec<u8> {
        return self.dump();
    }

    fn update_timers(&mut self, cycles: u32) {}
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.dump() == other.dump()
    }
}
