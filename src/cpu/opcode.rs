#[path = "opcode_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::bit;
use crate::cpu::LR35902;

pub struct Nop;
impl Nop {
    pub const OPCODE: u8 = 0x00;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        4
    }
}

pub struct LdImm16IntoBC;
impl LdImm16IntoBC {
    pub const OPCODE: u8 = 0x01;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        cpu.bc.lo = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 16 into BC failed to fetch lo byte. Dumping cpu state...\n{:?}",
                cpu
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        cpu.bc.hi = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 16 into BC failed to fetch hi byte. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        12
    }
}

pub struct LdAIntoMemoryBC;
impl LdAIntoMemoryBC {
    pub const OPCODE: u8 = 0x02;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.memory.write(usize::from(cpu.bc.word()), cpu.af.hi);
        cpu.pc = cpu.pc.wrapping_add(1);
        8
    }
}

pub struct IncBC;
impl IncBC {
    pub const OPCODE: u8 = 0x03;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let mut word = cpu.bc.word();
        word = word.wrapping_add(1);

        cpu.bc.hi = word.to_be_bytes()[0];
        cpu.bc.lo = word.to_be_bytes()[1];

        8
    }
}

pub struct IncB;
impl IncB {
    pub const OPCODE: u8 = 0x04;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        if bit::is_half_carry(cpu.bc.hi, 0x01, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        cpu.bc.hi = cpu.bc.hi.wrapping_add(1);

        if cpu.bc.hi == 0x00 {
            cpu.set_zero_flag();
        } else {
            cpu.reset_zero_flag();
        }

        cpu.reset_sub_flag();

        4
    }
}

pub struct DecB;
impl DecB {
    pub const OPCODE: u8 = 0x05;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        if bit::is_half_borrow(cpu.bc.hi, 0x1, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        cpu.bc.hi = cpu.bc.hi.wrapping_sub(1);

        if cpu.bc.hi == 0x00 {
            cpu.set_zero_flag();
        } else {
            cpu.reset_zero_flag();
        }

        cpu.set_sub_flag();

        4
    }
}

pub struct LdImm8IntoB;
impl LdImm8IntoB {
    pub const OPCODE: u8 = 0x06;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let byte = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 8 into B failed to fetch byte in memory. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.bc.hi = byte;

        cpu.pc = cpu.pc.wrapping_add(1);

        8
    }
}
