#[path = "lr35902_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::bit;
use crate::cpu::opcode::*;
use crate::cpu::register;
use crate::cpu::register::*;
use crate::cpu::Register;
use crate::cpu::LR35902;
use crate::memory;

pub const INTERRUPT_ENABLE_REGISTER_ADDR: usize = 0xFFFF;
pub const INTERRUPT_FLAG_REGISTER_ADDR: usize = 0xFF0F;

/// Bit mask for the zero flag
pub const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
pub const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
pub const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
pub const CARRY_FLAG_MASK: u8 = 1 << 4;

impl PartialEq for LR35902 {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
            && self.sp == other.sp
            && self.af.word() == other.af.word()
            && self.bc.word() == other.bc.word()
            && self.de.word() == other.de.word()
            && self.hl.word() == other.hl.word()
            && self.paused == other.paused
            && self.interrupt_master_enable == other.interrupt_master_enable
            && self.halted == other.halted
            && self.bugged_halt == other.bugged_halt
    }
}

impl LR35902 {
    pub fn new() -> Self {
        Self {
            af: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0x0000,
            pc: 0x0000,
            paused: false,
            interrupt_master_enable: false,
            halted: false,
            bugged_halt: false,
        }
    }

    pub fn execute_next_opcode(&mut self, memory: &mut impl memory::Interface) -> u32 {
        let op = match memory.read(usize::from(self.pc)) {
            Some(x) => Opcode::from(x),
            None => panic!(
                "memory returned empty value when attempting to fetch op code. Dumping cpu state...\n
                {:?}", self
            ),
        };

        return op.execute(self, memory);
    }

    pub fn reset_half_carry_flag(&mut self) {
        self.af.lo &= !HALF_CARRY_FLAG_MASK;
    }

    pub fn set_half_carry_flag(&mut self) {
        self.af.lo |= HALF_CARRY_FLAG_MASK;
    }

    pub fn reset_zero_flag(&mut self) {
        self.af.lo &= !ZERO_FLAG_MASK;
    }

    pub fn set_zero_flag(&mut self) {
        self.af.lo |= ZERO_FLAG_MASK;
    }

    pub fn reset_carry_flag(&mut self) {
        self.af.lo &= !CARRY_FLAG_MASK;
    }

    pub fn set_carry_flag(&mut self) {
        self.af.lo |= CARRY_FLAG_MASK;
    }

    pub fn reset_sub_flag(&mut self) {
        self.af.lo &= !SUB_FLAG_MASK;
    }

    pub fn set_sub_flag(&mut self) {
        self.af.lo |= SUB_FLAG_MASK;
    }

    pub fn test_half_carry_flag(&self) -> bool {
        return self.af.lo & (HALF_CARRY_FLAG_MASK) > 0;
    }

    pub fn test_carry_flag(&self) -> bool {
        return self.af.lo & (CARRY_FLAG_MASK) > 0;
    }

    pub fn test_sub_flag(&self) -> bool {
        return self.af.lo & (SUB_FLAG_MASK) > 0;
    }

    pub fn test_zero_flag(&self) -> bool {
        return self.af.lo & (ZERO_FLAG_MASK) > 0;
    }

    pub fn increment_8_bit_register(&mut self, reg_id: register::ID) {
        let current_reg_value = match reg_id {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!(
                "unrecognized 8 bit register identifier for 8 bit increment: {:?}",
                reg_id
            ),
        };

        self.reset_sub_flag();

        if current_reg_value.wrapping_add(1) == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        if bit::is_half_carry(current_reg_value, 1, false) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        match reg_id {
            ID::A => self.af.hi = self.af.hi.wrapping_add(1),
            ID::B => self.bc.hi = self.bc.hi.wrapping_add(1),
            ID::C => self.bc.lo = self.bc.lo.wrapping_add(1),
            ID::D => self.de.hi = self.de.hi.wrapping_add(1),
            ID::E => self.de.lo = self.de.lo.wrapping_add(1),
            ID::H => self.hl.hi = self.hl.hi.wrapping_add(1),
            ID::L => self.hl.lo = self.hl.lo.wrapping_add(1),
            _ => panic!(
                "unrecognized 8 bit register identifier for 8 bit increment: {:?}",
                reg_id
            ),
        }
    }

    pub fn decrement_8_bit_register(&mut self, reg_id: register::ID) {
        let current_reg_value = match reg_id {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!(
                "unrecognized 8 bit register identifier for 8 bit decrement: {:?}",
                reg_id
            ),
        };

        self.set_sub_flag();

        if current_reg_value.wrapping_sub(1) == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        if bit::is_half_borrow(current_reg_value, 1, false) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        match reg_id {
            ID::A => self.af.hi = self.af.hi.wrapping_sub(1),
            ID::B => self.bc.hi = self.bc.hi.wrapping_sub(1),
            ID::C => self.bc.lo = self.bc.lo.wrapping_sub(1),
            ID::D => self.de.hi = self.de.hi.wrapping_sub(1),
            ID::E => self.de.lo = self.de.lo.wrapping_sub(1),
            ID::H => self.hl.hi = self.hl.hi.wrapping_sub(1),
            ID::L => self.hl.lo = self.hl.lo.wrapping_sub(1),
            _ => panic!(
                "unrecognized 8 bit register identifier for 8 bit decrement: {:?}",
                reg_id
            ),
        }
    }

    pub fn add_16_bit_registers(&mut self, target: register::ID16, src: register::ID16) {
        let target_value = match target {
            ID16::BC => self.bc.word(),
            ID16::DE => self.de.word(),
            ID16::HL => self.hl.word(),
            ID16::SP => self.sp,
            _ => panic!("invalid 16 bit add operation: targetID {:?}", target),
        };

        let src_value = match src {
            ID16::BC => self.bc.word(),
            ID16::DE => self.de.word(),
            ID16::HL => self.hl.word(),
            ID16::SP => self.sp,
            _ => panic!("invalid 16 bit add operation: srcID {:?}", src),
        };

        self.reset_sub_flag();

        if bit::is_half_carry_word(target_value, src_value, 0x0FFF, false) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if target_value > (0xFFFF - src_value) {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let new_word = target_value.wrapping_add(src_value);

        match target {
            ID16::BC => self.bc.set_word(new_word),
            ID16::DE => self.de.set_word(new_word),
            ID16::HL => self.hl.set_word(new_word),
            ID16::SP => self.sp = new_word,
            _ => panic!("invalid 16 bit add operation: targetID {:?}", target),
        }
    }

    pub fn add_8_bit_registers(
        &mut self,
        target: register::ID,
        src: register::ID,
        with_carry_flag: bool,
    ) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit add operation: targetID {:?}", target),
        };

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let src_value = match src {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!("invalid 8 bit add operation: srcID {:?}", src),
        };

        if bit::is_half_carry(target_value, src_value, carry > 0x00) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if bit::is_carry(target_value, src_value, carry > 0x00) {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let new_target_value = target_value.wrapping_add(src_value).wrapping_add(carry);

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit add operation: targetID {:?}", target),
        };
    }

    pub fn add_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &impl memory::Interface,
        addr: usize,
        with_carry_flag: bool,
    ) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit add operation: targetID {:?}", target),
        };

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let byte = match memory.read(addr) {
            Some(byte) => byte,
            None => panic!("invalid memory address read in add_8_bit_memory (addr: {}), dumping cpu state...\n{:?}", addr, self),
        };

        if bit::is_half_carry(target_value, byte, carry > 0x00) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if bit::is_carry(target_value, byte, carry > 0x00) {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let new_target_value = target_value.wrapping_add(byte).wrapping_add(carry);

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit add operation: targetID {:?}", target),
        };
    }

    pub fn sub_8_bit_registers(
        &mut self,
        target: register::ID,
        src: register::ID,
        with_carry_flag: bool,
    ) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit sub operation: targetID {:?}", target),
        };

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let src_value = match src {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!("invalid 8 bit sub operation: srcID {:?}", src),
        };

        if bit::is_half_borrow(target_value, src_value, carry > 0x00) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if bit::is_borrow(target_value, src_value, carry > 0x00) {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let new_target_value = target_value.wrapping_sub(src_value).wrapping_sub(carry);

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.set_sub_flag();

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit sub operation: targetID {:?}", target),
        };
    }

    pub fn sub_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &impl memory::Interface,
        addr: usize,
        with_carry_flag: bool,
    ) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit sub operation: targetID {:?}", target),
        };

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let byte = match memory.read(addr) {
            Some(byte) => byte,
            None => panic!("invalid memory address read in sub_8_bit_memory (addr: {}), dumping cpu state...\n{:?}", addr, self),
        };

        if bit::is_half_borrow(target_value, byte, carry > 0x00) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if bit::is_borrow(target_value, byte, carry > 0x00) {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let new_target_value = target_value.wrapping_sub(byte).wrapping_sub(carry);

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.set_sub_flag();

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit sub operation: targetID {:?}", target),
        };
    }

    pub fn and_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit and operation: targetID {:?}", target),
        };

        let src_value = match src {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!("invalid 8 bit and operation: srcID {:?}", src),
        };

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.set_half_carry_flag();

        let new_target_value = target_value & src_value;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit and operation: targetID {:?}", target),
        };
    }

    pub fn and_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &impl memory::Interface,
        addr: usize,
    ) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit and operation: targetID {:?}", target),
        };

        let byte = match memory.read(addr) {
            Some(byte) => byte,
            None => panic!(
                "invalid 8 bit and operation: couldn't access byte at addr {:?}",
                addr
            ),
        };

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.set_half_carry_flag();

        let new_target_value = target_value & byte;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit and operation: targetID {:?}", target),
        };
    }

    pub fn xor_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = match target {
            ID::A => self.af.hi,
            _ => panic!("invalid 8 bit xor operation: targetID {:?}", target),
        };

        let src_value = match src {
            ID::A => self.af.hi,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
            _ => panic!("invalid 8 bit xor operation: srcID {:?}", src),
        };

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.reset_half_carry_flag();

        let new_target_value = target_value ^ src_value;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        match target {
            ID::A => self.af.hi = new_target_value,
            _ => panic!("invalid 8 bit xor operation: targetID {:?}", target),
        };
    }
}
