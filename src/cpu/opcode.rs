use crate::cpu::bit;
use crate::cpu::register;
use crate::cpu;
use crate::cpu::LR35902;
use crate::cpu::opcode_ext::*;
use crate::memory;
use crate::memory::io_registers::TIMER_DIV_ADDR;
use crate::timers;

use super::bit::two_compliment_byte;

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
    LdCIntoA_0x79,
    LdDIntoA_0x7A,
    LdEIntoA_0x7B,
    LdHIntoA_0x7C,
    LdLIntoA_0x7D,
    LdMemoryHLIntoA_0x7E,
    LdAIntoA_0x7F,
    AddBIntoA_0x80,
    AddCIntoA_0x81,
    AddDIntoA_0x82,
    AddEIntoA_0x83,
    AddHIntoA_0x84,
    AddLIntoA_0x85,
    AddMemoryHLIntoA_0x86,
    AddAIntoA_0x87,
    AddBIntoAWithCarry_0x88,
    AddCIntoAWithCarry_0x89,
    AddDIntoAWithCarry_0x8A,
    AddEIntoAWithCarry_0x8B,
    AddHIntoAWithCarry_0x8C,
    AddLIntoAWithCarry_0x8D,
    AddMemoryHLIntoAWithCarry_0x8E,
    AddAIntoAWithCarry_0x8F,
    SubBFromA_0x90,
    SubCFromA_0x91,
    SubDFromA_0x92,
    SubEFromA_0x93,
    SubHFromA_0x94,
    SubLFromA_0x95,
    SubMemoryHLFromA_0x96,
    SubAFromA_0x97,
    SubBFromAWithCarry_0x98,
    SubCFromAWithCarry_0x99,
    SubDFromAWithCarry_0x9A,
    SubEFromAWithCarry_0x9B,
    SubHFromAWithCarry_0x9C,
    SubLFromAWithCarry_0x9D,
    SubMemoryHLFromAWithCarry_0x9E,
    SubAFromAWithCarry_0x9F,
    AndBIntoA_0xA0,
    AndCIntoA_0xA1,
    AndDIntoA_0xA2,
    AndEIntoA_0xA3,
    AndHIntoA_0xA4,
    AndLIntoA_0xA5,
    AndMemoryHLIntoA_0xA6,
    AndAIntoA_0xA7,
    XorBIntoA_0xA8,
    XorCIntoA_0xA9,
    XorDIntoA_0xAA,
    XorEIntoA_0xAB,
    XorHIntoA_0xAC,
    XorLIntoA_0xAD,
    XorMemoryHLIntoA_0xAE,
    XorAIntoA_0xAF,
    OrBIntoA_0xB0,
    OrCIntoA_0xB1,
    OrDIntoA_0xB2,
    OrEIntoA_0xB3,
    OrHIntoA_0xB4,
    OrLIntoA_0xB5,
    OrMemoryHLIntoA_0xB6,
    OrAIntoA_0xB7,
    CompareBIntoA_0xB8,
    CompareCIntoA_0xB9,
    CompareDIntoA_0xBA,
    CompareEIntoA_0xBB,
    CompareHIntoA_0xBC,
    CompareLIntoA_0xBD,
    CompareMemoryHLIntoA_0xBE,
    CompareAIntoA_0xBF,
    ReturnNotZero_0xC0,
    PopBC_0xC1,
    JumpAbsoluteNotZero_0xC2,
    JumpAbsolute_0xC3,
    CallNotZero_0xC4,
    PushBC_0xC5,
    Add8ImmIntoA_0xC6,
    Reset00h_0xC7,
    ReturnZero_0xC8,
    Return_0xC9,
    JumpAbsoluteZero_0xCA,
    ExtendedOpCode_0xCB,
    CallZero_0xCC,
    Call_0xCD,
    Add8ImmIntoAWithCarry_0xCE,
    Reset08h_0xCF,
    ReturnNotCarry_0xD0,
    PopDE_0xD1,
    JumpAbsoluteNotCarry_0xD2,
    Nop_0xD3,
    CallNotCarry_0xD4,
    PushDE_0xD5,
    Sub8ImmFromA_0xD6,
    Reset10h_0xD7,
    ReturnCarry_0xD8,
    ReturnInterruptMasterEnable_0xD9,
    JumpAbsoluteCarry_0xDA,
    Nop_0xDB,
    CallCarry_0xDC,
    Nop_0xDD,
    Sub8ImmFromAWithCarry_0xDE,
    Reset18h_0xDF,
    LoadAIntoHiMemOffset_0xE0,
    PopHL_0xE1,
    LoadAIntoHiMemOffsetC_0xE2,
    Nop_0xE3,
    Nop_0xE4,
    PushHL_0xE5,
    And8ImmIntoA_0xE6,
    Reset20h_0xE7,
    AddSigned8ImmIntoSP_0xE8,
    JumpMemoryHL_0xE9,
    WriteAInto16ImmAddress_0xEA,
    Nop_0xEB,
    Nop_0xEC,
    Nop_0xED,
    Xor8ImmIntoA_0xEE,
    Reset28h_0xEF,
    LoadHiMemOffsetIntoA_0xF0,
    PopAF_0xF1,
    LoadMemOffsetCIntoA_0xF2,
    DisableInterrupts_0xF3,
    Nop_0xF4,
    PushAF_0xF5,
    Or8ImmIntoA_0xF6,
    Reset30h_0xF7,
    LoadSPSigned8ImmIntoHL_0xF8,
    LoadHLIntoSP_0xF9,
    LoadMemAddrIntoA_0xFA,
    EnableInterrupts_0xFB,
    Nop_0xFC,
    Nop_0xFD,
    CompareAWith8Imm_0xFE,
    Reset38h_0xFF,
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
            0x79 => Self::LdCIntoA_0x79,
            0x7A => Self::LdDIntoA_0x7A,
            0x7B => Self::LdEIntoA_0x7B,
            0x7C => Self::LdHIntoA_0x7C,
            0x7D => Self::LdLIntoA_0x7D,
            0x7E => Self::LdMemoryHLIntoA_0x7E,
            0x7F => Self::LdAIntoA_0x7F,
            0x80 => Self::AddBIntoA_0x80,
            0x81 => Self::AddCIntoA_0x81,
            0x82 => Self::AddDIntoA_0x82,
            0x83 => Self::AddEIntoA_0x83,
            0x84 => Self::AddHIntoA_0x84,
            0x85 => Self::AddLIntoA_0x85,
            0x86 => Self::AddMemoryHLIntoA_0x86,
            0x87 => Self::AddAIntoA_0x87,
            0x88 => Self::AddBIntoAWithCarry_0x88,
            0x89 => Self::AddCIntoAWithCarry_0x89,
            0x8A => Self::AddDIntoAWithCarry_0x8A,
            0x8B => Self::AddEIntoAWithCarry_0x8B,
            0x8C => Self::AddHIntoAWithCarry_0x8C,
            0x8D => Self::AddLIntoAWithCarry_0x8D,
            0x8E => Self::AddMemoryHLIntoAWithCarry_0x8E,
            0x8F => Self::AddAIntoAWithCarry_0x8F,
            0x90 => Self::SubBFromA_0x90,
            0x91 => Self::SubCFromA_0x91,
            0x92 => Self::SubDFromA_0x92,
            0x93 => Self::SubEFromA_0x93,
            0x94 => Self::SubHFromA_0x94,
            0x95 => Self::SubLFromA_0x95,
            0x96 => Self::SubMemoryHLFromA_0x96,
            0x97 => Self::SubAFromA_0x97,
            0x98 => Self::SubBFromAWithCarry_0x98,
            0x99 => Self::SubCFromAWithCarry_0x99,
            0x9A => Self::SubDFromAWithCarry_0x9A,
            0x9B => Self::SubEFromAWithCarry_0x9B,
            0x9C => Self::SubHFromAWithCarry_0x9C,
            0x9D => Self::SubLFromAWithCarry_0x9D,
            0x9E => Self::SubMemoryHLFromAWithCarry_0x9E,
            0x9F => Self::SubAFromAWithCarry_0x9F,
            0xA0 => Self::AndBIntoA_0xA0,
            0xA1 => Self::AndCIntoA_0xA1,
            0xA2 => Self::AndDIntoA_0xA2,
            0xA3 => Self::AndEIntoA_0xA3,
            0xA4 => Self::AndHIntoA_0xA4,
            0xA5 => Self::AndLIntoA_0xA5,
            0xA6 => Self::AndMemoryHLIntoA_0xA6,
            0xA7 => Self::AndAIntoA_0xA7,
            0xA8 => Self::XorBIntoA_0xA8,
            0xA9 => Self::XorCIntoA_0xA9,
            0xAA => Self::XorDIntoA_0xAA,
            0xAB => Self::XorEIntoA_0xAB,
            0xAC => Self::XorHIntoA_0xAC,
            0xAD => Self::XorLIntoA_0xAD,
            0xAE => Self::XorMemoryHLIntoA_0xAE,
            0xAF => Self::XorAIntoA_0xAF,
            0xB0 => Self::OrBIntoA_0xB0,
            0xB1 => Self::OrCIntoA_0xB1,
            0xB2 => Self::OrDIntoA_0xB2,
            0xB3 => Self::OrEIntoA_0xB3,
            0xB4 => Self::OrHIntoA_0xB4,
            0xB5 => Self::OrLIntoA_0xB5,
            0xB6 => Self::OrMemoryHLIntoA_0xB6,
            0xB7 => Self::OrAIntoA_0xB7,
            0xB8 => Self::CompareBIntoA_0xB8,
            0xB9 => Self::CompareCIntoA_0xB9,
            0xBA => Self::CompareDIntoA_0xBA,
            0xBB => Self::CompareEIntoA_0xBB,
            0xBC => Self::CompareHIntoA_0xBC,
            0xBD => Self::CompareLIntoA_0xBD,
            0xBE => Self::CompareMemoryHLIntoA_0xBE,
            0xBF => Self::CompareAIntoA_0xBF,
            0xC0 => Self::ReturnNotZero_0xC0,
            0xC1 => Self::PopBC_0xC1,
            0xC2 => Self::JumpAbsoluteNotZero_0xC2,
            0xC3 => Self::JumpAbsolute_0xC3,
            0xC4 => Self::CallNotZero_0xC4,
            0xC5 => Self::PushBC_0xC5,
            0xC6 => Self::Add8ImmIntoA_0xC6,
            0xC7 => Self::Reset00h_0xC7,
            0xC8 => Self::ReturnZero_0xC8,
            0xC9 => Self::Return_0xC9,
            0xCA => Self::JumpAbsoluteZero_0xCA,
            0xCB => Self::ExtendedOpCode_0xCB,
            0xCC => Self::CallZero_0xCC,
            0xCD => Self::Call_0xCD,
            0xCE => Self::Add8ImmIntoAWithCarry_0xCE,
            0xCF => Self::Reset08h_0xCF,
            0xD0 => Self::ReturnNotCarry_0xD0,
            0xD1 => Self::PopDE_0xD1,
            0xD2 => Self::JumpAbsoluteNotCarry_0xD2,
            0xD3 => Self::Nop_0xD3,
            0xD4 => Self::CallNotCarry_0xD4,
            0xD5 => Self::PushDE_0xD5,
            0xD6 => Self::Sub8ImmFromA_0xD6,
            0xD7 => Self::Reset10h_0xD7,
            0xD8 => Self::ReturnCarry_0xD8,
            0xD9 => Self::ReturnInterruptMasterEnable_0xD9,
            0xDA => Self::JumpAbsoluteCarry_0xDA,
            0xDB => Self::Nop_0xDB,
            0xDC => Self::CallCarry_0xDC,
            0xDD => Self::Nop_0xDD,
            0xDE => Self::Sub8ImmFromAWithCarry_0xDE,
            0xDF => Self::Reset18h_0xDF,
            0xE0 => Self::LoadAIntoHiMemOffset_0xE0,
            0xE1 => Self::PopHL_0xE1,
            0xE2 => Self::LoadAIntoHiMemOffsetC_0xE2,
            0xE3 => Self::Nop_0xE3,
            0xE4 => Self::Nop_0xE4,
            0xE5 => Self::PushHL_0xE5,
            0xE6 => Self::And8ImmIntoA_0xE6,
            0xE7 => Self::Reset20h_0xE7,
            0xE8 => Self::AddSigned8ImmIntoSP_0xE8,
            0xE9 => Self::JumpMemoryHL_0xE9,
            0xEA => Self::WriteAInto16ImmAddress_0xEA,
            0xEB => Self::Nop_0xEB,
            0xEC => Self::Nop_0xEC,
            0xED => Self::Nop_0xED,
            0xEE => Self::Xor8ImmIntoA_0xEE,
            0xEF => Self::Reset28h_0xEF,
            0xF0 => Self::LoadHiMemOffsetIntoA_0xF0,
            0xF1 => Self::PopAF_0xF1,
            0xF2 => Self::LoadMemOffsetCIntoA_0xF2,
            0xF3 => Self::DisableInterrupts_0xF3,
            0xF4 => Self::Nop_0xF4,
            0xF5 => Self::PushAF_0xF5,
            0xF6 => Self::Or8ImmIntoA_0xF6,
            0xF7 => Self::Reset30h_0xF7,
            0xF8 => Self::LoadSPSigned8ImmIntoHL_0xF8,
            0xF9 => Self::LoadHLIntoSP_0xF9,
            0xFA => Self::LoadMemAddrIntoA_0xFA,
            0xFB => Self::EnableInterrupts_0xFB,
            0xFC => Self::Nop_0xFC,
            0xFD => Self::Nop_0xFD,
            0xFE => Self::CompareAWith8Imm_0xFE,
            0xFF => Self::Reset38h_0xFF,
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
            Self::LdCIntoA_0x79 => 0x79,
            Self::LdDIntoA_0x7A => 0x7A,
            Self::LdEIntoA_0x7B => 0x7B,
            Self::LdHIntoA_0x7C => 0x7C,
            Self::LdLIntoA_0x7D => 0x7D,
            Self::LdMemoryHLIntoA_0x7E => 0x7E,
            Self::LdAIntoA_0x7F => 0x7F,
            Self::AddBIntoA_0x80 => 0x80,
            Self::AddCIntoA_0x81 => 0x81,
            Self::AddDIntoA_0x82 => 0x82,
            Self::AddEIntoA_0x83 => 0x83,
            Self::AddHIntoA_0x84 => 0x84,
            Self::AddLIntoA_0x85 => 0x85,
            Self::AddMemoryHLIntoA_0x86 => 0x86,
            Self::AddAIntoA_0x87 => 0x87,
            Self::AddBIntoAWithCarry_0x88 => 0x88,
            Self::AddCIntoAWithCarry_0x89 => 0x89,
            Self::AddDIntoAWithCarry_0x8A => 0x8A,
            Self::AddEIntoAWithCarry_0x8B => 0x8B,
            Self::AddHIntoAWithCarry_0x8C => 0x8C,
            Self::AddLIntoAWithCarry_0x8D => 0x8D,
            Self::AddMemoryHLIntoAWithCarry_0x8E => 0x8E,
            Self::AddAIntoAWithCarry_0x8F => 0x8F,
            Self::SubBFromA_0x90 => 0x90,
            Self::SubCFromA_0x91 => 0x91,
            Self::SubDFromA_0x92 => 0x92,
            Self::SubEFromA_0x93 => 0x93,
            Self::SubHFromA_0x94 => 0x94,
            Self::SubLFromA_0x95 => 0x95,
            Self::SubMemoryHLFromA_0x96 => 0x96,
            Self::SubAFromA_0x97 => 0x97,
            Self::SubBFromAWithCarry_0x98 => 0x98,
            Self::SubCFromAWithCarry_0x99 => 0x99,
            Self::SubDFromAWithCarry_0x9A => 0x9A,
            Self::SubEFromAWithCarry_0x9B => 0x9B,
            Self::SubHFromAWithCarry_0x9C => 0x9C,
            Self::SubLFromAWithCarry_0x9D => 0x9D,
            Self::SubMemoryHLFromAWithCarry_0x9E => 0x9E,
            Self::SubAFromAWithCarry_0x9F => 0x9F,
            Self::AndBIntoA_0xA0 => 0xA0,
            Self::AndCIntoA_0xA1 => 0xA1,
            Self::AndDIntoA_0xA2 => 0xA2,
            Self::AndEIntoA_0xA3 => 0xA3,
            Self::AndHIntoA_0xA4 => 0xA4,
            Self::AndLIntoA_0xA5 => 0xA5,
            Self::AndMemoryHLIntoA_0xA6 => 0xA6,
            Self::AndAIntoA_0xA7 => 0xA7,
            Self::XorBIntoA_0xA8 => 0xA8,
            Self::XorCIntoA_0xA9 => 0xA9,
            Self::XorDIntoA_0xAA => 0xAA,
            Self::XorEIntoA_0xAB => 0xAB,
            Self::XorHIntoA_0xAC => 0xAC,
            Self::XorLIntoA_0xAD => 0xAD,
            Self::XorMemoryHLIntoA_0xAE => 0xAE,
            Self::XorAIntoA_0xAF => 0xAF,
            Self::OrBIntoA_0xB0 => 0xB0,
            Self::OrCIntoA_0xB1 => 0xB1,
            Self::OrDIntoA_0xB2 => 0xB2,
            Self::OrEIntoA_0xB3 => 0xB3,
            Self::OrHIntoA_0xB4 => 0xB4,
            Self::OrLIntoA_0xB5 => 0xB5,
            Self::OrMemoryHLIntoA_0xB6 => 0xB6,
            Self::OrAIntoA_0xB7 => 0xB7,
            Self::CompareBIntoA_0xB8 => 0xB8,
            Self::CompareCIntoA_0xB9 => 0xB9,
            Self::CompareDIntoA_0xBA => 0xBA,
            Self::CompareEIntoA_0xBB => 0xBB,
            Self::CompareHIntoA_0xBC => 0xBC,
            Self::CompareLIntoA_0xBD => 0xBD,
            Self::CompareMemoryHLIntoA_0xBE => 0xBE,
            Self::CompareAIntoA_0xBF => 0xBF,
            Self::ReturnNotZero_0xC0 => 0xC0,
            Self::PopBC_0xC1 => 0xC1,
            Self::JumpAbsoluteNotZero_0xC2 => 0xC2,
            Self::JumpAbsolute_0xC3 => 0xC3,
            Self::CallNotZero_0xC4 => 0xC4,
            Self::PushBC_0xC5 => 0xC5,
            Self::Add8ImmIntoA_0xC6 => 0xC6,
            Self::Reset00h_0xC7 => 0xC7,
            Self::ReturnZero_0xC8 => 0xC8,
            Self::Return_0xC9 => 0xC9,
            Self::JumpAbsoluteZero_0xCA => 0xCA,
            Self::ExtendedOpCode_0xCB => 0xCB,
            Self::CallZero_0xCC => 0xCC,
            Self::Call_0xCD => 0xCD,
            Self::Add8ImmIntoAWithCarry_0xCE => 0xCE,
            Self::Reset08h_0xCF => 0xCF,
            Self::ReturnNotCarry_0xD0 => 0xD0,
            Self::PopDE_0xD1 => 0xD1,
            Self::JumpAbsoluteNotCarry_0xD2 => 0xD2,
            Self::Nop_0xD3 => 0xD3,
            Self::CallNotCarry_0xD4 => 0xD4,
            Self::PushDE_0xD5 => 0xD5,
            Self::Sub8ImmFromA_0xD6 => 0xD6,
            Self::Reset10h_0xD7 => 0xD7,
            Self::ReturnCarry_0xD8 => 0xD8,
            Self::ReturnInterruptMasterEnable_0xD9 => 0xD9,
            Self::JumpAbsoluteCarry_0xDA => 0xDA,
            Self::Nop_0xDB => 0xDB,
            Self::CallCarry_0xDC => 0xDC,
            Self::Nop_0xDD => 0xDD,
            Self::Sub8ImmFromAWithCarry_0xDE => 0xDE,
            Self::Reset18h_0xDF => 0xDF,
            Self::LoadAIntoHiMemOffset_0xE0 => 0xE0,
            Self::PopHL_0xE1 => 0xE1,
            Self::LoadAIntoHiMemOffsetC_0xE2 => 0xE2,
            Self::Nop_0xE3 => 0xE3,
            Self::Nop_0xE4 => 0xE4,
            Self::PushHL_0xE5 => 0xE5,
            Self::And8ImmIntoA_0xE6 => 0xE6,
            Self::Reset20h_0xE7 => 0xE7,
            Self::AddSigned8ImmIntoSP_0xE8 => 0xE8,
            Self::JumpMemoryHL_0xE9 => 0xE9,
            Self::WriteAInto16ImmAddress_0xEA => 0xEA,
            Self::Nop_0xEB => 0xEB,
            Self::Nop_0xEC => 0xEC,
            Self::Nop_0xED => 0xED,
            Self::Xor8ImmIntoA_0xEE => 0xEE,
            Self::Reset28h_0xEF => 0xEF,
            Self::LoadHiMemOffsetIntoA_0xF0 => 0xF0,
            Self::PopAF_0xF1 => 0xF1,
            Self::LoadMemOffsetCIntoA_0xF2 => 0xF2,
            Self::DisableInterrupts_0xF3 => 0xF3,
            Self::Nop_0xF4 => 0xF4,
            Self::PushAF_0xF5 => 0xF5,
            Self::Or8ImmIntoA_0xF6 => 0xF6,
            Self::Reset30h_0xF7 => 0xF7,
            Self::LoadSPSigned8ImmIntoHL_0xF8 => 0xF8,
            Self::LoadHLIntoSP_0xF9 => 0xF9,
            Self::LoadMemAddrIntoA_0xFA => 0xFA,
            Self::EnableInterrupts_0xFB => 0xFB,
            Self::Nop_0xFC => 0xFC,
            Self::Nop_0xFD => 0xFD,
            Self::CompareAWith8Imm_0xFE => 0xFE,
            Self::Reset38h_0xFF => 0xFF,
        }
    }
}

impl Opcode {
    pub fn execute(&self, cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
        match self {
            Self::Nop_0x00 => execute_0x00(cpu, memory, timers),
            Self::LdImm16IntoBC_0x01 => execute_0x01(cpu, memory, timers),
            Self::LdAIntoMemoryBC_0x02 => execute_0x02(cpu, memory, timers),
            Self::IncBC_0x03 => execute_0x03(cpu, memory, timers),
            Self::IncB_0x04 => execute_0x04(cpu, memory, timers),
            Self::DecB_0x05 => execute_0x05(cpu, memory, timers),
            Self::LdImm8IntoB_0x06 => execute_0x06(cpu, memory, timers),
            Self::RotateLeftIntoA_0x07 => execute_0x07(cpu, memory, timers),
            Self::LdSpInto16ImmAddress_0x08 => execute_0x08(cpu, memory, timers),
            Self::AddBCintoHL_0x09 => execute_0x09(cpu, memory, timers),
            Self::LdMemoryBCIntoA_0x0A => execute_0x0a(cpu, memory, timers),
            Self::DecBC_0x0B => execute_0x0b(cpu, memory, timers),
            Self::IncC_0x0C => execute_0x0c(cpu, memory, timers),
            Self::DecC_0x0D => execute_0x0d(cpu, memory, timers),
            Self::LdImm8IntoC_0x0E => execute_0x0e(cpu, memory, timers),
            Self::RotateRightIntoA_0x0F => execute_0x0f(cpu, memory, timers),
            Self::Stop_0x10 => execute_0x10(cpu, memory, timers),
            Self::LdImm16IntoDE_0x11 => execute_0x11(cpu, memory, timers),
            Self::LdAIntoMemoryDE_0x12 => execute_0x12(cpu, memory, timers),
            Self::IncDE_0x13 => execute_0x13(cpu, memory, timers),
            Self::IncD_0x14 => execute_0x14(cpu, memory, timers),
            Self::DecD_0x15 => execute_0x15(cpu, memory, timers),
            Self::LdImm8IntoD_0x16 => execute_0x16(cpu, memory, timers),
            Self::RotateLeftWithCarryIntoA_0x17 => execute_0x17(cpu, memory, timers),
            Self::RelativeJump8_0x18 => execute_0x18(cpu, memory, timers),
            Self::AddDEintoHL_0x19 => execute_0x19(cpu, memory, timers),
            Self::LdMemoryDEIntoA_0x1A => execute_0x1a(cpu, memory, timers),
            Self::DecDE_0x1B => execute_0x1b(cpu, memory, timers),
            Self::IncE_0x1C => execute_0x1c(cpu, memory, timers),
            Self::DecE_0x1D => execute_0x1d(cpu, memory, timers),
            Self::LdImm8IntoE_0x1E => execute_0x1e(cpu, memory, timers),
            Self::RotateRightWithCarryIntoA_0x1F => execute_0x1f(cpu, memory, timers),
            Self::RelativeJumpNotZero8_0x20 => execute_0x20(cpu, memory, timers),
            Self::LdImm16IntoHL_0x21 => execute_0x21(cpu, memory, timers),
            Self::LdAIntoMemoryHLPostInc_0x22 => execute_0x22(cpu, memory, timers),
            Self::IncHL_0x23 => execute_0x23(cpu, memory, timers),
            Self::IncH_0x24 => execute_0x24(cpu, memory, timers),
            Self::DecH_0x25 => execute_0x25(cpu, memory, timers),
            Self::LdImm8IntoH_0x26 => execute_0x26(cpu, memory, timers),
            Self::DAA_0x27 => execute_0x27(cpu, memory, timers),
            Self::RelativeJumpZero8_0x28 => execute_0x28(cpu, memory, timers),
            Self::AddHLintoHL_0x29 => execute_0x29(cpu, memory, timers),
            Self::LdMemoryHLIntoAPostInc_0x2A => execute_0x2a(cpu, memory, timers),
            Self::DecHL_0x2B => execute_0x2b(cpu, memory, timers),
            Self::IncL_0x2C => execute_0x2c(cpu, memory, timers),
            Self::DecL_0x2D => execute_0x2d(cpu, memory, timers),
            Self::LdImm8IntoL_0x2E => execute_0x2e(cpu, memory, timers),
            Self::ComplimentA_0x2F => execute_0x2f(cpu, memory, timers),
            Self::RelativeJumpNotCarry8_0x30 => execute_0x30(cpu, memory, timers),
            Self::LdImm16IntoSP_0x31 => execute_0x31(cpu, memory, timers),
            Self::LdAIntoMemoryHLPostDec_0x32 => execute_0x32(cpu, memory, timers),
            Self::IncSP_0x33 => execute_0x33(cpu, memory, timers),
            Self::IncMemoryHL_0x34 => execute_0x34(cpu, memory, timers),
            Self::DecMemoryHL_0x35 => execute_0x35(cpu, memory, timers),
            Self::LdImm8IntoMemoryHL_0x36 => execute_0x36(cpu, memory, timers),
            Self::SetCarryFlag_0x37 => execute_0x37(cpu, memory, timers),
            Self::RelativeJumpCarry8_0x38 => execute_0x38(cpu, memory, timers),
            Self::AddSPintoHL_0x39 => execute_0x39(cpu, memory, timers),
            Self::LdMemoryHLIntoAPostDec_0x3A => execute_0x3a(cpu, memory, timers),
            Self::DecSP_0x3B => execute_0x3b(cpu, memory, timers),
            Self::IncA_0x3C => execute_0x3c(cpu, memory, timers),
            Self::DecA_0x3D => execute_0x3d(cpu, memory, timers),
            Self::LdImm8IntoA_0x3E => execute_0x3e(cpu, memory, timers),
            Self::ComplimentCarryFlag_0x3F => execute_0x3f(cpu, memory, timers),
            Self::LdBIntoB_0x40 => execute_0x40(cpu, memory, timers),
            Self::LdCIntoB_0x41 => execute_0x41(cpu, memory, timers),
            Self::LdDIntoB_0x42 => execute_0x42(cpu, memory, timers),
            Self::LdEIntoB_0x43 => execute_0x43(cpu, memory, timers),
            Self::LdHIntoB_0x44 => execute_0x44(cpu, memory, timers),
            Self::LdLIntoB_0x45 => execute_0x45(cpu, memory, timers),
            Self::LdMemoryHLIntoB_0x46 => execute_0x46(cpu, memory, timers),
            Self::LdAIntoB_0x47 => execute_0x47(cpu, memory, timers),
            Self::LdBIntoC_0x48 => execute_0x48(cpu, memory, timers),
            Self::LdCIntoC_0x49 => execute_0x49(cpu, memory, timers),
            Self::LdDIntoC_0x4A => execute_0x4a(cpu, memory, timers),
            Self::LdEIntoC_0x4B => execute_0x4b(cpu, memory, timers),
            Self::LdHIntoC_0x4C => execute_0x4c(cpu, memory, timers),
            Self::LdLIntoC_0x4D => execute_0x4d(cpu, memory, timers),
            Self::LdMemoryHLIntoC_0x4E => execute_0x4e(cpu, memory, timers),
            Self::LdAIntoC_0x4F => execute_0x4f(cpu, memory, timers),
            Self::LdBIntoD_0x50 => execute_0x50(cpu, memory, timers),
            Self::LdCIntoD_0x51 => execute_0x51(cpu, memory, timers),
            Self::LdDIntoD_0x52 => execute_0x52(cpu, memory, timers),
            Self::LdEIntoD_0x53 => execute_0x53(cpu, memory, timers),
            Self::LdHIntoD_0x54 => execute_0x54(cpu, memory, timers),
            Self::LdLIntoD_0x55 => execute_0x55(cpu, memory, timers),
            Self::LdMemoryHLIntoD_0x56 => execute_0x56(cpu, memory, timers),
            Self::LdAIntoD_0x57 => execute_0x57(cpu, memory, timers),
            Self::LdBIntoE_0x58 => execute_0x58(cpu, memory, timers),
            Self::LdCIntoE_0x59 => execute_0x59(cpu, memory, timers),
            Self::LdDIntoE_0x5A => execute_0x5a(cpu, memory, timers),
            Self::LdEIntoE_0x5B => execute_0x5b(cpu, memory, timers),
            Self::LdHIntoE_0x5C => execute_0x5c(cpu, memory, timers),
            Self::LdLIntoE_0x5D => execute_0x5d(cpu, memory, timers),
            Self::LdMemoryHLIntoE_0x5E => execute_0x5e(cpu, memory, timers),
            Self::LdAIntoE_0x5F => execute_0x5f(cpu, memory, timers),
            Self::LdBIntoH_0x60 => execute_0x60(cpu, memory, timers),
            Self::LdCIntoH_0x61 => execute_0x61(cpu, memory, timers),
            Self::LdDIntoH_0x62 => execute_0x62(cpu, memory, timers),
            Self::LdEIntoH_0x63 => execute_0x63(cpu, memory, timers),
            Self::LdHIntoH_0x64 => execute_0x64(cpu, memory, timers),
            Self::LdLIntoH_0x65 => execute_0x65(cpu, memory, timers),
            Self::LdMemoryHLIntoH_0x66 => execute_0x66(cpu, memory, timers),
            Self::LdAIntoH_0x67 => execute_0x67(cpu, memory, timers),
            Self::LdBIntoL_0x68 => execute_0x68(cpu, memory, timers),
            Self::LdCIntoL_0x69 => execute_0x69(cpu, memory, timers),
            Self::LdDIntoL_0x6A => execute_0x6a(cpu, memory, timers),
            Self::LdEIntoL_0x6B => execute_0x6b(cpu, memory, timers),
            Self::LdHIntoL_0x6C => execute_0x6c(cpu, memory, timers),
            Self::LdLIntoL_0x6D => execute_0x6d(cpu, memory, timers),
            Self::LdMemoryHLIntoL_0x6E => execute_0x6e(cpu, memory, timers),
            Self::LdAIntoL_0x6F => execute_0x6f(cpu, memory, timers),
            Self::LdBIntoMemoryHL_0x70 => execute_0x70(cpu, memory, timers),
            Self::LdCIntoMemoryHL_0x71 => execute_0x71(cpu, memory, timers),
            Self::LdDIntoMemoryHL_0x72 => execute_0x72(cpu, memory, timers),
            Self::LdEIntoMemoryHL_0x73 => execute_0x73(cpu, memory, timers),
            Self::LdHIntoMemoryHL_0x74 => execute_0x74(cpu, memory, timers),
            Self::LdLIntoMemoryHL_0x75 => execute_0x75(cpu, memory, timers),
            Self::Halt_0x76 => execute_0x76(cpu, memory, timers),
            Self::LdAIntoMemoryHL_0x77 => execute_0x77(cpu, memory, timers),
            Self::LdBIntoA_0x78 => execute_0x78(cpu, memory, timers),
            Self::LdCIntoA_0x79 => execute_0x79(cpu, memory, timers),
            Self::LdDIntoA_0x7A => execute_0x7a(cpu, memory, timers),
            Self::LdEIntoA_0x7B => execute_0x7b(cpu, memory, timers),
            Self::LdHIntoA_0x7C => execute_0x7c(cpu, memory, timers),
            Self::LdLIntoA_0x7D => execute_0x7d(cpu, memory, timers),
            Self::LdMemoryHLIntoA_0x7E => execute_0x7e(cpu, memory, timers),
            Self::LdAIntoA_0x7F => execute_0x7f(cpu, memory, timers),
            Self::AddBIntoA_0x80 => execute_0x80(cpu, memory, timers),
            Self::AddCIntoA_0x81 => execute_0x81(cpu, memory, timers),
            Self::AddDIntoA_0x82 => execute_0x82(cpu, memory, timers),
            Self::AddEIntoA_0x83 => execute_0x83(cpu, memory, timers),
            Self::AddHIntoA_0x84 => execute_0x84(cpu, memory, timers),
            Self::AddLIntoA_0x85 => execute_0x85(cpu, memory, timers),
            Self::AddMemoryHLIntoA_0x86 => execute_0x86(cpu, memory, timers),
            Self::AddAIntoA_0x87 => execute_0x87(cpu, memory, timers),
            Self::AddBIntoAWithCarry_0x88 => execute_0x88(cpu, memory, timers),
            Self::AddCIntoAWithCarry_0x89 => execute_0x89(cpu, memory, timers),
            Self::AddDIntoAWithCarry_0x8A => execute_0x8a(cpu, memory, timers),
            Self::AddEIntoAWithCarry_0x8B => execute_0x8b(cpu, memory, timers),
            Self::AddHIntoAWithCarry_0x8C => execute_0x8c(cpu, memory, timers),
            Self::AddLIntoAWithCarry_0x8D => execute_0x8d(cpu, memory, timers),
            Self::AddMemoryHLIntoAWithCarry_0x8E => execute_0x8e(cpu, memory, timers),
            Self::AddAIntoAWithCarry_0x8F => execute_0x8f(cpu, memory, timers),
            Self::SubBFromA_0x90 => execute_0x90(cpu, memory, timers),
            Self::SubCFromA_0x91 => execute_0x91(cpu, memory, timers),
            Self::SubDFromA_0x92 => execute_0x92(cpu, memory, timers),
            Self::SubEFromA_0x93 => execute_0x93(cpu, memory, timers),
            Self::SubHFromA_0x94 => execute_0x94(cpu, memory, timers),
            Self::SubLFromA_0x95 => execute_0x95(cpu, memory, timers),
            Self::SubMemoryHLFromA_0x96 => execute_0x96(cpu, memory, timers),
            Self::SubAFromA_0x97 => execute_0x97(cpu, memory, timers),
            Self::SubBFromAWithCarry_0x98 => execute_0x98(cpu, memory, timers),
            Self::SubCFromAWithCarry_0x99 => execute_0x99(cpu, memory, timers),
            Self::SubDFromAWithCarry_0x9A => execute_0x9a(cpu, memory, timers),
            Self::SubEFromAWithCarry_0x9B => execute_0x9b(cpu, memory, timers),
            Self::SubHFromAWithCarry_0x9C => execute_0x9c(cpu, memory, timers),
            Self::SubLFromAWithCarry_0x9D => execute_0x9d(cpu, memory, timers),
            Self::SubMemoryHLFromAWithCarry_0x9E => execute_0x9e(cpu, memory, timers),
            Self::SubAFromAWithCarry_0x9F => execute_0x9f(cpu, memory, timers),
            Self::AndBIntoA_0xA0 => execute_0xa0(cpu, memory, timers),
            Self::AndCIntoA_0xA1 => execute_0xa1(cpu, memory, timers),
            Self::AndDIntoA_0xA2 => execute_0xa2(cpu, memory, timers),
            Self::AndEIntoA_0xA3 => execute_0xa3(cpu, memory, timers),
            Self::AndHIntoA_0xA4 => execute_0xa4(cpu, memory, timers),
            Self::AndLIntoA_0xA5 => execute_0xa5(cpu, memory, timers),
            Self::AndMemoryHLIntoA_0xA6 => execute_0xa6(cpu, memory, timers),
            Self::AndAIntoA_0xA7 => execute_0xa7(cpu, memory, timers),
            Self::XorBIntoA_0xA8 => execute_0xa8(cpu, memory, timers),
            Self::XorCIntoA_0xA9 => execute_0xa9(cpu, memory, timers),
            Self::XorDIntoA_0xAA => execute_0xaa(cpu, memory, timers),
            Self::XorEIntoA_0xAB => execute_0xab(cpu, memory, timers),
            Self::XorHIntoA_0xAC => execute_0xac(cpu, memory, timers),
            Self::XorLIntoA_0xAD => execute_0xad(cpu, memory, timers),
            Self::XorMemoryHLIntoA_0xAE => execute_0xae(cpu, memory, timers),
            Self::XorAIntoA_0xAF => execute_0xaf(cpu, memory, timers),
            Self::OrBIntoA_0xB0 => execute_0xb0(cpu, memory, timers),
            Self::OrCIntoA_0xB1 => execute_0xb1(cpu, memory, timers),
            Self::OrDIntoA_0xB2 => execute_0xb2(cpu, memory, timers),
            Self::OrEIntoA_0xB3 => execute_0xb3(cpu, memory, timers),
            Self::OrHIntoA_0xB4 => execute_0xb4(cpu, memory, timers),
            Self::OrLIntoA_0xB5 => execute_0xb5(cpu, memory, timers),
            Self::OrMemoryHLIntoA_0xB6 => execute_0xb6(cpu, memory, timers),
            Self::OrAIntoA_0xB7 => execute_0xb7(cpu, memory, timers),
            Self::CompareBIntoA_0xB8 => execute_0xb8(cpu, memory, timers),
            Self::CompareCIntoA_0xB9 => execute_0xb9(cpu, memory, timers),
            Self::CompareDIntoA_0xBA => execute_0xba(cpu, memory, timers),
            Self::CompareEIntoA_0xBB => execute_0xbb(cpu, memory, timers),
            Self::CompareHIntoA_0xBC => execute_0xbc(cpu, memory, timers),
            Self::CompareLIntoA_0xBD => execute_0xbd(cpu, memory, timers),
            Self::CompareMemoryHLIntoA_0xBE => execute_0xbe(cpu, memory, timers),
            Self::CompareAIntoA_0xBF => execute_0xbf(cpu, memory, timers),
            Self::ReturnNotZero_0xC0 => execute_0xc0(cpu, memory, timers),
            Self::PopBC_0xC1 => execute_0xc1(cpu, memory, timers),
            Self::JumpAbsoluteNotZero_0xC2 => execute_0xc2(cpu, memory, timers),
            Self::JumpAbsolute_0xC3 => execute_0xc3(cpu, memory, timers),
            Self::CallNotZero_0xC4 => execute_0xc4(cpu, memory, timers),
            Self::PushBC_0xC5 => execute_0xc5(cpu, memory, timers),
            Self::Add8ImmIntoA_0xC6 => execute_0xc6(cpu, memory, timers),
            Self::Reset00h_0xC7 => execute_0xc7(cpu, memory, timers),
            Self::ReturnZero_0xC8 => execute_0xc8(cpu, memory, timers),
            Self::Return_0xC9 => execute_0xc9(cpu, memory, timers),
            Self::JumpAbsoluteZero_0xCA => execute_0xca(cpu, memory, timers),
            Self::ExtendedOpCode_0xCB => execute_0xcb(cpu, memory, timers),
            Self::CallZero_0xCC => execute_0xcc(cpu, memory, timers),
            Self::Call_0xCD => execute_0xcd(cpu, memory, timers),
            Self::Add8ImmIntoAWithCarry_0xCE => execute_0xce(cpu, memory, timers),
            Self::Reset08h_0xCF => execute_0xcf(cpu, memory, timers),
            Self::ReturnNotCarry_0xD0 => execute_0xd0(cpu, memory, timers),
            Self::PopDE_0xD1 => execute_0xd1(cpu, memory, timers),
            Self::JumpAbsoluteNotCarry_0xD2 => execute_0xd2(cpu, memory, timers),
            Self::Nop_0xD3 => invalid_opcode(Self::Nop_0xD3.into()),
            Self::CallNotCarry_0xD4 => execute_0xd4(cpu, memory, timers),
            Self::PushDE_0xD5 => execute_0xd5(cpu, memory, timers),
            Self::Sub8ImmFromA_0xD6 => execute_0xd6(cpu, memory, timers),
            Self::Reset10h_0xD7 => execute_0xd7(cpu, memory, timers),
            Self::ReturnCarry_0xD8 => execute_0xd8(cpu, memory, timers),
            Self::ReturnInterruptMasterEnable_0xD9 => execute_0xd9(cpu, memory, timers),
            Self::JumpAbsoluteCarry_0xDA => execute_0xda(cpu, memory, timers),
            Self::Nop_0xDB => invalid_opcode(Self::Nop_0xDB.into()),
            Self::CallCarry_0xDC => execute_0xdc(cpu, memory, timers),
            Self::Nop_0xDD => invalid_opcode(Self::Nop_0xDD.into()),
            Self::Sub8ImmFromAWithCarry_0xDE => execute_0xde(cpu, memory, timers),
            Self::Reset18h_0xDF => execute_0xdf(cpu, memory, timers),
            Self::LoadAIntoHiMemOffset_0xE0 => execute_0xe0(cpu, memory, timers),
            Self::PopHL_0xE1 => execute_0xe1(cpu, memory, timers),
            Self::LoadAIntoHiMemOffsetC_0xE2 => execute_0xe2(cpu, memory, timers),
            Self::Nop_0xE3 => invalid_opcode(Self::Nop_0xE3.into()),
            Self::Nop_0xE4 => invalid_opcode(Self::Nop_0xE4.into()),
            Self::PushHL_0xE5 => execute_0xe5(cpu, memory, timers),
            Self::And8ImmIntoA_0xE6 => execute_0xe6(cpu, memory, timers),
            Self::Reset20h_0xE7 => execute_0xe7(cpu, memory, timers),
            Self::AddSigned8ImmIntoSP_0xE8 => execute_0xe8(cpu, memory, timers),
            Self::JumpMemoryHL_0xE9 => execute_0xe9(cpu, memory, timers),
            Self::WriteAInto16ImmAddress_0xEA => execute_0xea(cpu, memory, timers),
            Self::Nop_0xEB => invalid_opcode(Self::Nop_0xEB.into()),
            Self::Nop_0xEC => invalid_opcode(Self::Nop_0xEC.into()),
            Self::Nop_0xED => invalid_opcode(Self::Nop_0xED.into()),
            Self::Xor8ImmIntoA_0xEE => execute_0xee(cpu, memory, timers),
            Self::Reset28h_0xEF => execute_0xef(cpu, memory, timers),
            Self::LoadHiMemOffsetIntoA_0xF0 => execute_0xf0(cpu, memory, timers),
            Self::PopAF_0xF1 => execute_0xf1(cpu, memory, timers),
            Self::LoadMemOffsetCIntoA_0xF2 => execute_0xf2(cpu, memory, timers),
            Self::DisableInterrupts_0xF3 => execute_0xf3(cpu, memory, timers),
            Self::Nop_0xF4 => invalid_opcode(Self::Nop_0xF4.into()),
            Self::PushAF_0xF5 => execute_0xf5(cpu, memory, timers),
            Self::Or8ImmIntoA_0xF6 => execute_0xf6(cpu, memory, timers),
            Self::Reset30h_0xF7 => execute_0xf7(cpu, memory, timers),
            Self::LoadSPSigned8ImmIntoHL_0xF8 => execute_0xf8(cpu, memory, timers),
            Self::LoadHLIntoSP_0xF9 => execute_0xf9(cpu, memory, timers),
            Self::LoadMemAddrIntoA_0xFA => execute_0xfa(cpu, memory, timers),
            Self::EnableInterrupts_0xFB => execute_0xfb(cpu, memory, timers),
            Self::Nop_0xFC => invalid_opcode(Self::Nop_0xFC.into()),
            Self::Nop_0xFD => invalid_opcode(Self::Nop_0xFD.into()),
            Self::CompareAWith8Imm_0xFE => execute_0xfe(cpu, memory, timers),
            Self::Reset38h_0xFF => execute_0xff(cpu, memory, timers),
        }
    }
}

fn invalid_opcode(opcode: u8) -> u32 {
    log::warn!("invalid opcode executed in rom: 0x{:X}", opcode);
    4
}

fn execute_0x00(_: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    4
}

fn execute_0x01(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.bc.lo = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into BC failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.bc.hi = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into BC failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,

        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x02(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.bc.word()), cpu.af.hi, cpu, timers);

    8
}

fn execute_0x03(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.set_word(cpu.bc.word().wrapping_add(1));

    8
}

fn execute_0x04(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::B);

    4
}

fn execute_0x05(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::B);

    4
}

fn execute_0x06(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x07(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
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

fn execute_0x08(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let lo_address_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x08 failed to load lo address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_address_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x08 failed to load hi address byte from memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let mut addr = usize::from(u16::from(hi_address_byte) << 8 | u16::from(lo_address_byte));

    memory.write(addr, cpu.sp.to_be_bytes()[1], cpu, timers);
    addr += 1;
    memory.write(addr, cpu.sp.to_be_bytes()[0], cpu, timers);

    20
}

fn execute_0x09(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::BC);

    8
}

fn execute_0x0a(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let value = match memory.read(usize::from(cpu.bc.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x0A failed to load byte from memory pointed to by BC. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x0b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    let new_bc = cpu.bc.word().wrapping_sub(1);

    cpu.bc.set_word(new_bc);

    8
}

fn execute_0x0c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::C);

    4
}

fn execute_0x0d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::C);

    4
}

fn execute_0x0e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x0f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
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

fn execute_0x10(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.paused = true;

    // This opcode resets the DIV timer register
    // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
    memory.write(TIMER_DIV_ADDR, 0x00, cpu, timers);
    timers.reset_sys_clock();

    4
}

fn execute_0x11(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.de.lo = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.de.hi = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x12(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.de.word()), cpu.af.hi, cpu, timers);

    8
}

fn execute_0x13(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.set_word(cpu.de.word().wrapping_add(1));

    8
}

fn execute_0x14(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::D);

    4
}

fn execute_0x15(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::D);

    4
}

fn execute_0x16(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x17(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
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

fn execute_0x18(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let relative_addr = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x19(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::DE);

    8
}

fn execute_0x1a(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let value = match memory.read(usize::from(cpu.de.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode 0x1A failed to load byte from memory pointed to by DE. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = value;

    8
}

fn execute_0x1b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    let new_de = cpu.de.word().wrapping_sub(1);

    cpu.de.set_word(new_de);

    8
}

fn execute_0x1c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::E);

    4
}

fn execute_0x1d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::E);

    4
}

fn execute_0x1e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x1f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
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

fn execute_0x20(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    if cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x21(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.hl.lo = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.hl.hi = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into DE failed to fetch hi byte. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    12
}

fn execute_0x22(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.af.hi, cpu, timers);

    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    8
}

fn execute_0x23(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.set_word(cpu.hl.word().wrapping_add(1));

    8
}

fn execute_0x24(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::H);

    4
}

fn execute_0x25(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::H);

    4
}

fn execute_0x26(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x27(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
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

fn execute_0x28(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    if !cpu.test_zero_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x29(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::HL);

    8
}

fn execute_0x2a(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let value = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
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

fn execute_0x2b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    let new_hl = cpu.hl.word().wrapping_sub(1);

    cpu.hl.set_word(new_hl);

    8
}

fn execute_0x2c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::L);

    4
}

fn execute_0x2d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::L);

    4
}

fn execute_0x2e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x2f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.af.hi ^ 0xFF;

    cpu.set_sub_flag();
    cpu.set_half_carry_flag();

    4
}

fn execute_0x30(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    if cpu.test_carry_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x31(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let lo_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 16 into SP failed to fetch lo byte. Dumping cpu state...\n{:?}",
            cpu
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x32(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.af.hi, cpu, timers);

    cpu.hl.set_word(cpu.hl.word().wrapping_sub(1));

    8
}

fn execute_0x33(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sp = cpu.sp.wrapping_add(1);

    8
}

fn execute_0x34(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let mut byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
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

    memory.write(usize::from(cpu.hl.word()), byte, cpu, timers);

    12
}

fn execute_0x35(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let mut byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
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

    memory.write(usize::from(cpu.hl.word()), byte, cpu, timers);

    12
}

fn execute_0x36(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load imm 8 into Memory pointed to by HL failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    memory.write(usize::from(cpu.hl.word()), byte, cpu, timers);

    12
}

fn execute_0x37(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.reset_sub_flag();
    cpu.reset_half_carry_flag();
    cpu.set_carry_flag();

    4
}

fn execute_0x38(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    if !cpu.test_carry_flag() {
        cpu.pc = cpu.pc.wrapping_add(1);
        return 8;
    }

    let relative_addr = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x39(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_16_bit_registers(register::ID16::HL, register::ID16::SP);

    8
}

fn execute_0x3a(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let value = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
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

fn execute_0x3b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sp = cpu.sp.wrapping_sub(1);

    8
}

fn execute_0x3c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.increment_8_bit_register(register::ID::A);

    4
}

fn execute_0x3d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.decrement_8_bit_register(register::ID::A);

    4
}

fn execute_0x3e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
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

fn execute_0x3f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    if cpu.test_carry_flag() {
        cpu.reset_carry_flag();
    } else {
        cpu.set_carry_flag();
    }

    cpu.reset_sub_flag();
    cpu.reset_half_carry_flag();

    4
}

fn execute_0x40(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.bc.hi;

    4
}

fn execute_0x41(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.bc.lo;

    4
}

fn execute_0x42(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.de.hi;

    4
}

fn execute_0x43(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.de.lo;

    4
}

fn execute_0x44(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.hl.hi;

    4
}

fn execute_0x45(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.hl.lo;

    4
}

fn execute_0x46(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into B failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.bc.hi = byte;

    8
}

fn execute_0x47(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.hi = cpu.af.hi;

    4
}

fn execute_0x48(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.bc.hi;

    4
}

fn execute_0x49(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.bc.lo;

    4
}

fn execute_0x4a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.de.hi;

    4
}

fn execute_0x4b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.de.lo;

    4
}

fn execute_0x4c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.hl.hi;

    4
}

fn execute_0x4d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.hl.lo;

    4
}

fn execute_0x4e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte, 
        None => panic!(
            "opcode load memory pointed by HL into C failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.bc.lo = byte;

    8
}

fn execute_0x4f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.bc.lo = cpu.af.hi;

    4
}

fn execute_0x50(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.bc.hi;

    4
}

fn execute_0x51(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.bc.lo;

    4
}

fn execute_0x52(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.de.hi;

    4
}

fn execute_0x53(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.de.lo;

    4
}

fn execute_0x54(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.hl.hi;

    4
}

fn execute_0x55(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.hl.lo;

    4
}

fn execute_0x56(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into D failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.hi = byte;

    8
}

fn execute_0x57(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.hi = cpu.af.hi;

    4
}

fn execute_0x58(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.bc.hi;

    4
}

fn execute_0x59(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.bc.lo;

    4
}

fn execute_0x5a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.de.hi;

    4
}

fn execute_0x5b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.de.lo;

    4
}

fn execute_0x5c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.hl.hi;

    4
}

fn execute_0x5d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.hl.lo;

    4
}

fn execute_0x5e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into E failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.de.lo = byte;

    8
}

fn execute_0x5f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.de.lo = cpu.af.hi;

    4
}

fn execute_0x60(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.bc.hi;

    4
}

fn execute_0x61(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.bc.lo;

    4
}

fn execute_0x62(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.de.hi;

    4
}

fn execute_0x63(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.de.lo;

    4
}

fn execute_0x64(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.hl.hi;

    4
}

fn execute_0x65(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.hl.lo;

    4
}

fn execute_0x66(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into H failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.hi = byte;

    8
}

fn execute_0x67(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.hi = cpu.af.hi;

    4
}

fn execute_0x68(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.bc.hi;

    4
}

fn execute_0x69(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.bc.lo;

    4
}

fn execute_0x6a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.de.hi;

    4
}

fn execute_0x6b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.de.lo;

    4
}

fn execute_0x6c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.hl.hi;

    4
}

fn execute_0x6d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.hl.lo;

    4
}

fn execute_0x6e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into E failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.hl.lo = byte;

    8
}

fn execute_0x6f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.hl.lo = cpu.af.hi;

    4
}

fn execute_0x70(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.bc.hi, cpu, timers);

    8
}

fn execute_0x71(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.bc.lo, cpu, timers);

    8
}

fn execute_0x72(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.de.hi, cpu, timers);

    8
}

fn execute_0x73(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.de.lo, cpu, timers);

    8
}

fn execute_0x74(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.hl.hi, cpu, timers);

    8
}

fn execute_0x75(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.hl.lo, cpu, timers);

    8
}

fn execute_0x76(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let clock_cycles = 4;

    if cpu.interrupt_master_enable {
        cpu.halted = true;
        return clock_cycles
    }

    let interrupt_enable_register = memory.read(cpu::INTERRUPT_ENABLE_REGISTER_ADDR, cpu, timers).unwrap();
    let interrupt_flag_register = memory.read(cpu::INTERRUPT_FLAG_REGISTER_ADDR, cpu, timers).unwrap();

    if !((interrupt_enable_register & interrupt_flag_register & 0x1F) > 0x00) {
        cpu.halted = true;
    }

    cpu.bugged_halt = true;

    clock_cycles
}

fn execute_0x77(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    memory.write(usize::from(cpu.hl.word()), cpu.af.hi, cpu, timers);

    8
}

fn execute_0x78(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.bc.hi;

    4
}

fn execute_0x79(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.bc.lo;

    4
}

fn execute_0x7a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.de.hi;

    4
}

fn execute_0x7b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.de.lo;

    4
}

fn execute_0x7c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.hl.hi;

    4
}

fn execute_0x7d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.hl.lo;

    4
}

fn execute_0x7e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let byte = match memory.read(usize::from(cpu.hl.word()), cpu, timers) {
        Some(byte) => byte,
        None => panic!(
            "opcode load memory pointed by HL into A failed to fetch byte in memory. Dumping cpu state...\n{:?}",
            cpu,
        ),
    };

    cpu.af.hi = byte;

    8
}

fn execute_0x7f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.af.hi = cpu.af.hi;

    4
}

fn execute_0x80(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::B, false);

    4
}

fn execute_0x81(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::C, false);

    4
}

fn execute_0x82(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::D, false);

    4
}

fn execute_0x83(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::E, false);

    4
}

fn execute_0x84(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::H, false);

    4
}

fn execute_0x85(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::L, false);

    4
}

fn execute_0x86(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), false, timers);

    8
}

fn execute_0x87(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::A, false);

    4
}

fn execute_0x88(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::B, true);

    4
}

fn execute_0x89(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::C, true);

    4
}

fn execute_0x8a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::D, true);

    4
}

fn execute_0x8b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::E, true);

    4
}

fn execute_0x8c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::H, true);

    4
}

fn execute_0x8d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::L, true);

    4
}

fn execute_0x8e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), true, timers);

    8
}

fn execute_0x8f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_registers(register::ID::A, register::ID::A, true);

    4
}

fn execute_0x90(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::B, false);

    4
}

fn execute_0x91(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::C, false);

    4
}

fn execute_0x92(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::D, false);

    4
}

fn execute_0x93(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::E, false);

    4
}

fn execute_0x94(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::H, false);

    4
}

fn execute_0x95(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::L, false);

    4
}

fn execute_0x96(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), false, timers);

    8
}

fn execute_0x97(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::A, false);

    4
}

fn execute_0x98(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::B, true);

    4
}

fn execute_0x99(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::C, true);

    4
}

fn execute_0x9a(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::D, true);

    4
}

fn execute_0x9b(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::E, true);

    4
}

fn execute_0x9c(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::H, true);

    4
}

fn execute_0x9d(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::L, true);

    4
}

fn execute_0x9e(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), true, timers);

    8
}

fn execute_0x9f(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_registers(register::ID::A, register::ID::A, true);

    4
}

fn execute_0xa0(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::B);

    4
}

fn execute_0xa1(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::C);

    4
}

fn execute_0xa2(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::D);

    4
}

fn execute_0xa3(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::E);

    4
}

fn execute_0xa4(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::H);

    4
}

fn execute_0xa5(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::L);

    4
}

fn execute_0xa6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), timers);

    8
}

fn execute_0xa7(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_registers(register::ID::A, register::ID::A);

    4
}

fn execute_0xa8(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::B);

    4
}

fn execute_0xa9(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::C);

    4
}

fn execute_0xaa(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::D);

    4
}

fn execute_0xab(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::E);

    4
}

fn execute_0xac(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::H);

    4
}

fn execute_0xad(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::L);

    4
}

fn execute_0xae(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), timers);

    8
}

fn execute_0xaf(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_registers(register::ID::A, register::ID::A);

    4
}

fn execute_0xb0(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::B);

    4
}

fn execute_0xb1(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::C);

    4
}

fn execute_0xb2(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::D);

    4
}

fn execute_0xb3(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::E);

    4
}

fn execute_0xb4(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::H);

    4
}

fn execute_0xb5(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::L);

    4
}

fn execute_0xb6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), timers);

    8
}

fn execute_0xb7(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_registers(register::ID::A, register::ID::A);

    4
}

fn execute_0xb8(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::B);

    4
}

fn execute_0xb9(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::C);

    4
}

fn execute_0xba(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::D);

    4
}

fn execute_0xbb(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::E);

    4
}

fn execute_0xbc(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::H);

    4
}

fn execute_0xbd(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::L);

    4
}

fn execute_0xbe(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_memory(register::ID::A, memory, usize::from(cpu.hl.word()), timers);

    8
}

fn execute_0xbf(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_registers(register::ID::A, register::ID::A);

    4
}

fn execute_0xc0(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.return_from_call_conditional(memory, !cpu.test_zero_flag(), timers);
}

fn execute_0xc1(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.pop_stack_into_16_bit_register(register::ID16::BC, memory, timers);
    return 12;
}

fn execute_0xc2(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.jump_to_imm_address(memory, !cpu.test_zero_flag(), timers);
}

fn execute_0xc3(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.jump_to_imm_address(memory, true, timers);
}

fn execute_0xc4(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.call_to_imm_address(memory, !cpu.test_zero_flag(), timers);
}

fn execute_0xc5(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::BC, memory, timers);

    return 16;
}

fn execute_0xc6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), false, timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xc7(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0000;

    return 16;
}

fn execute_0xc8(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.return_from_call_conditional(memory, cpu.test_zero_flag(), timers);
}

fn execute_0xc9(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.return_from_call(memory, timers);
}

fn execute_0xca(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.jump_to_imm_address(memory, cpu.test_zero_flag(), timers);
}

fn execute_0xcb(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let ext_opcode = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => ExtendedOpcode::from(byte),
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    return ext_opcode.execute(cpu, memory, timers) + 4;
}

fn execute_0xcc(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.call_to_imm_address(memory, cpu.test_zero_flag(), timers);
}

fn execute_0xcd(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.call_to_imm_address(memory, true, timers);
}

fn execute_0xce(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.add_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), true, timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xcf(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0008;

    return 16;
}

fn execute_0xd0(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.return_from_call_conditional(memory, !cpu.test_carry_flag(), timers);
}

fn execute_0xd1(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.pop_stack_into_16_bit_register(register::ID16::DE, memory, timers);
    return 12;
}

fn execute_0xd2(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.jump_to_imm_address(memory, !cpu.test_carry_flag(), timers);
}

fn execute_0xd4(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.call_to_imm_address(memory, !cpu.test_carry_flag(), timers);
}

fn execute_0xd5(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::DE, memory, timers);

    return 16;
}

fn execute_0xd6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), false, timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xd7(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0010;

    return 16;
}

fn execute_0xd8(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.return_from_call_conditional(memory, cpu.test_carry_flag(), timers);
}

fn execute_0xd9(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.interrupt_master_enable = true;
    return cpu.return_from_call(memory, timers);
}

fn execute_0xda(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.jump_to_imm_address(memory, cpu.test_carry_flag(), timers);
}

fn execute_0xdc(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    return cpu.call_to_imm_address(memory, cpu.test_carry_flag(), timers);
}

fn execute_0xde(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.sub_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), true, timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xdf(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0018;

    return 16;
}

fn execute_0xe0(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let offset = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO")
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let effective_addr: usize = 0xFF00 + usize::from(offset);

    memory.write(effective_addr, cpu.af.hi, cpu, timers);

    return 12;
}

fn execute_0xe1(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.pop_stack_into_16_bit_register(register::ID16::HL, memory, timers);
    return 12;
}

fn execute_0xe2(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let effective_addr: usize = 0xFF00 + usize::from(cpu.bc.lo);

    memory.write(effective_addr, cpu.af.hi, cpu, timers);

    return 8;
}

fn execute_0xe5(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::HL, memory, timers);

    return 16;
}

fn execute_0xe6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.and_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xe7(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0020;

    return 16;
}

fn execute_0xe8(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let added_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_sp: u8 = cpu.sp.to_be_bytes()[1];

    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    // Negative number
    if bit::test_most_significant_bit(added_byte) {
        if bit::is_half_carry(lo_sp, added_byte, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        if bit::is_carry(lo_sp, added_byte, false) {
            cpu.set_carry_flag();
        } else {
            cpu.reset_carry_flag();
        }

        cpu.sp = cpu.sp.wrapping_sub(two_compliment_byte(added_byte).into());
    } else {
        if bit::is_half_carry(lo_sp, added_byte, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        if bit::is_carry(lo_sp, added_byte, false) {
            cpu.set_carry_flag();
        } else {
            cpu.reset_carry_flag();
        }

        cpu.sp = cpu.sp.wrapping_add(added_byte.into());
    }

    return 16;
}

fn execute_0xe9(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.pc = cpu.hl.word();
    return 4;
}

fn execute_0xea(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let lo_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let effective_addr: usize = (usize::from(hi_byte) << 8) | usize::from(lo_byte);

    memory.write(effective_addr, cpu.af.hi, cpu, timers);

    return 16;
}

fn execute_0xee(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.xor_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), timers);
    cpu.pc = cpu.pc.wrapping_add(1);
    return 8;
}

fn execute_0xef(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0028;

    return 16;
}

fn execute_0xf0(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let offset = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO")
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let effective_addr: usize = 0xFF00 + usize::from(offset);

    let byte = match memory.read(effective_addr, cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.af.hi = byte;

    return 12;
}

fn execute_0xf1(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.pop_stack_into_16_bit_register(register::ID16::AF, memory, timers);
    return 12;
}

fn execute_0xf2(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let effective_addr: usize = 0xFF00 + usize::from(cpu.bc.lo);

    let byte = match memory.read(effective_addr, cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.af.hi = byte;

    return 8;
}

fn execute_0xf3(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.interrupt_master_enable = false;
    return 4;
}

fn execute_0xf5(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::AF, memory, timers);

    return 16;
}

fn execute_0xf6(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.or_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xf7(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0030;

    return 16;
}

fn execute_0xf8(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let added_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let lo_sp: u8 = cpu.sp.to_be_bytes()[1];

    cpu.reset_sub_flag();
    cpu.reset_zero_flag();

    // Negative number
    if bit::test_most_significant_bit(added_byte) {
        if bit::is_half_carry(lo_sp, added_byte, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        if bit::is_carry(lo_sp, added_byte, false) {
            cpu.set_carry_flag();
        } else {
            cpu.reset_carry_flag();
        }

        cpu.hl.set_word(cpu.sp.wrapping_sub(bit::two_compliment_byte(added_byte).into()));
    } else {
        if bit::is_half_carry(lo_sp, added_byte, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        if bit::is_carry(lo_sp, added_byte, false) {
            cpu.set_carry_flag();
        } else {
            cpu.reset_carry_flag();
        }

        cpu.hl.set_word(cpu.sp.wrapping_add(added_byte.into()));
    }

    return 12;
}

fn execute_0xf9(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.sp = cpu.hl.word();

    return 8;
}

fn execute_0xfa(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    let lo_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let hi_byte = match memory.read(usize::from(cpu.pc), cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    cpu.pc = cpu.pc.wrapping_add(1);

    let effective_addr: usize = (usize::from(hi_byte) << 8) | usize::from(lo_byte); 

    cpu.af.hi = match memory.read(effective_addr, cpu, timers) {
        Some(byte) => byte,
        None => panic!("TODO"),
    };

    return 16;
}

fn execute_0xfb(cpu: &mut LR35902, _: &mut memory::Memory, _: &mut timers::Timers) -> u32 {
    cpu.interrupt_master_enable = true;

    return 4;
}

fn execute_0xfe(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.compare_8_bit_memory(register::ID::A, memory, usize::from(cpu.pc), timers);

    cpu.pc = cpu.pc.wrapping_add(1);

    return 8;
}

fn execute_0xff(cpu: &mut LR35902, memory: &mut memory::Memory, timers: &mut timers::Timers) -> u32 {
    cpu.push_16bit_register_on_stack(register::ID16::PC, memory, timers);

    cpu.pc = 0x0038;

    return 16;
}
