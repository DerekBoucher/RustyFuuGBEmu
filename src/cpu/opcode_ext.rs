#[path = "opcode_ext_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::register;
use crate::cpu::LR35902;
use crate::memory;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum ExtendedOpcode {
    RotateBLeft_0x00,
    RotateCLeft_0x01,
    RotateDLeft_0x02,
    RotateELeft_0x03,
    RotateHLeft_0x04,
    RotateLLeft_0x05,
    RotateMemoryHLLeft_0x06,
    RotateALeft_0x07,
    RotateBRight_0x08,
    RotateCRight_0x09,
    RotateDRight_0x0A,
    RotateERight_0x0B,
    RotateHRight_0x0C,
    RotateLRight_0x0D,
    RotateMemoryHLRight_0x0E,
    RotateARight_0x0F,
    RotateBLeftWithCarry_0x10,
    RotateCLeftWithCarry_0x11,
    RotateDLeftWithCarry_0x12,
    RotateELeftWithCarry_0x13,
    RotateHLeftWithCarry_0x14,
    RotateLLeftWithCarry_0x15,
    RotateMemoryHLLeftWithCarry_0x16,
    RotateALeftWithCarry_0x17,
    RotateBRightWithCarry_0x18,
    RotateCRightWithCarry_0x19,
    RotateDRightWithCarry_0x1A,
    RotateERightWithCarry_0x1B,
    RotateHRightWithCarry_0x1C,
    RotateLRightWithCarry_0x1D,
    RotateMemoryHLRightWithCarry_0x1E,
    RotateARightWithCarry_0x1F,
    ShiftLeftBIntoCarry_0x20,
    ShiftLeftCIntoCarry_0x21,
    ShiftLeftDIntoCarry_0x22,
    ShiftLeftEIntoCarry_0x23,
    ShiftLeftHIntoCarry_0x24,
    ShiftLeftLIntoCarry_0x25,
    ShiftLeftMemoryHLIntoCarry_0x26,
}

impl std::convert::From<u8> for ExtendedOpcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::RotateBLeft_0x00,
            0x01 => Self::RotateCLeft_0x01,
            0x02 => Self::RotateDLeft_0x02,
            0x03 => Self::RotateELeft_0x03,
            0x04 => Self::RotateHLeft_0x04,
            0x05 => Self::RotateLLeft_0x05,
            0x06 => Self::RotateMemoryHLLeft_0x06,
            0x07 => Self::RotateALeft_0x07,
            0x08 => Self::RotateBRight_0x08,
            0x09 => Self::RotateCRight_0x09,
            0x0A => Self::RotateDRight_0x0A,
            0x0B => Self::RotateERight_0x0B,
            0x0C => Self::RotateHRight_0x0C,
            0x0D => Self::RotateLRight_0x0D,
            0x0E => Self::RotateMemoryHLRight_0x0E,
            0x0F => Self::RotateARight_0x0F,
            0x10 => Self::RotateBLeftWithCarry_0x10,
            0x11 => Self::RotateCLeftWithCarry_0x11,
            0x12 => Self::RotateDLeftWithCarry_0x12,
            0x13 => Self::RotateELeftWithCarry_0x13,
            0x14 => Self::RotateHLeftWithCarry_0x14,
            0x15 => Self::RotateLLeftWithCarry_0x15,
            0x16 => Self::RotateMemoryHLLeftWithCarry_0x16,
            0x17 => Self::RotateALeftWithCarry_0x17,
            0x18 => Self::RotateBRightWithCarry_0x18,
            0x19 => Self::RotateCRightWithCarry_0x19,
            0x1A => Self::RotateDRightWithCarry_0x1A,
            0x1B => Self::RotateERightWithCarry_0x1B,
            0x1C => Self::RotateHRightWithCarry_0x1C,
            0x1D => Self::RotateLRightWithCarry_0x1D,
            0x1E => Self::RotateMemoryHLRightWithCarry_0x1E,
            0x1F => Self::RotateARightWithCarry_0x1F,
            0x20 => Self::ShiftLeftBIntoCarry_0x20,
            0x21 => Self::ShiftLeftCIntoCarry_0x21,
            0x22 => Self::ShiftLeftDIntoCarry_0x22,
            0x23 => Self::ShiftLeftEIntoCarry_0x23,
            0x24 => Self::ShiftLeftHIntoCarry_0x24,
            0x25 => Self::ShiftLeftLIntoCarry_0x25,
            0x26 => Self::ShiftLeftMemoryHLIntoCarry_0x26,
            _ => panic!("todo"),
        }
    }
}

impl std::convert::Into<u8> for ExtendedOpcode {
    fn into(self) -> u8 {
        match self {
            Self::RotateBLeft_0x00 => 0x00,
            Self::RotateCLeft_0x01 => 0x01,
            Self::RotateDLeft_0x02 => 0x02,
            Self::RotateELeft_0x03 => 0x03,
            Self::RotateHLeft_0x04 => 0x04,
            Self::RotateLLeft_0x05 => 0x05,
            Self::RotateMemoryHLLeft_0x06 => 0x06,
            Self::RotateALeft_0x07 => 0x07,
            Self::RotateBRight_0x08 => 0x08,
            Self::RotateCRight_0x09 => 0x09,
            Self::RotateDRight_0x0A => 0x0A,
            Self::RotateERight_0x0B => 0x0B,
            Self::RotateHRight_0x0C => 0x0C,
            Self::RotateLRight_0x0D => 0x0D,
            Self::RotateMemoryHLRight_0x0E => 0x0E,
            Self::RotateARight_0x0F => 0x0F,
            Self::RotateBLeftWithCarry_0x10 => 0x10,
            Self::RotateCLeftWithCarry_0x11 => 0x11,
            Self::RotateDLeftWithCarry_0x12 => 0x12,
            Self::RotateELeftWithCarry_0x13 => 0x13,
            Self::RotateHLeftWithCarry_0x14 => 0x14,
            Self::RotateLLeftWithCarry_0x15 => 0x15,
            Self::RotateMemoryHLLeftWithCarry_0x16 => 0x16,
            Self::RotateALeftWithCarry_0x17 => 0x17,
            Self::RotateBRightWithCarry_0x18 => 0x18,
            Self::RotateCRightWithCarry_0x19 => 0x19,
            Self::RotateDRightWithCarry_0x1A => 0x1A,
            Self::RotateERightWithCarry_0x1B => 0x1B,
            Self::RotateHRightWithCarry_0x1C => 0x1C,
            Self::RotateLRightWithCarry_0x1D => 0x1D,
            Self::RotateMemoryHLRightWithCarry_0x1E => 0x1E,
            Self::RotateARightWithCarry_0x1F => 0x1F,
            Self::ShiftLeftBIntoCarry_0x20 => 0x20,
            Self::ShiftLeftCIntoCarry_0x21 => 0x21,
            Self::ShiftLeftDIntoCarry_0x22 => 0x22,
            Self::ShiftLeftEIntoCarry_0x23 => 0x23,
            Self::ShiftLeftHIntoCarry_0x24 => 0x24,
            Self::ShiftLeftLIntoCarry_0x25 => 0x25,
            Self::ShiftLeftMemoryHLIntoCarry_0x26 => 0x26,
        }
    }
}

impl ExtendedOpcode {
    pub fn execute(&self, cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
        match self {
            Self::RotateBLeft_0x00 => execute_0x00(cpu, memory),
            Self::RotateCLeft_0x01 => execute_0x01(cpu, memory),
            Self::RotateDLeft_0x02 => execute_0x02(cpu, memory),
            Self::RotateELeft_0x03 => execute_0x03(cpu, memory),
            Self::RotateHLeft_0x04 => execute_0x04(cpu, memory),
            Self::RotateLLeft_0x05 => execute_0x05(cpu, memory),
            Self::RotateMemoryHLLeft_0x06 => execute_0x06(cpu, memory),
            Self::RotateALeft_0x07 => execute_0x07(cpu, memory),
            Self::RotateBRight_0x08 => execute_0x08(cpu, memory),
            Self::RotateCRight_0x09 => execute_0x09(cpu, memory),
            Self::RotateDRight_0x0A => execute_0x0a(cpu, memory),
            Self::RotateERight_0x0B => execute_0x0b(cpu, memory),
            Self::RotateHRight_0x0C => execute_0x0c(cpu, memory),
            Self::RotateLRight_0x0D => execute_0x0d(cpu, memory),
            Self::RotateMemoryHLRight_0x0E => execute_0x0e(cpu, memory),
            Self::RotateARight_0x0F => execute_0x0f(cpu, memory),
            Self::RotateBLeftWithCarry_0x10 => execute_0x10(cpu, memory),
            Self::RotateCLeftWithCarry_0x11 => execute_0x11(cpu, memory),
            Self::RotateDLeftWithCarry_0x12 => execute_0x12(cpu, memory),
            Self::RotateELeftWithCarry_0x13 => execute_0x13(cpu, memory),
            Self::RotateHLeftWithCarry_0x14 => execute_0x14(cpu, memory),
            Self::RotateLLeftWithCarry_0x15 => execute_0x15(cpu, memory),
            Self::RotateMemoryHLLeftWithCarry_0x16 => execute_0x16(cpu, memory),
            Self::RotateALeftWithCarry_0x17 => execute_0x17(cpu, memory),
            Self::RotateBRightWithCarry_0x18 => execute_0x18(cpu, memory),
            Self::RotateCRightWithCarry_0x19 => execute_0x19(cpu, memory),
            Self::RotateDRightWithCarry_0x1A => execute_0x1a(cpu, memory),
            Self::RotateERightWithCarry_0x1B => execute_0x1b(cpu, memory),
            Self::RotateHRightWithCarry_0x1C => execute_0x1c(cpu, memory),
            Self::RotateLRightWithCarry_0x1D => execute_0x1d(cpu, memory),
            Self::RotateMemoryHLRightWithCarry_0x1E => execute_0x1e(cpu, memory),
            Self::RotateARightWithCarry_0x1F => execute_0x1f(cpu, memory),
            Self::ShiftLeftBIntoCarry_0x20 => execute_0x20(cpu, memory),
            Self::ShiftLeftCIntoCarry_0x21 => execute_0x21(cpu, memory),
            Self::ShiftLeftDIntoCarry_0x22 => execute_0x22(cpu, memory),
            Self::ShiftLeftEIntoCarry_0x23 => execute_0x23(cpu, memory),
            Self::ShiftLeftHIntoCarry_0x24 => execute_0x24(cpu, memory),
            Self::ShiftLeftLIntoCarry_0x25 => execute_0x25(cpu, memory),
            Self::ShiftLeftMemoryHLIntoCarry_0x26 => execute_0x26(cpu, memory),
        }
    }
}

fn execute_0x00(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::B);
}

fn execute_0x01(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::C);
}

fn execute_0x02(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::D);
}

fn execute_0x03(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::E);
}

fn execute_0x04(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::H);
}

fn execute_0x05(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::L);
}

fn execute_0x06(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_memory_left(memory, usize::from(cpu.hl.word()));
}

fn execute_0x07(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left(register::ID::A);
}

fn execute_0x08(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::B);
}

fn execute_0x09(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::C);
}

fn execute_0x0a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::D);
}

fn execute_0x0b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::E);
}

fn execute_0x0c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::H);
}

fn execute_0x0d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::L);
}

fn execute_0x0e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_memory_right(memory, usize::from(cpu.hl.word()));
}

fn execute_0x0f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right(register::ID::A);
}

fn execute_0x10(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::B);
}

fn execute_0x11(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::C);
}

fn execute_0x12(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::D);
}

fn execute_0x13(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::E);
}

fn execute_0x14(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::H);
}

fn execute_0x15(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::L);
}

fn execute_0x16(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_memory_left_carry(memory, usize::from(cpu.hl.word()));
}

fn execute_0x17(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_left_carry(register::ID::A);
}

fn execute_0x18(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::B);
}

fn execute_0x19(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::C);
}

fn execute_0x1a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::D);
}

fn execute_0x1b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::E);
}

fn execute_0x1c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::H);
}

fn execute_0x1d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::L);
}

fn execute_0x1e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_memory_right_carry(memory, usize::from(cpu.hl.word()));
}

fn execute_0x1f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.rotate_8bit_register_right_carry(register::ID::A);
}

fn execute_0x20(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::B);
}

fn execute_0x21(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::C);
}

fn execute_0x22(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::D);
}

fn execute_0x23(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::E);
}

fn execute_0x24(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::H);
}

fn execute_0x25(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::L);
}

fn execute_0x26(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_memory_into_carry(memory, usize::from(cpu.hl.word()));
}
