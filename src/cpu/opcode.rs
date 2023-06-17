#[path = "opcode_test.rs"]
#[cfg(test)]
mod test;

use super::{bit, register, MemoryDriver, lr35902};
use crate::{cpu::LR35902};

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
    RelativeJumpCarry8_0x38,
    AddSPintoHL_0x39,
    LdMemoryHLIntoAPostDec_0x3A,
    DecSP_0x3B,
    IncA_0x3C,
    DecA_0x3D,
    LdImm8IntoA_0x3E,
    ComplimentCarryFlag_0x3F,
    LdBIntoB_0x40,
    LdCIntoB_0x41,
    LdDIntoB_0x42,
    LdEIntoB_0x43,
    LdHIntoB_0x44,
    LdLIntoB_0x45,
    LdMemoryHLIntoB_0x46,
    LdAIntoB_0x47,
    LdBIntoC_0x48,
    LdCIntoC_0x49,
    LdDIntoC_0x4A,
    LdEIntoC_0x4B,
    LdHIntoC_0x4C,
    LdLIntoC_0x4D,
    LdMemoryHLIntoC_0x4E,
    LdAIntoC_0x4F,
    LdBIntoD_0x50,
    LdCIntoD_0x51,
    LdDIntoD_0x52,
    LdEIntoD_0x53,
    LdHIntoD_0x54,
    LdLIntoD_0x55,
    LdMemoryHLIntoD_0x56,
    LdAIntoD_0x57,
    LdBIntoE_0x58,
    LdCIntoE_0x59,
    LdDIntoE_0x5A,
    LdEIntoE_0x5B,
    LdHIntoE_0x5C,
    LdLIntoE_0x5D,
    LdMemoryHLIntoE_0x5E,
    LdAIntoE_0x5F,
    LdBIntoH_0x60,
    LdCIntoH_0x61,
    LdDIntoH_0x62,
    LdEIntoH_0x63,
    LdHIntoH_0x64,
    LdLIntoH_0x65,
    LdMemoryHLIntoH_0x66,
    LdAIntoH_0x67,
    LdCIntoL_0x69,
    LdBIntoL_0x68,
    LdDIntoL_0x6A,
    LdEIntoL_0x6B,
    LdHIntoL_0x6C,
    LdLIntoL_0x6D,
    LdMemoryHLIntoL_0x6E,
    LdAIntoL_0x6F,
    LdBIntoMemoryHL_0x70,
    LdCIntoMemoryHL_0x71,
    LdDIntoMemoryHL_0x72,
    LdEIntoMemoryHL_0x73,
    LdHIntoMemoryHL_0x74,
    LdLIntoMemoryHL_0x75,
    Halt_0x76,
    LdAIntoMemoryHL_0x77,
    LdBIntoA_0x78,
}

impl std::convert::From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Nop_0x00,
            0x01 => Self::LdImm16IntoBC_0x01,
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
            0x38 => Self::RelativeJumpCarry8_0x38,
            0x39 => Self::AddSPintoHL_0x39,
            0x3A => Self::LdMemoryHLIntoAPostDec_0x3A,
            0x3B => Self::DecSP_0x3B,
            0x3C => Self::IncA_0x3C,
            0x3D => Self::DecA_0x3D,
            0x3E => Self::LdImm8IntoA_0x3E,
            0x3F => Self::ComplimentCarryFlag_0x3F,
            0x40 => Self::LdBIntoB_0x40,
            0x41 => Self::LdCIntoB_0x41,
            0x42 => Self::LdDIntoB_0x42,
            0x43 => Self::LdEIntoB_0x43,
            0x44 => Self::LdHIntoB_0x44,
            0x45 => Self::LdLIntoB_0x45,
            0x46 => Self::LdMemoryHLIntoB_0x46,
            0x47 => Self::LdAIntoB_0x47,
            0x48 => Self::LdBIntoC_0x48,
            0x49 => Self::LdCIntoC_0x49,
            0x4A => Self::LdDIntoC_0x4A,
            0x4B => Self::LdEIntoC_0x4B,
            0x4C => Self::LdHIntoC_0x4C,
            0x4D => Self::LdLIntoC_0x4D,
            0x4E => Self::LdMemoryHLIntoC_0x4E,
            0x4F => Self::LdAIntoC_0x4F,
            0x50 => Self::LdBIntoD_0x50,
            0x51 => Self::LdCIntoD_0x51,
            0x52 => Self::LdDIntoD_0x52,
            0x53 => Self::LdEIntoD_0x53,
            0x54 => Self::LdHIntoD_0x54,
            0x55 => Self::LdLIntoD_0x55,
            0x56 => Self::LdMemoryHLIntoD_0x56,
            0x57 => Self::LdAIntoD_0x57,
            0x58 => Self::LdBIntoE_0x58,
            0x59 => Self::LdCIntoE_0x59,
            0x5A => Self::LdDIntoE_0x5A,
            0x5B => Self::LdEIntoE_0x5B,
            0x5C => Self::LdHIntoE_0x5C,
            0x5D => Self::LdLIntoE_0x5D,
            0x5E => Self::LdMemoryHLIntoE_0x5E,
            0x5F => Self::LdAIntoE_0x5F,
            0x60 => Self::LdBIntoH_0x60,
            0x61 => Self::LdCIntoH_0x61,
            0x62 => Self::LdDIntoH_0x62,
            0x63 => Self::LdEIntoH_0x63,
            0x64 => Self::LdHIntoH_0x64,
            0x65 => Self::LdLIntoH_0x65,
            0x66 => Self::LdMemoryHLIntoH_0x66,
            0x67 => Self::LdAIntoH_0x67,
            0x68 => Self::LdBIntoL_0x68,
            0x69 => Self::LdCIntoL_0x69,
            0x6A => Self::LdDIntoL_0x6A,
            0x6B => Self::LdEIntoL_0x6B,
            0x6C => Self::LdHIntoL_0x6C,
            0x6D => Self::LdLIntoL_0x6D,
            0x6E => Self::LdMemoryHLIntoL_0x6E,
            0x6F => Self::LdAIntoL_0x6F,
            0x70 => Self::LdBIntoMemoryHL_0x70,
            0x71 => Self::LdCIntoMemoryHL_0x71,
            0x72 => Self::LdDIntoMemoryHL_0x72,
            0x73 => Self::LdEIntoMemoryHL_0x73,
            0x74 => Self::LdHIntoMemoryHL_0x74,
            0x75 => Self::LdLIntoMemoryHL_0x75,
            0x76 => Self::Halt_0x76,
            0x77 => Self::LdAIntoMemoryHL_0x77,
            0x78 => Self::LdBIntoA_0x78,
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
            Self::RelativeJumpCarry8_0x38 => 0x38,
            Self::AddSPintoHL_0x39 => 0x39,
            Self::LdMemoryHLIntoAPostDec_0x3A => 0x3A,
            Self::DecSP_0x3B => 0x3B,
            Self::IncA_0x3C => 0x3C,
            Self::DecA_0x3D => 0x3D,
            Self::LdImm8IntoA_0x3E => 0x3E,
            Self::ComplimentCarryFlag_0x3F => 0x3F,
            Self::LdBIntoB_0x40 => 0x40,
            Self::LdCIntoB_0x41 => 0x41,
            Self::LdDIntoB_0x42 => 0x42,
            Self::LdEIntoB_0x43 => 0x43,
            Self::LdHIntoB_0x44 => 0x44,
            Self::LdLIntoB_0x45 => 0x45,
            Self::LdMemoryHLIntoB_0x46 => 0x46,
            Self::LdAIntoB_0x47 => 0x47,
            Self::LdBIntoC_0x48 => 0x48,
            Self::LdCIntoC_0x49 => 0x49,
            Self::LdDIntoC_0x4A => 0x4A,
            Self::LdEIntoC_0x4B => 0x4B,
            Self::LdHIntoC_0x4C => 0x4C,
            Self::LdLIntoC_0x4D => 0x4D,
            Self::LdMemoryHLIntoC_0x4E => 0x4E,
            Self::LdAIntoC_0x4F => 0x4F,
            Self::LdBIntoD_0x50 => 0x50,
            Self::LdCIntoD_0x51 => 0x51,
            Self::LdDIntoD_0x52 => 0x52,
            Self::LdEIntoD_0x53 => 0x53,
            Self::LdHIntoD_0x54 => 0x54,
            Self::LdLIntoD_0x55 => 0x55,
            Self::LdMemoryHLIntoD_0x56 => 0x56,
            Self::LdAIntoD_0x57 => 0x57,
            Self::LdBIntoE_0x58 => 0x58,
            Self::LdCIntoE_0x59 => 0x59,
            Self::LdDIntoE_0x5A => 0x5A,
            Self::LdEIntoE_0x5B => 0x5B,
            Self::LdHIntoE_0x5C => 0x5C,
            Self::LdLIntoE_0x5D => 0x5D,
            Self::LdMemoryHLIntoE_0x5E => 0x5E,
            Self::LdAIntoE_0x5F => 0x5F,
            Self::LdBIntoH_0x60 => 0x60,
            Self::LdCIntoH_0x61 => 0x61,
            Self::LdDIntoH_0x62 => 0x62,
            Self::LdEIntoH_0x63 => 0x63,
            Self::LdHIntoH_0x64 => 0x64,
            Self::LdLIntoH_0x65 => 0x65,
            Self::LdMemoryHLIntoH_0x66 => 0x66,
            Self::LdAIntoH_0x67 => 0x67,
            Self::LdBIntoL_0x68 => 0x68,
            Self::LdCIntoL_0x69 => 0x69,
            Self::LdDIntoL_0x6A => 0x6A,
            Self::LdEIntoL_0x6B => 0x6B,
            Self::LdHIntoL_0x6C => 0x6C,
            Self::LdLIntoL_0x6D => 0x6D,
            Self::LdMemoryHLIntoL_0x6E => 0x6E,
            Self::LdAIntoL_0x6F => 0x6F,
            Self::LdBIntoMemoryHL_0x70 => 0x70,
            Self::LdCIntoMemoryHL_0x71 => 0x71,
            Self::LdDIntoMemoryHL_0x72 => 0x72,
            Self::LdEIntoMemoryHL_0x73 => 0x73,
            Self::LdHIntoMemoryHL_0x74 => 0x74,
            Self::LdLIntoMemoryHL_0x75 => 0x75,
            Self::Halt_0x76 => 0x76,
            Self::LdAIntoMemoryHL_0x77 => 0x77,
            Self::LdBIntoA_0x78 => 0x78,
        }
    }
}

impl Opcode {
    pub fn execute(&self, cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
        match self {
            Self::Nop_0x00 => execute_0x00(cpu, memory),
            Self::LdImm16IntoBC_0x01 => execute_0x01(cpu, memory),
            Self::LdAIntoMemoryBC_0x02 => execute_0x02(cpu, memory),
            Self::IncBC_0x03 => execute_0x03(cpu, memory),
            Self::IncB_0x04 => execute_0x04(cpu, memory),
            Self::DecB_0x05 => execute_0x05(cpu, memory),
            Self::LdImm8IntoB_0x06 => execute_0x06(cpu, memory),
            Self::RotateLeftIntoA_0x07 => execute_0x07(cpu, memory),
            Self::LdSpInto16ImmAddress_0x08 => execute_0x08(cpu, memory),
            Self::AddBCintoHL_0x09 => execute_0x09(cpu, memory),
            Self::LdMemoryBCIntoA_0x0A => execute_0x0a(cpu, memory),
            Self::DecBC_0x0B => execute_0x0b(cpu, memory),
            Self::IncC_0x0C => execute_0x0c(cpu, memory),
            Self::DecC_0x0D => execute_0x0d(cpu, memory),
            Self::LdImm8IntoC_0x0E => execute_0x0e(cpu, memory),
            Self::RotateRightIntoA_0x0F => execute_0x0f(cpu, memory),
            Self::Stop_0x10 => execute_0x10(cpu, memory),
            Self::LdImm16IntoDE_0x11 => execute_0x11(cpu, memory),
            Self::LdAIntoMemoryDE_0x12 => execute_0x12(cpu, memory),
            Self::IncDE_0x13 => execute_0x13(cpu, memory),
            Self::IncD_0x14 => execute_0x14(cpu, memory),
            Self::DecD_0x15 => execute_0x15(cpu, memory),
            Self::LdImm8IntoD_0x16 => execute_0x16(cpu, memory),
            Self::RotateLeftWithCarryIntoA_0x17 => execute_0x17(cpu, memory),
            Self::RelativeJump8_0x18 => execute_0x18(cpu, memory),
            Self::AddDEintoHL_0x19 => execute_0x19(cpu, memory),
            Self::LdMemoryDEIntoA_0x1A => execute_0x1a(cpu, memory),
            Self::DecDE_0x1B => execute_0x1b(cpu, memory),
            Self::IncE_0x1C => execute_0x1c(cpu, memory),
            Self::DecE_0x1D => execute_0x1d(cpu, memory),
            Self::LdImm8IntoE_0x1E => execute_0x1e(cpu, memory),
            Self::RotateRightWithCarryIntoA_0x1F => execute_0x1f(cpu, memory),
            Self::RelativeJumpNotZero8_0x20 => execute_0x20(cpu, memory),
            Self::LdImm16IntoHL_0x21 => execute_0x21(cpu, memory),
            Self::LdAIntoMemoryHLPostInc_0x22 => execute_0x22(cpu, memory),
            Self::IncHL_0x23 => execute_0x23(cpu, memory),
            Self::IncH_0x24 => execute_0x24(cpu, memory),
            Self::DecH_0x25 => execute_0x25(cpu, memory),
            Self::LdImm8IntoH_0x26 => execute_0x26(cpu, memory),
            Self::DAA_0x27 => execute_0x27(cpu, memory),
            Self::RelativeJumpZero8_0x28 => execute_0x28(cpu, memory),
            Self::AddHLintoHL_0x29 => execute_0x29(cpu, memory),
            Self::LdMemoryHLIntoAPostInc_0x2A => execute_0x2a(cpu, memory),
            Self::DecHL_0x2B => execute_0x2b(cpu, memory),
            Self::IncL_0x2C => execute_0x2c(cpu, memory),
            Self::DecL_0x2D => execute_0x2d(cpu, memory),
            Self::LdImm8IntoL_0x2E => execute_0x2e(cpu, memory),
            Self::ComplimentA_0x2F => execute_0x2f(cpu, memory),
            Self::RelativeJumpNotCarry8_0x30 => execute_0x30(cpu, memory),
            Self::LdImm16IntoSP_0x31 => execute_0x31(cpu, memory),
            Self::LdAIntoMemoryHLPostDec_0x32 => execute_0x32(cpu, memory),
            Self::IncSP_0x33 => execute_0x33(cpu, memory),
            Self::IncMemoryHL_0x34 => execute_0x34(cpu, memory),
            Self::DecMemoryHL_0x35 => execute_0x35(cpu, memory),
            Self::LdImm8IntoMemoryHL_0x36 => execute_0x36(cpu, memory),
            Self::SetCarryFlag_0x37 => execute_0x37(cpu, memory),
            Self::RelativeJumpCarry8_0x38 => execute_0x38(cpu, memory),
            Self::AddSPintoHL_0x39 => execute_0x39(cpu, memory),
            Self::LdMemoryHLIntoAPostDec_0x3A => execute_0x3a(cpu, memory),
            Self::DecSP_0x3B => execute_0x3b(cpu, memory),
            Self::IncA_0x3C => execute_0x3c(cpu, memory),
            Self::DecA_0x3D => execute_0x3d(cpu, memory),
            Self::LdImm8IntoA_0x3E => execute_0x3e(cpu, memory),
            Self::ComplimentCarryFlag_0x3F => execute_0x3f(cpu, memory),
            Self::LdBIntoB_0x40 => execute_0x40(cpu, memory),
            Self::LdCIntoB_0x41 => execute_0x41(cpu, memory),
            Self::LdDIntoB_0x42 => execute_0x42(cpu, memory),
            Self::LdEIntoB_0x43 => execute_0x43(cpu, memory),
            Self::LdHIntoB_0x44 => execute_0x44(cpu, memory),
            Self::LdLIntoB_0x45 => execute_0x45(cpu, memory),
            Self::LdMemoryHLIntoB_0x46 => execute_0x46(cpu, memory),
            Self::LdAIntoB_0x47 => execute_0x47(cpu, memory),
            Self::LdBIntoC_0x48 => execute_0x48(cpu, memory),
            Self::LdCIntoC_0x49 => execute_0x49(cpu, memory),
            Self::LdDIntoC_0x4A => execute_0x4a(cpu, memory),
            Self::LdEIntoC_0x4B => execute_0x4b(cpu, memory),
            Self::LdHIntoC_0x4C => execute_0x4c(cpu, memory),
            Self::LdLIntoC_0x4D => execute_0x4d(cpu, memory),
            Self::LdMemoryHLIntoC_0x4E => execute_0x4e(cpu, memory),
            Self::LdAIntoC_0x4F => execute_0x4f(cpu, memory),
            Self::LdBIntoD_0x50 => execute_0x50(cpu, memory),
            Self::LdCIntoD_0x51 => execute_0x51(cpu, memory),
            Self::LdDIntoD_0x52 => execute_0x52(cpu, memory),
            Self::LdEIntoD_0x53 => execute_0x53(cpu, memory),
            Self::LdHIntoD_0x54 => execute_0x54(cpu, memory),
            Self::LdLIntoD_0x55 => execute_0x55(cpu, memory),
            Self::LdMemoryHLIntoD_0x56 => execute_0x56(cpu, memory),
            Self::LdAIntoD_0x57 => execute_0x57(cpu, memory),
            Self::LdBIntoE_0x58 => execute_0x58(cpu, memory),
            Self::LdCIntoE_0x59 => execute_0x59(cpu, memory),
            Self::LdDIntoE_0x5A => execute_0x5a(cpu, memory),
            Self::LdEIntoE_0x5B => execute_0x5b(cpu, memory),
            Self::LdHIntoE_0x5C => execute_0x5c(cpu, memory),
            Self::LdLIntoE_0x5D => execute_0x5d(cpu, memory),
            Self::LdMemoryHLIntoE_0x5E => execute_0x5e(cpu, memory),
            Self::LdAIntoE_0x5F => execute_0x5f(cpu, memory),
            Self::LdBIntoH_0x60 => execute_0x60(cpu, memory),
            Self::LdCIntoH_0x61 => execute_0x61(cpu, memory),
            Self::LdDIntoH_0x62 => execute_0x62(cpu, memory),
            Self::LdEIntoH_0x63 => execute_0x63(cpu, memory),
            Self::LdHIntoH_0x64 => execute_0x64(cpu, memory),
            Self::LdLIntoH_0x65 => execute_0x65(cpu, memory),
            Self::LdMemoryHLIntoH_0x66 => execute_0x66(cpu, memory),
            Self::LdAIntoH_0x67 => execute_0x67(cpu, memory),
            Self::LdBIntoL_0x68 => execute_0x68(cpu, memory),
            Self::LdCIntoL_0x69 => execute_0x69(cpu, memory),
            Self::LdDIntoL_0x6A => execute_0x6a(cpu, memory),
            Self::LdEIntoL_0x6B => execute_0x6b(cpu, memory),
            Self::LdHIntoL_0x6C => execute_0x6c(cpu, memory),
            Self::LdLIntoL_0x6D => execute_0x6d(cpu, memory),
            Self::LdMemoryHLIntoL_0x6E => execute_0x6e(cpu, memory),
            Self::LdAIntoL_0x6F => execute_0x6f(cpu, memory),
            Self::LdBIntoMemoryHL_0x70 => execute_0x70(cpu, memory),
            Self::LdCIntoMemoryHL_0x71 => execute_0x71(cpu, memory),
            Self::LdDIntoMemoryHL_0x72 => execute_0x72(cpu, memory),
            Self::LdEIntoMemoryHL_0x73 => execute_0x73(cpu, memory),
            Self::LdHIntoMemoryHL_0x74 => execute_0x74(cpu, memory),
            Self::LdLIntoMemoryHL_0x75 => execute_0x75(cpu, memory),
            Self::Halt_0x76 => execute_0x76(cpu, memory),
            Self::LdAIntoMemoryHL_0x77 => execute_0x77(cpu, memory),
            Self::LdBIntoA_0x78 => execute_0x78(cpu, memory),
        }
    }
}

fn execute_0x00(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    4
}

fn execute_0x01(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into BC failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into BC failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x02(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    memory.write(usize::from(cpu.bc.word()), cpu.af.hi);

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x03(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.set_word(cpu.bc.word().wrapping_add(1));

    8
}

fn execute_0x04(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::B);

    4
}

fn execute_0x05(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::B);

    4
}

fn execute_0x06(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x07(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
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

fn execute_0x08(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_address_byte = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x08 failed to load lo address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_address_byte = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x08 failed to load hi address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let mut addr = usize::from(u16::from(hi_address_byte) << 8 | u16::from(lo_address_byte));

    memory.write(addr, cpu.sp.to_be_bytes()[1]);
    addr += 1;
    memory.write(addr, cpu.sp.to_be_bytes()[0]);

    20
}

pub fn execute_0x09(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::BC);

    8
}

fn execute_0x0a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match memory.read(usize::from(cpu.bc.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x0A failed to load byte from memory pointed to by BC. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x0b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_bc = cpu.bc.word().wrapping_sub(1);

    cpu.bc.set_word(new_bc);

    8
}

fn execute_0x0c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::C);

    4
}

fn execute_0x0d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::C);

    4
}

fn execute_0x0e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x0f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
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

fn execute_0x10(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.paused = true;

    4
}

fn execute_0x11(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x12(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    memory.write(usize::from(cpu.de.word()), cpu.af.hi);

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x13(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.set_word(cpu.de.word().wrapping_add(1));

    8
}

fn execute_0x14(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::D);

    4
}

fn execute_0x15(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::D);

    4
}

fn execute_0x16(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x17(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
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

fn execute_0x18(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let relative_addr = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x19(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::DE);

    8
}

fn execute_0x1a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match memory.read(usize::from(cpu.de.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x1A failed to load byte from memory pointed to by DE. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x1b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_de = cpu.de.word().wrapping_sub(1);

    cpu.de.set_word(new_de);

    8
}

fn execute_0x1c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::E);

    4
}

fn execute_0x1d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::E);

    4
}

fn execute_0x1e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x1f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
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

fn execute_0x20(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x21(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x22(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.af.hi);

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x23(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    8
}

fn execute_0x24(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::H);

    4
}

fn execute_0x25(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::H);

    4
}

fn execute_0x26(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x27(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
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

fn execute_0x28(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if !cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x29(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::HL);

    8
}

fn execute_0x2a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match memory.read(usize::from(cpu.hl.word())) {
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

fn execute_0x2b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let new_hl = cpu.hl.word().wrapping_sub(1);

    cpu.hl.set_word(new_hl);

    8
}

fn execute_0x2c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::L);

    4
}

fn execute_0x2d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::L);

    4
}

fn execute_0x2e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x2f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.af.hi = cpu.af.hi ^ 0xFF;

    cpu.set_sub_flag();
    cpu.set_half_carry_flag();

    4
}

fn execute_0x30(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if cpu.test_carry_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x31(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_byte = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into SP failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_byte = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x32(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.af.hi);

    cpu.hl.set_word(cpu.hl.word().wrapping_sub(1));

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x33(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.sp = cpu.sp.wrapping_add(1);

    8
}

fn execute_0x34(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let mut byte = match memory.read(usize::from(cpu.hl.word())) {
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

    memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x35(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let mut byte = match memory.read(usize::from(cpu.hl.word())) {
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

    memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x36(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into Memoryy pointed to by HL failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), byte);

    12
}

fn execute_0x37(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.reset_sub_flag();
    cpu.reset_half_carry_flag();
    cpu.set_carry_flag();

    4
}

fn execute_0x38(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if !cpu.test_carry_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc)) {
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

fn execute_0x39(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::SP);

    8
}

fn execute_0x3a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let value = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x3A failed to load byte from memory pointed to by HL. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    cpu.hl.set_word(cpu.hl.word().wrapping_sub(1));

    8
}

fn execute_0x3b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.sp = cpu.sp.wrapping_sub(1);

    8
}

fn execute_0x3c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.increment_8_bit_register(register::ID::A);

    4
}

fn execute_0x3d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.decrement_8_bit_register(register::ID::A);

    4
}

fn execute_0x3e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.pc)) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into A failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = byte;

    cpu.pc = cpu.pc.wrapping_add(1);

    8
}

fn execute_0x3f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    if cpu.test_carry_flag() {
        cpu.reset_carry_flag();
    } else {
        cpu.set_carry_flag();
    }

    cpu.reset_sub_flag();
    cpu.reset_half_carry_flag();

    4
}

fn execute_0x40(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.bc.hi;

    4
}

fn execute_0x41(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.bc.lo;

    4
}

fn execute_0x42(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.de.hi;

    4
}

fn execute_0x43(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.de.lo;

    4
}

fn execute_0x44(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.hl.hi;

    4
}

fn execute_0x45(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.hl.lo;

    4
}

fn execute_0x46(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into B failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.bc.hi = byte;

    8
}

fn execute_0x47(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = cpu.af.hi;

    4
}

fn execute_0x48(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.bc.hi;

    4
}

fn execute_0x49(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.bc.lo;

    4
}

fn execute_0x4a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.de.hi;

    4
}

fn execute_0x4b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.de.lo;

    4
}

fn execute_0x4c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.hl.hi;

    4
}

fn execute_0x4d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.hl.lo;

    4
}

fn execute_0x4e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte, 
        None => panic!(
            "opcode load memory pointed by HL into C failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.bc.lo = byte;

    8
}

fn execute_0x4f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.lo = cpu.af.hi;

    4
}

fn execute_0x50(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.bc.hi;

    4
}

fn execute_0x51(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.bc.lo;

    4
}

fn execute_0x52(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.de.hi;

    4
}

fn execute_0x53(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.de.lo;

    4
}

fn execute_0x54(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.hl.hi;

    4
}

fn execute_0x55(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.hl.lo;

    4
}

fn execute_0x56(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into D failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.hi = byte;

    8
}

fn execute_0x57(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = cpu.af.hi;

    4
}

fn execute_0x58(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.bc.hi;

    4
}

fn execute_0x59(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.bc.lo;

    4
}

fn execute_0x5a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.de.hi;

    4
}

fn execute_0x5b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.de.lo;

    4
}

fn execute_0x5c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.hl.hi;

    4
}

fn execute_0x5d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.hl.lo;

    4
}

fn execute_0x5e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into E failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.lo = byte;

    8
}

fn execute_0x5f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.lo = cpu.af.hi;

    4
}

fn execute_0x60(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.bc.hi;

    4
}

fn execute_0x61(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.bc.lo;

    4
}

fn execute_0x62(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.de.hi;

    4
}

fn execute_0x63(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.de.lo;

    4
}

fn execute_0x64(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.hl.hi;

    4
}

fn execute_0x65(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.hl.lo;

    4
}

fn execute_0x66(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into H failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.hi = byte;

    8
}

fn execute_0x67(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = cpu.af.hi;

    4
}

fn execute_0x68(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.bc.hi;

    4
}

fn execute_0x69(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.bc.lo;

    4
}

fn execute_0x6a(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.de.hi;

    4
}

fn execute_0x6b(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.de.lo;

    4
}

fn execute_0x6c(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.hl.hi;

    4
}

fn execute_0x6d(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.hl.lo;

    4
}

fn execute_0x6e(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let byte = match memory.read(usize::from(cpu.hl.word())) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into E failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.lo = byte;

    8
}

fn execute_0x6f(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.lo = cpu.af.hi;

    4
}

fn execute_0x70(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.bc.hi);

    8
}

fn execute_0x71(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.bc.lo);

    8
}

fn execute_0x72(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.de.hi);

    8
}

fn execute_0x73(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.de.lo);

    8
}

fn execute_0x74(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.hl.hi);

    8
}

fn execute_0x75(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.hl.lo);

    8
}

fn execute_0x76(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    let clock_cycles = 4;

    if cpu.interrupt_master_enable {
        cpu.halted = true;
        return clock_cycles
    }

    let interrupt_enable_register = memory.read(lr35902::INTERRUPT_ENABLE_REGISTER_ADDR).unwrap();
    let interrupt_flag_register = memory.read(lr35902::INTERRUPT_FLAG_REGISTER_ADDR).unwrap();

    if (interrupt_enable_register & interrupt_flag_register & 0x1F) == 0x00 {
        cpu.halted = true;
    }

    cpu.bugged_halt = true;

    clock_cycles
}

fn execute_0x77(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), cpu.af.hi);

    8
}

fn execute_0x78(cpu: &mut LR35902, memory: &mut impl MemoryDriver) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.af.hi = cpu.bc.hi;

    4
}
