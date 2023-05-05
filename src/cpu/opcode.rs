#[path = "opcode_test.rs"]
#[cfg(test)]
mod test;

use super::register;
use crate::cpu::LR35902;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    Nop_0x00,
    LdImm16IntoBC_0x01,
    LdAIntoMemoryBC_0x02,
    IncBC_0x03,
    IncB_0x04,
    DecB_0x05,
    LdImm8IntoB_0x06,
    RotateLeftIntoA_0x07,
    LdSpInto16ImmAddress_0x08,
    AddBCintoHL_0x09,
    LdMemoryBCIntoA_0x0A,
    DecBC_0x0B,
    IncC_0x0C,
    DecC_0x0D,
    LdImm8IntoC_0x0E,
    RotateRightIntoA_0x0F,
    Stop_0x10,
    LdImm16IntoDE_0x11,
    LdAIntoMemoryDE_0x12,
    IncDE_0x13,
    IncD_0x14,
    DecD_0x15,
}

impl std::convert::From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::Nop_0x00,
            0x01 => Opcode::LdImm16IntoBC_0x01,
            0x02 => Self::LdAIntoMemoryBC_0x02,
            0x03 => Self::IncBC_0x03,
            0x04 => Self::IncB_0x04,
            0x05 => Self::DecB_0x05,
            0x06 => Self::LdImm8IntoB_0x06,
            0x07 => Self::RotateLeftIntoA_0x07,
            0x08 => Self::LdSpInto16ImmAddress_0x08,
            0x09 => Self::AddBCintoHL_0x09,
            0x0A => Self::LdMemoryBCIntoA_0x0A,
            0x0B => Self::DecBC_0x0B,
            0x0C => Self::IncC_0x0C,
            0x0D => Self::DecC_0x0D,
            0x0E => Self::LdImm8IntoC_0x0E,
            0x0F => Self::RotateRightIntoA_0x0F,
            0x10 => Self::Stop_0x10,
            0x11 => Self::LdImm16IntoDE_0x11,
            0x12 => Self::LdAIntoMemoryDE_0x12,
            0x13 => Self::IncDE_0x13,
            0x14 => Self::IncD_0x14,
            0x15 => Self::DecD_0x15,
            _ => panic!("unsupported op code (TODO)"),
        }
    }
}

impl std::convert::Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Self::Nop_0x00 => 0x00,
            Self::LdImm16IntoBC_0x01 => 0x01,
            Self::LdAIntoMemoryBC_0x02 => 0x02,
            Self::IncBC_0x03 => 0x03,
            Self::IncB_0x04 => 0x04,
            Self::DecB_0x05 => 0x05,
            Self::LdImm8IntoB_0x06 => 0x06,
            Self::RotateLeftIntoA_0x07 => 0x07,
            Self::LdSpInto16ImmAddress_0x08 => 0x08,
            Self::AddBCintoHL_0x09 => 0x09,
            Self::LdMemoryBCIntoA_0x0A => 0x0A,
            Self::DecBC_0x0B => 0x0B,
            Self::IncC_0x0C => 0x0C,
            Self::DecC_0x0D => 0x0D,
            Self::LdImm8IntoC_0x0E => 0x0E,
            Self::RotateRightIntoA_0x0F => 0x0F,
            Self::Stop_0x10 => 0x10,
            Self::LdImm16IntoDE_0x11 => 0x11,
            Self::LdAIntoMemoryDE_0x12 => 0x12,
            Self::IncDE_0x13 => 0x13,
            Self::IncD_0x14 => 0x14,
            Self::DecD_0x15 => 0x15,
        }
    }
}

impl Opcode {
    pub fn execute(&self, cpu: &mut LR35902) -> u32 {
        match self {
            Self::Nop_0x00 => execute_0x00(cpu),
            Self::LdImm16IntoBC_0x01 => execute_0x01(cpu),
            Self::LdAIntoMemoryBC_0x02 => execute_0x02(cpu),
            Self::IncBC_0x03 => execute_0x03(cpu),
            Self::IncB_0x04 => execute_0x04(cpu),
            Self::DecB_0x05 => execute_0x05(cpu),
            Self::LdImm8IntoB_0x06 => execute_0x06(cpu),
            Self::RotateLeftIntoA_0x07 => execute_0x07(cpu),
            Self::LdSpInto16ImmAddress_0x08 => execute_0x08(cpu),
            Self::AddBCintoHL_0x09 => execute_0x09(cpu),
            Self::LdMemoryBCIntoA_0x0A => execute_0x0a(cpu),
            Self::DecBC_0x0B => execute_0x0b(cpu),
            Self::IncC_0x0C => execute_0x0c(cpu),
            Self::DecC_0x0D => execute_0x0d(cpu),
            Self::LdImm8IntoC_0x0E => execute_0x0e(cpu),
            Self::RotateRightIntoA_0x0F => execute_0x0f(cpu),
            Self::Stop_0x10 => execute_0x10(cpu),
            Self::LdImm16IntoDE_0x11 => execute_0x11(cpu),
            Self::LdAIntoMemoryDE_0x12 => execute_0x12(cpu),
            Self::IncDE_0x13 => execute_0x13(cpu),
            Self::IncD_0x14 => execute_0x14(cpu),
            Self::DecD_0x15 => execute_0x15(cpu),
        }
    }
}

fn execute_0x00(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    4
}

fn execute_0x01(cpu: &mut LR35902) -> u32 {
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

fn execute_0x02(cpu: &mut LR35902) -> u32 {
    cpu.memory.write(usize::from(cpu.bc.word()), cpu.af.hi);

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x03(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.set_word(cpu.bc.word().wrapping_add(1));

    8
}

fn execute_0x04(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::B);

    4
}

fn execute_0x05(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::B);

    4
}

fn execute_0x06(cpu: &mut LR35902) -> u32 {
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

fn execute_0x07(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let leftmost_bit_a: bool = (cpu.af.hi & (1 << 7)) > 0;

    if leftmost_bit_a {
        cpu.set_carry_flag();
    } else {
        cpu.reset_carry_flag();
    }

    cpu.af.hi = cpu.af.hi.rotate_left(1);

    cpu.reset_half_carry_flag();
    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    4
}

fn execute_0x08(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_address_byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => *byte,
        None => panic!(
            "opcode 0x08 failed to load lo address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_address_byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => *byte,
        None => panic!(
            "opcode 0x08 failed to load hi address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let mut addr = usize::from(u16::from(hi_address_byte) << 8 | u16::from(lo_address_byte));

    cpu.memory.write(addr, cpu.sp.to_be_bytes()[1]);
    addr += 1;
    cpu.memory.write(addr, cpu.sp.to_be_bytes()[0]);

    20
}

pub fn execute_0x09(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::BC);

    8
}

fn execute_0x0a(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match cpu.memory.read(usize::from(cpu.bc.word())) {
        Some(byte) => *byte,
        None => panic!(
            "opcode 0x0A failed to load byte from memory pointed to by BC. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x0b(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_bc = cpu.bc.word().wrapping_sub(1);

    cpu.bc.set_word(new_bc);

    8
}

fn execute_0x0c(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::C);

    4
}

fn execute_0x0d(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::C);

    4
}

fn execute_0x0e(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => *byte,
        None => panic!(
            "opcode load imm 8 into C failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.bc.lo = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x0f(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let rightmost_bit_a: bool = (cpu.af.hi & 1) > 0;

    if rightmost_bit_a {
        cpu.set_carry_flag();
    } else {
        cpu.reset_carry_flag();
    }

    cpu.af.hi = cpu.af.hi.rotate_right(1);

    cpu.reset_half_carry_flag();
    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    4
}

fn execute_0x10(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.paused = true;

    4
}

fn execute_0x11(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => *byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => *byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x12(cpu: &mut LR35902) -> u32 {
    cpu.memory.write(usize::from(cpu.de.word()), cpu.af.hi);

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x13(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.set_word(cpu.de.word().wrapping_add(1));

    8
}

fn execute_0x14(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::D);

    4
}

fn execute_0x15(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::D);

    4
}
