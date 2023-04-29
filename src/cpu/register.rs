#[path = "register_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::Register;

impl Register {
    pub fn new() -> Self {
        Self { hi: 0x00, lo: 0x00 }
    }

    pub fn word(&self) -> u16 {
        u16::from(self.hi) << 8 | u16::from(self.lo)
    }

    pub fn set_word(&mut self, word: u16) {
        self.hi = word.to_be_bytes()[0];
        self.lo = word.to_be_bytes()[1];
    }
}

#[derive(Debug)]
pub enum ID {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum ID16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}
