#[path = "opcode_test.rs"]
#[cfg(test)]
mod test;

use super::{bit, register};
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
    LdImm8IntoD_0x16,
    RotateLeftWithCarryIntoA_0x17,
    RelativeJump8_0x18,
    AddDEintoHL_0x19,
    LdMemoryDEIntoA_0x1A,
    DecDE_0x1B,
    IncE_0x1C,
    DecE_0x1D,
    LdImm8IntoE_0x1E,
    RotateRightWithCarryIntoA_0x1F,
    RelativeJumpNotZero8_0x20,
    LdImm16IntoHL_0x21,
    LdAIntoMemoryHLPostInc_0x22,
    IncHL_0x23,
    IncH_0x24,
    DecH_0x25,
    LdImm8IntoH_0x26,
    DAA_0x27,
    RelativeJumpZero8_0x28,
    AddHLintoHL_0x29,
    LdMemoryHLIntoAPostInc_0x2A,
    DecHL_0x2B,
    IncL_0x2C,
    DecL_0x2D,
    LdImm8IntoL_0x2E,
    ComplimentA_0x2F,
    RelativeJumpNotCarry8_0x30,
    LdImm16IntoSP_0x31,
    LdAIntoMemoryHLPostDec_0x32,
    IncSP_0x33,
    IncMemoryHL_0x34,
    DecMemoryHL_0x35,
    LdImm8IntoMemoryHL_0x36,
    SetCarryFlag_0x37,
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
            0x16 => Self::LdImm8IntoD_0x16,
            0x17 => Self::RotateLeftWithCarryIntoA_0x17,
            0x18 => Self::RelativeJump8_0x18,
            0x19 => Self::AddDEintoHL_0x19,
            0x1A => Self::LdMemoryDEIntoA_0x1A,
            0x1B => Self::DecDE_0x1B,
            0x1C => Self::IncE_0x1C,
            0x1D => Self::DecE_0x1D,
            0x1E => Self::LdImm8IntoE_0x1E,
            0x1F => Self::RotateRightWithCarryIntoA_0x1F,
            0x20 => Self::RelativeJumpNotZero8_0x20,
            0x21 => Self::LdImm16IntoHL_0x21,
            0x22 => Self::LdAIntoMemoryHLPostInc_0x22,
            0x23 => Self::IncHL_0x23,
            0x24 => Self::IncH_0x24,
            0x25 => Self::DecH_0x25,
            0x26 => Self::LdImm8IntoH_0x26,
            0x27 => Self::DAA_0x27,
            0x28 => Self::RelativeJumpZero8_0x28,
            0x29 => Self::AddHLintoHL_0x29,
            0x2A => Self::LdMemoryHLIntoAPostInc_0x2A,
            0x2B => Self::DecHL_0x2B,
            0x2C => Self::IncL_0x2C,
            0x2D => Self::DecL_0x2D,
            0x2E => Self::LdImm8IntoL_0x2E,
            0x2F => Self::ComplimentA_0x2F,
            0x30 => Self::RelativeJumpNotCarry8_0x30,
            0x31 => Self::LdImm16IntoSP_0x31,
            0x32 => Self::LdAIntoMemoryHLPostDec_0x32,
            0x33 => Self::IncSP_0x33,
            0x34 => Self::IncMemoryHL_0x34,
            0x35 => Self::DecMemoryHL_0x35,
            0x36 => Self::LdImm8IntoMemoryHL_0x36,
            0x37 => Self::SetCarryFlag_0x37,
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
            Self::LdImm8IntoD_0x16 => 0x16,
            Self::RotateLeftWithCarryIntoA_0x17 => 0x17,
            Self::RelativeJump8_0x18 => 0x18,
            Self::AddDEintoHL_0x19 => 0x19,
            Self::LdMemoryDEIntoA_0x1A => 0x1A,
            Self::DecDE_0x1B => 0x1B,
            Self::IncE_0x1C => 0x1C,
            Self::DecE_0x1D => 0x1D,
            Self::LdImm8IntoE_0x1E => 0x1E,
            Self::RotateRightWithCarryIntoA_0x1F => 0x1F,
            Self::RelativeJumpNotZero8_0x20 => 0x20,
            Self::LdImm16IntoHL_0x21 => 0x21,
            Self::LdAIntoMemoryHLPostInc_0x22 => 0x22,
            Self::IncHL_0x23 => 0x23,
            Self::IncH_0x24 => 0x24,
            Self::DecH_0x25 => 0x25,
            Self::LdImm8IntoH_0x26 => 0x26,
            Self::DAA_0x27 => 0x27,
            Self::RelativeJumpZero8_0x28 => 0x28,
            Self::AddHLintoHL_0x29 => 0x29,
            Self::LdMemoryHLIntoAPostInc_0x2A => 0x2A,
            Self::DecHL_0x2B => 0x2B,
            Self::IncL_0x2C => 0x2C,
            Self::DecL_0x2D => 0x2D,
            Self::LdImm8IntoL_0x2E => 0x2E,
            Self::ComplimentA_0x2F => 0x2F,
            Self::RelativeJumpNotCarry8_0x30 => 0x30,
            Self::LdImm16IntoSP_0x31 => 0x31,
            Self::LdAIntoMemoryHLPostDec_0x32 => 0x32,
            Self::IncSP_0x33 => 0x33,
            Self::IncMemoryHL_0x34 => 0x34,
            Self::DecMemoryHL_0x35 => 0x35,
            Self::LdImm8IntoMemoryHL_0x36 => 0x36,
            Self::SetCarryFlag_0x37 => 0x37,
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
            Self::LdImm8IntoD_0x16 => execute_0x16(cpu),
            Self::RotateLeftWithCarryIntoA_0x17 => execute_0x17(cpu),
            Self::RelativeJump8_0x18 => execute_0x18(cpu),
            Self::AddDEintoHL_0x19 => execute_0x19(cpu),
            Self::LdMemoryDEIntoA_0x1A => execute_0x1a(cpu),
            Self::DecDE_0x1B => execute_0x1b(cpu),
            Self::IncE_0x1C => execute_0x1c(cpu),
            Self::DecE_0x1D => execute_0x1d(cpu),
            Self::LdImm8IntoE_0x1E => execute_0x1e(cpu),
            Self::RotateRightWithCarryIntoA_0x1F => execute_0x1f(cpu),
            Self::RelativeJumpNotZero8_0x20 => execute_0x20(cpu),
            Self::LdImm16IntoHL_0x21 => execute_0x21(cpu),
            Self::LdAIntoMemoryHLPostInc_0x22 => execute_0x22(cpu),
            Self::IncHL_0x23 => execute_0x23(cpu),
            Self::IncH_0x24 => execute_0x24(cpu),
            Self::DecH_0x25 => execute_0x25(cpu),
            Self::LdImm8IntoH_0x26 => execute_0x26(cpu),
            Self::DAA_0x27 => execute_0x27(cpu),
            Self::RelativeJumpZero8_0x28 => execute_0x28(cpu),
            Self::AddHLintoHL_0x29 => execute_0x29(cpu),
            Self::LdMemoryHLIntoAPostInc_0x2A => execute_0x2a(cpu),
            Self::DecHL_0x2B => execute_0x2b(cpu),
            Self::IncL_0x2C => execute_0x2c(cpu),
            Self::DecL_0x2D => execute_0x2d(cpu),
            Self::LdImm8IntoL_0x2E => execute_0x2e(cpu),
            Self::ComplimentA_0x2F => execute_0x2f(cpu),
            Self::RelativeJumpNotCarry8_0x30 => execute_0x30(cpu),
            Self::LdImm16IntoSP_0x31 => execute_0x31(cpu),
            Self::LdAIntoMemoryHLPostDec_0x32 => execute_0x32(cpu),
            Self::IncSP_0x33 => execute_0x33(cpu),
            Self::IncMemoryHL_0x34 => execute_0x34(cpu),
            Self::DecMemoryHL_0x35 => execute_0x35(cpu),
            Self::LdImm8IntoMemoryHL_0x36 => execute_0x36(cpu),
            Self::SetCarryFlag_0x37 => execute_0x37(cpu),
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
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into BC failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
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
        Some(byte) => byte,
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
        Some(byte) => byte,
        None => panic!(
            "opcode 0x08 failed to load lo address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_address_byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
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
        Some(byte) => byte,
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
        Some(byte) => byte,
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
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
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

fn execute_0x16(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into B failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.hi = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x17(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let leftmost_bit_a: bool = (cpu.af.hi & (1 << 7)) > 0;
    let current_carry_flag = cpu.test_carry_flag();

    if leftmost_bit_a {
        cpu.set_carry_flag();
    } else {
        cpu.reset_carry_flag();
    }

    cpu.af.hi = cpu.af.hi << 1;
    cpu.af.hi |= match current_carry_flag {
        true => 0x01,
        false => 0x00,
    };

    cpu.reset_half_carry_flag();
    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    4
}

fn execute_0x18(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let relative_addr = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode relative 8bit jump failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    if bit::test_most_significant_bit(relative_addr) {
        cpu.pc = cpu
            .pc
            .wrapping_sub(bit::two_compliment_byte(relative_addr).into());
    } else {
        cpu.pc = cpu.pc.wrapping_add(relative_addr.into());
    }

    12
}

fn execute_0x19(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::DE);

    8
}

fn execute_0x1a(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match cpu.memory.read(usize::from(cpu.de.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x1A failed to load byte from memory pointed to by DE. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x1b(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_de = cpu.de.word().wrapping_sub(1);

    cpu.de.set_word(new_de);

    8
}

fn execute_0x1c(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::E);

    4
}

fn execute_0x1d(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::E);

    4
}

fn execute_0x1e(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into C failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.lo = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x1f(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let rightmost_bit_a: bool = (cpu.af.hi & 1) > 0;
    let current_carry_flag = cpu.test_carry_flag();

    if rightmost_bit_a {
        cpu.set_carry_flag();
    } else {
        cpu.reset_carry_flag();
    }

    cpu.af.hi = cpu.af.hi >> 1;
    cpu.af.hi |= match current_carry_flag {
        true => 0x80,
        false => 0x00,
    };

    cpu.reset_half_carry_flag();
    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    4
}

fn execute_0x20(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode relative 8bit jump failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    if bit::test_most_significant_bit(relative_addr) {
        cpu.pc = cpu
            .pc
            .wrapping_sub(bit::two_compliment_byte(relative_addr).into());
    } else {
        cpu.pc = cpu.pc.wrapping_add(relative_addr.into());
    }

    12
}

fn execute_0x21(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x22(cpu: &mut LR35902) -> u32 {
    cpu.memory.write(usize::from(cpu.hl.word()), cpu.af.hi);

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x23(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    8
}

fn execute_0x24(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::H);

    4
}

fn execute_0x25(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::H);

    4
}

fn execute_0x26(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into H failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.hi = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x27(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let mut a = cpu.af.hi.clone();

    if !cpu.test_sub_flag() {
        if cpu.test_carry_flag() || a > 0x99 {
            a = a.wrapping_add(0x60);
            cpu.set_carry_flag();
        }

        if cpu.test_half_carry_flag() || ((a & 0x0F) > 0x09) {
            a = a.wrapping_add(0x06);
        }
    } else {
        if cpu.test_carry_flag() {
            a = a.wrapping_sub(0x60);
        }

        if cpu.test_half_carry_flag() {
            a = a.wrapping_sub(0x06);
        }
    }

    if a == 0x00 {
        cpu.set_zero_flag();
    } else {
        cpu.reset_zero_flag();
    }

    cpu.reset_half_carry_flag();

    cpu.af.hi = a;

    4
}

fn execute_0x28(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if !cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode relative 8bit jump failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    if bit::test_most_significant_bit(relative_addr) {
        cpu.pc = cpu
            .pc
            .wrapping_sub(bit::two_compliment_byte(relative_addr).into());
    } else {
        cpu.pc = cpu.pc.wrapping_add(relative_addr.into());
    }

    12
}

fn execute_0x29(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::HL);

    8
}

fn execute_0x2a(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match cpu.memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x2A failed to load byte from memory pointed to by HL. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    8
}

fn execute_0x2b(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_hl = cpu.hl.word().wrapping_sub(1);

    cpu.hl.set_word(new_hl);

    8
}

fn execute_0x2c(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::L);

    4
}

fn execute_0x2d(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::L);

    4
}

fn execute_0x2e(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into L failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.lo = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x2f(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.af.hi = cpu.af.hi ^ 0xFF;

    cpu.set_sub_flag();
    cpu.set_half_carry_flag();

    4
}

fn execute_0x30(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if cpu.test_carry_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode relative 8bit jump failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    if bit::test_most_significant_bit(relative_addr) {
        cpu.pc = cpu
            .pc
            .wrapping_sub(bit::two_compliment_byte(relative_addr).into());
    } else {
        cpu.pc = cpu.pc.wrapping_add(relative_addr.into());
    }

    12
}

fn execute_0x31(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into SP failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into SP failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.sp = u16::from(hi_byte) << 8 | u16::from(lo_byte);

    12
}

fn execute_0x32(cpu: &mut LR35902) -> u32 {
    cpu.memory.write(usize::from(cpu.hl.word()), cpu.af.hi);

    cpu.hl.set_word(cpu.hl.word().wrapping_sub(1));

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x33(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.sp = cpu.sp.wrapping_add(1);

    8
}

fn execute_0x34(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let mut byte = match cpu.memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode increment byte at memory pointed to by HL failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.reset_sub_flag();

    if byte.wrapping_add(1) == 0x00 {
        cpu.set_zero_flag();
    } else {
        cpu.reset_zero_flag();
    }

    if bit::is_half_carry(byte, 1, false) {
        cpu.set_half_carry_flag();
    } else {
        cpu.reset_half_carry_flag();
    }

    byte = byte.wrapping_add(1);

    cpu.memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x35(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let mut byte = match cpu.memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode increment byte at memory pointed to by HL failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.set_sub_flag();

    if byte.wrapping_sub(1) == 0x00 {
        cpu.set_zero_flag();
    } else {
        cpu.reset_zero_flag();
    }

    if bit::is_half_borrow(byte, 1, false) {
        cpu.set_half_carry_flag();
    } else {
        cpu.reset_half_carry_flag();
    }

    byte = byte.wrapping_sub(1);

    cpu.memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x36(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match cpu.memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into Memoryy pointed to by HL failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x37(cpu: &mut LR35902) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.reset_sub_flag();
    cpu.reset_half_carry_flag();
    cpu.set_carry_flag();

    4
}
