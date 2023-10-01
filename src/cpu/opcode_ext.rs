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
    ShiftLeftAIntoCarry_0x27,
    ShiftRightBIntoCarry_0x28,
    ShiftRightCIntoCarry_0x29,
    ShiftRightDIntoCarry_0x2A,
    ShiftRightEIntoCarry_0x2B,
    ShiftRightHIntoCarry_0x2C,
    ShiftRightLIntoCarry_0x2D,
    ShiftRightMemoryHLIntoCarry_0x2E,
    ShiftRightAIntoCarry_0x2F,
    SwapB_0x30,
    SwapC_0x31,
    SwapD_0x32,
    SwapE_0x33,
    SwapH_0x34,
    SwapL_0x35,
    SwapMemoryHL_0x36,
    SwapA_0x37,
    ShiftRightB_0x38,
    ShiftRightC_0x39,
    ShiftRightD_0x3A,
    ShiftRightE_0x3B,
    ShiftRightH_0x3C,
    ShiftRightL_0x3D,
    ShiftRightMemoryHL_0x3E,
    ShiftRightA_0x3F,
    TestBit0_B_0x40,
    TestBit0_C_0x41,
    TestBit0_D_0x42,
    TestBit0_E_0x43,
    TestBit0_H_0x44,
    TestBit0_L_0x45,
    TestBit0_MemoryHL_0x46,
    TestBit0_A_0x47,
    TestBit1_B_0x48,
    TestBit1_C_0x49,
    TestBit1_D_0x4A,
    TestBit1_E_0x4B,
    TestBit1_H_0x4C,
    TestBit1_L_0x4D,
    TestBit1_MemoryHL_0x4E,
    TestBit1_A_0x4F,
    TestBit2_B_0x50,
    TestBit2_C_0x51,
    TestBit2_D_0x52,
    TestBit2_E_0x53,
    TestBit2_H_0x54,
    TestBit2_L_0x55,
    TestBit2_MemoryHL_0x56,
    TestBit2_A_0x57,
    TestBit3_B_0x58,
    TestBit3_C_0x59,
    TestBit3_D_0x5A,
    TestBit3_E_0x5B,
    TestBit3_H_0x5C,
    TestBit3_L_0x5D,
    TestBit3_MemoryHL_0x5E,
    TestBit3_A_0x5F,
    TestBit4_B_0x60,
    TestBit4_C_0x61,
    TestBit4_D_0x62,
    TestBit4_E_0x63,
    TestBit4_H_0x64,
    TestBit4_L_0x65,
    TestBit4_MemoryHL_0x66,
    TestBit4_A_0x67,
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
            0x27 => Self::ShiftLeftAIntoCarry_0x27,
            0x28 => Self::ShiftRightBIntoCarry_0x28,
            0x29 => Self::ShiftRightCIntoCarry_0x29,
            0x2A => Self::ShiftRightDIntoCarry_0x2A,
            0x2B => Self::ShiftRightEIntoCarry_0x2B,
            0x2C => Self::ShiftRightHIntoCarry_0x2C,
            0x2D => Self::ShiftRightLIntoCarry_0x2D,
            0x2E => Self::ShiftRightMemoryHLIntoCarry_0x2E,
            0x2F => Self::ShiftRightAIntoCarry_0x2F,
            0x30 => Self::SwapB_0x30,
            0x31 => Self::SwapC_0x31,
            0x32 => Self::SwapD_0x32,
            0x33 => Self::SwapE_0x33,
            0x34 => Self::SwapH_0x34,
            0x35 => Self::SwapL_0x35,
            0x36 => Self::SwapMemoryHL_0x36,
            0x37 => Self::SwapA_0x37,
            0x38 => Self::ShiftRightB_0x38,
            0x39 => Self::ShiftRightC_0x39,
            0x3A => Self::ShiftRightD_0x3A,
            0x3B => Self::ShiftRightE_0x3B,
            0x3C => Self::ShiftRightH_0x3C,
            0x3D => Self::ShiftRightL_0x3D,
            0x3E => Self::ShiftRightMemoryHL_0x3E,
            0x3F => Self::ShiftRightA_0x3F,
            0x40 => Self::TestBit0_B_0x40,
            0x41 => Self::TestBit0_C_0x41,
            0x42 => Self::TestBit0_D_0x42,
            0x43 => Self::TestBit0_E_0x43,
            0x44 => Self::TestBit0_H_0x44,
            0x45 => Self::TestBit0_L_0x45,
            0x46 => Self::TestBit0_MemoryHL_0x46,
            0x47 => Self::TestBit0_A_0x47,
            0x48 => Self::TestBit1_B_0x48,
            0x49 => Self::TestBit1_C_0x49,
            0x4A => Self::TestBit1_D_0x4A,
            0x4B => Self::TestBit1_E_0x4B,
            0x4C => Self::TestBit1_H_0x4C,
            0x4D => Self::TestBit1_L_0x4D,
            0x4E => Self::TestBit1_MemoryHL_0x4E,
            0x4F => Self::TestBit1_A_0x4F,
            0x50 => Self::TestBit2_B_0x50,
            0x51 => Self::TestBit2_C_0x51,
            0x52 => Self::TestBit2_D_0x52,
            0x53 => Self::TestBit2_E_0x53,
            0x54 => Self::TestBit2_H_0x54,
            0x55 => Self::TestBit2_L_0x55,
            0x56 => Self::TestBit2_MemoryHL_0x56,
            0x57 => Self::TestBit2_A_0x57,
            0x58 => Self::TestBit3_B_0x58,
            0x59 => Self::TestBit3_C_0x59,
            0x5A => Self::TestBit3_D_0x5A,
            0x5B => Self::TestBit3_E_0x5B,
            0x5C => Self::TestBit3_H_0x5C,
            0x5D => Self::TestBit3_L_0x5D,
            0x5E => Self::TestBit3_MemoryHL_0x5E,
            0x5F => Self::TestBit3_A_0x5F,
            0x60 => Self::TestBit4_B_0x60,
            0x61 => Self::TestBit4_C_0x61,
            0x62 => Self::TestBit4_D_0x62,
            0x63 => Self::TestBit4_E_0x63,
            0x64 => Self::TestBit4_H_0x64,
            0x65 => Self::TestBit4_L_0x65,
            0x66 => Self::TestBit4_MemoryHL_0x66,
            0x67 => Self::TestBit4_A_0x67,
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
            Self::ShiftLeftAIntoCarry_0x27 => 0x27,
            Self::ShiftRightBIntoCarry_0x28 => 0x28,
            Self::ShiftRightCIntoCarry_0x29 => 0x29,
            Self::ShiftRightDIntoCarry_0x2A => 0x2A,
            Self::ShiftRightEIntoCarry_0x2B => 0x2B,
            Self::ShiftRightHIntoCarry_0x2C => 0x2C,
            Self::ShiftRightLIntoCarry_0x2D => 0x2D,
            Self::ShiftRightMemoryHLIntoCarry_0x2E => 0x2E,
            Self::ShiftRightAIntoCarry_0x2F => 0x2F,
            Self::SwapB_0x30 => 0x30,
            Self::SwapC_0x31 => 0x31,
            Self::SwapD_0x32 => 0x32,
            Self::SwapE_0x33 => 0x33,
            Self::SwapH_0x34 => 0x34,
            Self::SwapL_0x35 => 0x35,
            Self::SwapMemoryHL_0x36 => 0x36,
            Self::SwapA_0x37 => 0x37,
            Self::ShiftRightB_0x38 => 0x38,
            Self::ShiftRightC_0x39 => 0x39,
            Self::ShiftRightD_0x3A => 0x3A,
            Self::ShiftRightE_0x3B => 0x3B,
            Self::ShiftRightH_0x3C => 0x3C,
            Self::ShiftRightL_0x3D => 0x3D,
            Self::ShiftRightMemoryHL_0x3E => 0x3E,
            Self::ShiftRightA_0x3F => 0x3F,
            Self::TestBit0_B_0x40 => 0x40,
            Self::TestBit0_C_0x41 => 0x41,
            Self::TestBit0_D_0x42 => 0x42,
            Self::TestBit0_E_0x43 => 0x43,
            Self::TestBit0_H_0x44 => 0x44,
            Self::TestBit0_L_0x45 => 0x45,
            Self::TestBit0_MemoryHL_0x46 => 0x46,
            Self::TestBit0_A_0x47 => 0x47,
            Self::TestBit1_B_0x48 => 0x48,
            Self::TestBit1_C_0x49 => 0x49,
            Self::TestBit1_D_0x4A => 0x4A,
            Self::TestBit1_E_0x4B => 0x4B,
            Self::TestBit1_H_0x4C => 0x4C,
            Self::TestBit1_L_0x4D => 0x4D,
            Self::TestBit1_MemoryHL_0x4E => 0x4E,
            Self::TestBit1_A_0x4F => 0x4F,
            Self::TestBit2_B_0x50 => 0x50,
            Self::TestBit2_C_0x51 => 0x51,
            Self::TestBit2_D_0x52 => 0x52,
            Self::TestBit2_E_0x53 => 0x53,
            Self::TestBit2_H_0x54 => 0x54,
            Self::TestBit2_L_0x55 => 0x55,
            Self::TestBit2_MemoryHL_0x56 => 0x56,
            Self::TestBit2_A_0x57 => 0x57,
            Self::TestBit3_B_0x58 => 0x58,
            Self::TestBit3_C_0x59 => 0x59,
            Self::TestBit3_D_0x5A => 0x5A,
            Self::TestBit3_E_0x5B => 0x5B,
            Self::TestBit3_H_0x5C => 0x5C,
            Self::TestBit3_L_0x5D => 0x5D,
            Self::TestBit3_MemoryHL_0x5E => 0x5E,
            Self::TestBit3_A_0x5F => 0x5F,
            Self::TestBit4_B_0x60 => 0x60,
            Self::TestBit4_C_0x61 => 0x61,
            Self::TestBit4_D_0x62 => 0x62,
            Self::TestBit4_E_0x63 => 0x63,
            Self::TestBit4_H_0x64 => 0x64,
            Self::TestBit4_L_0x65 => 0x65,
            Self::TestBit4_MemoryHL_0x66 => 0x66,
            Self::TestBit4_A_0x67 => 0x67,
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
            Self::ShiftLeftAIntoCarry_0x27 => execute_0x27(cpu, memory),
            Self::ShiftRightBIntoCarry_0x28 => execute_0x28(cpu, memory),
            Self::ShiftRightCIntoCarry_0x29 => execute_0x29(cpu, memory),
            Self::ShiftRightDIntoCarry_0x2A => execute_0x2a(cpu, memory),
            Self::ShiftRightEIntoCarry_0x2B => execute_0x2b(cpu, memory),
            Self::ShiftRightHIntoCarry_0x2C => execute_0x2c(cpu, memory),
            Self::ShiftRightLIntoCarry_0x2D => execute_0x2d(cpu, memory),
            Self::ShiftRightMemoryHLIntoCarry_0x2E => execute_0x2e(cpu, memory),
            Self::ShiftRightAIntoCarry_0x2F => execute_0x2f(cpu, memory),
            Self::SwapB_0x30 => execute_0x30(cpu, memory),
            Self::SwapC_0x31 => execute_0x31(cpu, memory),
            Self::SwapD_0x32 => execute_0x32(cpu, memory),
            Self::SwapE_0x33 => execute_0x33(cpu, memory),
            Self::SwapH_0x34 => execute_0x34(cpu, memory),
            Self::SwapL_0x35 => execute_0x35(cpu, memory),
            Self::SwapMemoryHL_0x36 => execute_0x36(cpu, memory),
            Self::SwapA_0x37 => execute_0x37(cpu, memory),
            Self::ShiftRightB_0x38 => execute_0x38(cpu, memory),
            Self::ShiftRightC_0x39 => execute_0x39(cpu, memory),
            Self::ShiftRightD_0x3A => execute_0x3a(cpu, memory),
            Self::ShiftRightE_0x3B => execute_0x3b(cpu, memory),
            Self::ShiftRightH_0x3C => execute_0x3c(cpu, memory),
            Self::ShiftRightL_0x3D => execute_0x3d(cpu, memory),
            Self::ShiftRightMemoryHL_0x3E => execute_0x3e(cpu, memory),
            Self::ShiftRightA_0x3F => execute_0x3f(cpu, memory),
            Self::TestBit0_B_0x40 => execute_0x40(cpu, memory),
            Self::TestBit0_C_0x41 => execute_0x41(cpu, memory),
            Self::TestBit0_D_0x42 => execute_0x42(cpu, memory),
            Self::TestBit0_E_0x43 => execute_0x43(cpu, memory),
            Self::TestBit0_H_0x44 => execute_0x44(cpu, memory),
            Self::TestBit0_L_0x45 => execute_0x45(cpu, memory),
            Self::TestBit0_MemoryHL_0x46 => execute_0x46(cpu, memory),
            Self::TestBit0_A_0x47 => execute_0x47(cpu, memory),
            Self::TestBit1_B_0x48 => execute_0x48(cpu, memory),
            Self::TestBit1_C_0x49 => execute_0x49(cpu, memory),
            Self::TestBit1_D_0x4A => execute_0x4a(cpu, memory),
            Self::TestBit1_E_0x4B => execute_0x4b(cpu, memory),
            Self::TestBit1_H_0x4C => execute_0x4c(cpu, memory),
            Self::TestBit1_L_0x4D => execute_0x4d(cpu, memory),
            Self::TestBit1_MemoryHL_0x4E => execute_0x4e(cpu, memory),
            Self::TestBit1_A_0x4F => execute_0x4f(cpu, memory),
            Self::TestBit2_B_0x50 => execute_0x50(cpu, memory),
            Self::TestBit2_C_0x51 => execute_0x51(cpu, memory),
            Self::TestBit2_D_0x52 => execute_0x52(cpu, memory),
            Self::TestBit2_E_0x53 => execute_0x53(cpu, memory),
            Self::TestBit2_H_0x54 => execute_0x54(cpu, memory),
            Self::TestBit2_L_0x55 => execute_0x55(cpu, memory),
            Self::TestBit2_MemoryHL_0x56 => execute_0x56(cpu, memory),
            Self::TestBit2_A_0x57 => execute_0x57(cpu, memory),
            Self::TestBit3_B_0x58 => execute_0x58(cpu, memory),
            Self::TestBit3_C_0x59 => execute_0x59(cpu, memory),
            Self::TestBit3_D_0x5A => execute_0x5a(cpu, memory),
            Self::TestBit3_E_0x5B => execute_0x5b(cpu, memory),
            Self::TestBit3_H_0x5C => execute_0x5c(cpu, memory),
            Self::TestBit3_L_0x5D => execute_0x5d(cpu, memory),
            Self::TestBit3_MemoryHL_0x5E => execute_0x5e(cpu, memory),
            Self::TestBit3_A_0x5F => execute_0x5f(cpu, memory),
            Self::TestBit4_B_0x60 => execute_0x60(cpu, memory),
            Self::TestBit4_C_0x61 => execute_0x61(cpu, memory),
            Self::TestBit4_D_0x62 => execute_0x62(cpu, memory),
            Self::TestBit4_E_0x63 => execute_0x63(cpu, memory),
            Self::TestBit4_H_0x64 => execute_0x64(cpu, memory),
            Self::TestBit4_L_0x65 => execute_0x65(cpu, memory),
            Self::TestBit4_MemoryHL_0x66 => execute_0x66(cpu, memory),
            Self::TestBit4_A_0x67 => execute_0x67(cpu, memory),
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

fn execute_0x27(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_left_8bit_register_into_carry(register::ID::A);
}

fn execute_0x28(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::B);
}

fn execute_0x29(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::C);
}

fn execute_0x2a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::D);
}

fn execute_0x2b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::E);
}

fn execute_0x2c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::H);
}

fn execute_0x2d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::L);
}

fn execute_0x2e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_memory_into_carry(memory, usize::from(cpu.hl.word()));
}

fn execute_0x2f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register_into_carry(register::ID::A);
}

fn execute_0x30(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::B);
}

fn execute_0x31(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::C);
}

fn execute_0x32(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::D);
}

fn execute_0x33(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::E);
}

fn execute_0x34(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::H);
}

fn execute_0x35(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::L);
}

fn execute_0x36(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_memory(memory, usize::from(cpu.hl.word()));
}

fn execute_0x37(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.swap_8bit_register(register::ID::A);
}

fn execute_0x38(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::B);
}

fn execute_0x39(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::C);
}

fn execute_0x3a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::D);
}

fn execute_0x3b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::E);
}

fn execute_0x3c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::H);
}

fn execute_0x3d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::L);
}

fn execute_0x3e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_memory(memory, usize::from(cpu.hl.word()));
}

fn execute_0x3f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.shift_right_8bit_register(register::ID::A);
}

fn execute_0x40(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::B, 0);
}

fn execute_0x41(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::C, 0);
}

fn execute_0x42(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::D, 0);
}

fn execute_0x43(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::E, 0);
}

fn execute_0x44(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::H, 0);
}

fn execute_0x45(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::L, 0);
}

fn execute_0x46(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit_memory(memory, usize::from(cpu.hl.word()), 0);
}

fn execute_0x47(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::A, 0);
}

fn execute_0x48(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::B, 1);
}

fn execute_0x49(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::C, 1);
}

fn execute_0x4a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::D, 1);
}

fn execute_0x4b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::E, 1);
}

fn execute_0x4c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::H, 1);
}

fn execute_0x4d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::L, 1);
}

fn execute_0x4e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit_memory(memory, usize::from(cpu.hl.word()), 1);
}

fn execute_0x4f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::A, 1);
}

fn execute_0x50(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::B, 2);
}

fn execute_0x51(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::C, 2);
}

fn execute_0x52(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::D, 2);
}

fn execute_0x53(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::E, 2);
}

fn execute_0x54(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::H, 2);
}

fn execute_0x55(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::L, 2);
}

fn execute_0x56(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit_memory(memory, usize::from(cpu.hl.word()), 2);
}

fn execute_0x57(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::A, 2);
}

fn execute_0x58(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::B, 3);
}

fn execute_0x59(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::C, 3);
}

fn execute_0x5a(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::D, 3);
}

fn execute_0x5b(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::E, 3);
}

fn execute_0x5c(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::H, 3);
}

fn execute_0x5d(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::L, 3);
}

fn execute_0x5e(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit_memory(memory, usize::from(cpu.hl.word()), 3);
}

fn execute_0x5f(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::A, 3);
}

fn execute_0x60(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::B, 4);
}

fn execute_0x61(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::C, 4);
}

fn execute_0x62(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::D, 4);
}

fn execute_0x63(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::E, 4);
}

fn execute_0x64(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::H, 4);
}

fn execute_0x65(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::L, 4);
}

fn execute_0x66(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit_memory(memory, usize::from(cpu.hl.word()), 4);
}

fn execute_0x67(cpu: &mut LR35902, memory: &mut impl memory::Interface) -> u32 {
    return cpu.test_bit(register::ID::A, 4);
}
