#[path = "register_test.rs"]
#[cfg(test)]
mod register_test;

use crate::cpu::Register;

impl Register {
    pub fn new() -> Self {
        Self { hi: 0x00, lo: 0x00 }
    }

    pub fn word(&self) -> u16 {
        u16::from(self.hi) << 8 | u16::from(self.lo)
    }
}
