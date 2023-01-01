use crate::cpu::Error;
use crate::cpu::Register;
use crate::cpu::RegisterID;
use crate::cpu::LR35902;
use crate::memory::Memory;

#[path = "cpu_impl_test.rs"]
#[cfg(test)]
mod tests;

/// Bit mask for the zero flag
const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
const CARRY_FLAG_MASK: u8 = 1 << 4;

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
            memory: Memory::new(),
        };
    }

    /// Implements all of the byte addressable, register to register, load operations that
    /// the Sharp LR25902 supports
    fn ld_8_reg_to_reg(&mut self, dest: RegisterID, src: RegisterID) -> Result<(), Error> {
        let src_value: u8;

        match src {
            RegisterID::A => src_value = self.af.hi,
            RegisterID::B => src_value = self.bc.hi,
            RegisterID::C => src_value = self.bc.lo,
            RegisterID::D => src_value = self.de.hi,
            RegisterID::E => src_value = self.de.lo,
            RegisterID::H => src_value = self.hl.hi,
            RegisterID::L => src_value = self.hl.lo,
            _ => return Err(Error::InvalidLoadOperands),
        }

        match dest {
            RegisterID::A => unsafe { write_byte_ptr(&mut self.af.hi, src_value) },
            RegisterID::B => unsafe { write_byte_ptr(&mut self.bc.hi, src_value) },
            RegisterID::C => unsafe { write_byte_ptr(&mut self.bc.lo, src_value) },
            RegisterID::D => unsafe { write_byte_ptr(&mut self.de.hi, src_value) },
            RegisterID::E => unsafe { write_byte_ptr(&mut self.de.lo, src_value) },
            RegisterID::H => unsafe { write_byte_ptr(&mut self.hl.hi, src_value) },
            RegisterID::L => unsafe { write_byte_ptr(&mut self.hl.lo, src_value) },
            _ => return Err(Error::InvalidLoadOperands),
        }

        return Ok(());
    }

    /// implements all of the immediate 8-bit load operations that the LR25902 supports
    fn ld_8_imm_to_reg(&mut self, dest: RegisterID, val: u8) -> Result<(), Error> {
        match dest {
            RegisterID::A => self.af.hi = val,
            RegisterID::B => self.bc.hi = val,
            RegisterID::C => self.bc.lo = val,
            RegisterID::D => self.de.hi = val,
            RegisterID::E => self.de.lo = val,
            RegisterID::H => self.hl.hi = val,
            RegisterID::L => self.hl.lo = val,
            _ => return Err(Error::InvalidLoadOperands),
        }

        return Ok(());
    }

    /// Load a 8-bit register value into memory location pointed to by the HL word register
    fn ld_reg_into_mem_hl(&mut self, src: RegisterID) -> Result<(), Error> {
        let value: u8;

        match src {
            RegisterID::A => value = self.af.hi,
            RegisterID::B => value = self.bc.hi,
            RegisterID::C => value = self.bc.lo,
            RegisterID::D => value = self.de.hi,
            RegisterID::E => value = self.de.lo,
            RegisterID::H => value = self.hl.hi,
            RegisterID::L => value = self.hl.lo,
            _ => return Err(Error::InvalidLoadOperands),
        }

        Ok(self.memory.write(self.hl.word().into(), value))
    }

    /// Load a byte pointed to by the HL word register into a destination register
    fn ld_mem_hl_into_reg(&mut self, dest: RegisterID) -> Result<(), Error> {
        match dest {
            RegisterID::A => self.af.hi = self.memory.read(self.hl.word().into()),
            RegisterID::B => self.bc.hi = self.memory.read(self.hl.word().into()),
            RegisterID::C => self.bc.lo = self.memory.read(self.hl.word().into()),
            RegisterID::D => self.de.hi = self.memory.read(self.hl.word().into()),
            RegisterID::E => self.de.lo = self.memory.read(self.hl.word().into()),
            RegisterID::H => self.hl.hi = self.memory.read(self.hl.word().into()),
            RegisterID::L => self.hl.lo = self.memory.read(self.hl.word().into()),
            _ => return Err(Error::InvalidLoadOperands),
        }

        Ok(())
    }
}

/// Helper function to write a byte of data to a memory location.
/// This function executes in an unsafe scope
unsafe fn write_byte_ptr(ptr: *mut u8, val: u8) {
    *ptr = val;
}
