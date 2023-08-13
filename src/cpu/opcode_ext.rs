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
}

impl std::convert::From<u8> for ExtendedOpcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::RotateBLeft_0x00,
            0x01 => Self::RotateCLeft_0x01,
            0x02 => Self::RotateDLeft_0x02,
            0x03 => Self::RotateELeft_0x03,
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
