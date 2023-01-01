#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "cpu_test.rs"]
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum CpuError {
    InvalidLoadOperands,
}

/// Bit mask for the zero flag
const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
const CARRY_FLAG_MASK: u8 = 1 << 4;

/// ID denoting a register inside the Sharp LR35902 CPU.
/// If a register is byte-addressable, it will have a specific
/// enum entry for that nibble (i.e. register AF will have 2 enums: A and F).
pub enum RegisterID {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
}

/// Represents a byte addressable word register found
/// inside the Sharp LR35902
struct Register {
    hi: u8,
    lo: u8,
}

impl Register {
    fn word(&self) -> u16 {
        let hi: u16 = self.hi.into();
        let lo: u16 = self.lo.into();
        return (hi << 8) | lo;
    }
}

struct Operation {
    clock_cycles: u32,
    exec_fn: fn(&mut LR35902),
}

/// Struct representing the Sharp LR35902 CPU found inside the original
/// DMG Gameboy hardware
pub struct LR35902 {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,
}

/// Methods associated to the LR25902 emulated CPU
impl LR35902 {
    /// Instantiates a new instance of an LR35902 CPU.
    /// All Registers are set to 0
    pub fn new() -> Self {
        return Self {
            af: Register { hi: 0x00, lo: 0x0 },
            bc: Register { hi: 0x00, lo: 0x0 },
            de: Register { hi: 0x00, lo: 0x0 },
            hl: Register { hi: 0x00, lo: 0x0 },
            sp: (0x0000),
            pc: (0x0000),
        };
    }

    /// Executes the next instruction pointed to by the PC register
    fn execute_next_op_code(&mut self) {}

    /// Implements all of the byte addressable, register to register, load operations that
    /// the Sharp LR25902 supports
    fn ld_8_reg_to_reg(&mut self, dest: RegisterID, src: RegisterID) -> Result<(), CpuError> {
        let src_value: u8;

        match src {
            RegisterID::A => src_value = self.af.hi,
            RegisterID::B => src_value = self.bc.hi,
            RegisterID::C => src_value = self.bc.lo,
            RegisterID::D => src_value = self.de.hi,
            RegisterID::E => src_value = self.de.lo,
            RegisterID::H => src_value = self.hl.hi,
            RegisterID::L => src_value = self.hl.lo,
            _ => return Err(CpuError::InvalidLoadOperands),
        }

        match dest {
            RegisterID::A => unsafe { write_byte_ptr(&mut self.af.hi, src_value) },
            RegisterID::B => unsafe { write_byte_ptr(&mut self.bc.hi, src_value) },
            RegisterID::C => unsafe { write_byte_ptr(&mut self.bc.lo, src_value) },
            RegisterID::D => unsafe { write_byte_ptr(&mut self.de.hi, src_value) },
            RegisterID::E => unsafe { write_byte_ptr(&mut self.de.lo, src_value) },
            RegisterID::H => unsafe { write_byte_ptr(&mut self.hl.hi, src_value) },
            RegisterID::L => unsafe { write_byte_ptr(&mut self.hl.lo, src_value) },
            _ => return Err(CpuError::InvalidLoadOperands),
        }

        return Ok(());
    }

    /// implements all of the immediate 8-bit load operations that the LR25902 supports
    fn ld_8_imm_to_reg(&mut self, dest: RegisterID, val: u8) -> Result<(), CpuError> {
        match dest {
            RegisterID::A => self.af.hi = val,
            RegisterID::B => self.bc.hi = val,
            RegisterID::C => self.bc.lo = val,
            RegisterID::D => self.de.hi = val,
            RegisterID::E => self.de.lo = val,
            RegisterID::H => self.hl.hi = val,
            RegisterID::L => self.hl.lo = val,
            _ => return Err(CpuError::InvalidLoadOperands),
        }

        return Ok(());
    }
}

/// Helper function to write a byte of data to a memory location.
/// This function executes in an unsafe scope
unsafe fn write_byte_ptr(ptr: *mut u8, val: u8) {
    *ptr = val;
}
