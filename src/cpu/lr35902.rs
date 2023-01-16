#[path = "lr35902_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::opcode::*;
use crate::cpu::MemoryDriver;
use crate::cpu::Register;
use crate::cpu::LR35902;

/// Bit mask for the zero flag
const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
const CARRY_FLAG_MASK: u8 = 1 << 4;

impl PartialEq for LR35902 {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
            && self.sp == other.sp
            && self.af.word() == other.af.word()
            && self.bc.word() == other.bc.word()
            && self.de.word() == other.de.word()
            && self.hl.word() == other.hl.word()
    }
}

impl LR35902 {
    pub fn new(memory_driver: Box<dyn MemoryDriver>) -> Self {
        Self {
            af: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0x0000,
            pc: 0x0000,
            memory: memory_driver,
        }
    }

    pub fn execute_next_opcode(&mut self) -> u32 {
        let op = match self.memory.read(usize::from(self.pc)) {
            Some(x) => *x,
            None => panic!(
                "memory returned empty value when attempting to fetch op code. Dumping cpu state...\n
                {:?}", self
            ),
        };

        match op {
            Nop::OPCODE => Nop::execute(self),
            LdImm16IntoBC::OPCODE => LdImm16IntoBC::execute(self),
            LdAIntoMemoryBC::OPCODE => LdAIntoMemoryBC::execute(self),
            IncBC::OPCODE => IncBC::execute(self),
            IncB::OPCODE => IncB::execute(self),
            DecB::OPCODE => DecB::execute(self),
            LdImm8IntoB::OPCODE => LdImm8IntoB::execute(self),
            _ => 0,
        }
    }

    pub fn reset_half_carry_flag(&mut self) {
        self.af.lo &= !HALF_CARRY_FLAG_MASK;
    }

    pub fn set_half_carry_flag(&mut self) {
        self.af.lo |= HALF_CARRY_FLAG_MASK;
    }

    pub fn reset_zero_flag(&mut self) {
        self.af.lo &= !ZERO_FLAG_MASK;
    }

    pub fn set_zero_flag(&mut self) {
        self.af.lo |= ZERO_FLAG_MASK;
    }

    pub fn reset_carry_flag(&mut self) {
        self.af.lo &= !CARRY_FLAG_MASK;
    }

    pub fn set_carry_flag(&mut self) {
        self.af.lo |= CARRY_FLAG_MASK;
    }

    pub fn reset_sub_flag(&mut self) {
        self.af.lo &= !SUB_FLAG_MASK;
    }

    pub fn set_sub_flag(&mut self) {
        self.af.lo |= SUB_FLAG_MASK;
    }
}
