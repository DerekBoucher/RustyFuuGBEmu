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

#[derive(Debug, PartialEq)]
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

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::A => write!(f, "A"),
            ID::F => write!(f, "F"),
            ID::B => write!(f, "B"),
            ID::C => write!(f, "C"),
            ID::D => write!(f, "D"),
            ID::E => write!(f, "E"),
            ID::H => write!(f, "H"),
            ID::L => write!(f, "L"),
        }
    }
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
