#[path = "opcode_test.rs"]
#[cfg(test)]
mod opcode_test;

use crate::cpu::LR35902;

pub struct Nop;
impl Nop {
    pub const OPCODE: u8 = 0x00;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc += 0x01;

        4
    }
}

pub struct LdImm16IntoBC;
impl LdImm16IntoBC {
    pub const OPCODE: u8 = 0x01;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc += 1;

        cpu.bc.lo = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!("opcode load imm 16 into BC failed to fetch lo byte"),
        };

        cpu.pc += 1;

        cpu.bc.hi = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!("opcode load imm 16 into BC failed to fetch hi byte"),
        };

        cpu.pc += 1;

        12
    }
}

pub struct LdAIntoMemoryBC;
impl LdAIntoMemoryBC {
    pub const OPCODE: u8 = 0x02;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.memory.write(usize::from(cpu.bc.word()), cpu.af.hi);
        cpu.pc += 1;
        8
    }
}

pub struct IncBC;
impl IncBC {
    pub const OPCODE: u8 = 0x03;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc += 1;
        cpu.bc.hi += 1;
        8
    }
}
