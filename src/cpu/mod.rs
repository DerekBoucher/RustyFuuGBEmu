//! Module containing all logic relevant to the emulation of the original
//! Gameboy's CPU (Sharp LR35902)
mod bit;
pub mod opcode;
mod opcode_ext;
mod register;

#[cfg(feature = "serial_debug")]
use crate::memory::io_registers;
use crate::{interrupt, memory};

use opcode::Opcode;
use register::{ID, ID16};
use std::{
    fmt::Debug,
    sync::{self, Arc},
};

/// Represents a byte addressable word register found
/// inside the Sharp LR35902
#[derive(Debug)]
struct Register {
    hi: u8,
    lo: u8,
}

/// Struct representing the Sharp LR35902 CPU found inside the original
/// DMG Gameboy hardware
#[derive(Debug)]
pub struct LR35902 {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,
    paused: bool,
    interrupt_master_enable: bool,
    halted: bool,
    bugged_halt: bool,
}

const INTERRUPT_ENABLE_REGISTER_ADDR: usize = 0xFFFF;
const INTERRUPT_FLAG_REGISTER_ADDR: usize = 0xFF0F;

/// Bit mask for the zero flag
const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
const CARRY_FLAG_MASK: u8 = 1 << 4;

pub const CPU_FREQUENCY: u32 = 4194304;
pub const CPU_CYCLES_PER_FRAME: u32 = CPU_FREQUENCY / 60;

const V_BLANK_INTERRUPT_VECTOR: u16 = 0x0040;
const LCDC_INTERRUPT_VECTOR: u16 = 0x0048;
const TIMER_OVERFLOW_INTERRUPT_VECTOR: u16 = 0x0050;
const SERIAL_IO_INTERRUPT_VECTOR: u16 = 0x0058;
const CONTROLLER_IO_INTERRUPT_VECTOR: u16 = 0x0060;

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

    pub fn is_halted(&self) -> bool {
        return self.halted;
    }

    pub fn handle_halt(&mut self, interrupt_bus: &Arc<sync::Mutex<interrupt::Bus>>) {
        if self.bugged_halt {
            self.bugged_halt = false;
            return;
        }

        if interrupt_bus
            .lock()
            .unwrap()
            .get_highest_priority_interrupt()
            .is_some()
        {
            self.halted = false;
        }
    }

    pub fn reset(&mut self) {
        *self = LR35902::new();
    }

    pub fn execute_next_opcode(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let op = match memory.lock().unwrap().read(usize::from(self.pc)) {
            Some(x) => Opcode::from(x),
            None => panic!(
                "memory returned empty value when attempting to fetch op code. Dumping cpu state...\n
                {:?}", self
            ),
        };

        self.pc = self.pc.wrapping_add(1);
        if self.bugged_halt {
            self.bugged_halt = false;
            self.pc = self.pc.wrapping_sub(1);
        }

        #[cfg(feature = "serial_debug")]
        LR35902::serial_debug_output(memory);

        return op.execute(self, memory, step_fn);
    }

    pub fn is_stopped(&self) -> bool {
        return self.paused;
    }

    pub fn set_post_boot_rom_state(&mut self) {
        self.af.set_word(0x01B0);
        self.bc.set_word(0x0013);
        self.de.set_word(0x00D8);
        self.hl.set_word(0x014D);
        self.pc = 0x0100;
        self.sp = 0xFFFE;
    }

    pub fn process_interrupts(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        interrupt_bus: &Arc<sync::Mutex<interrupt::Bus>>,
        step_fn: &mut impl FnMut(),
    ) {
        if !self.interrupt_master_enable {
            return;
        }

        let pending_interrupt = interrupt_bus
            .lock()
            .unwrap()
            .get_highest_priority_interrupt();

        match pending_interrupt {
            Some(interrupt) => {
                self.interrupt_master_enable = false;
                self.push_16bit_register_on_stack(ID16::PC, &memory, step_fn);

                match interrupt {
                    interrupt::Interrupt::VBlank => self.pc = V_BLANK_INTERRUPT_VECTOR,
                    interrupt::Interrupt::LcdStat => self.pc = LCDC_INTERRUPT_VECTOR,
                    interrupt::Interrupt::TimerOverflow => {
                        self.pc = TIMER_OVERFLOW_INTERRUPT_VECTOR
                    }
                    interrupt::Interrupt::Serial => self.pc = SERIAL_IO_INTERRUPT_VECTOR,
                    interrupt::Interrupt::Joypad => self.pc = CONTROLLER_IO_INTERRUPT_VECTOR,
                }

                interrupt_bus.lock().unwrap().clear_interrupt(interrupt);
            }
            None => {}
        }
    }

    fn reset_half_carry_flag(&mut self) {
        self.af.lo &= !HALF_CARRY_FLAG_MASK;
    }

    fn set_half_carry_flag(&mut self) {
        self.af.lo |= HALF_CARRY_FLAG_MASK;
    }

    fn reset_zero_flag(&mut self) {
        self.af.lo &= !ZERO_FLAG_MASK;
    }

    fn set_zero_flag(&mut self) {
        self.af.lo |= ZERO_FLAG_MASK;
    }

    fn reset_carry_flag(&mut self) {
        self.af.lo &= !CARRY_FLAG_MASK;
    }

    fn set_carry_flag(&mut self) {
        self.af.lo |= CARRY_FLAG_MASK;
    }

    fn reset_sub_flag(&mut self) {
        self.af.lo &= !SUB_FLAG_MASK;
    }

    fn set_sub_flag(&mut self) {
        self.af.lo |= SUB_FLAG_MASK;
    }

    fn test_half_carry_flag(&self) -> bool {
        return self.af.lo & (HALF_CARRY_FLAG_MASK) > 0;
    }

    fn test_carry_flag(&self) -> bool {
        return self.af.lo & (CARRY_FLAG_MASK) > 0;
    }

    fn test_sub_flag(&self) -> bool {
        return self.af.lo & (SUB_FLAG_MASK) > 0;
    }

    fn test_zero_flag(&self) -> bool {
        return self.af.lo & (ZERO_FLAG_MASK) > 0;
    }

    fn increment_8_bit_register(&mut self, reg_id: register::ID) {
        let current_reg_value = self.read_register(&reg_id);

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

        self.write_register(&reg_id, current_reg_value.wrapping_add(1));
    }

    fn decrement_8_bit_register(&mut self, reg_id: register::ID) {
        let current_reg_value = self.read_register(&reg_id);

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

        self.write_register(&reg_id, current_reg_value.wrapping_sub(1));
    }

    fn add_16_bit_registers(&mut self, target: register::ID16, src: register::ID16) {
        let target_value = self.read_16bit_register(&target);

        let src_value = self.read_16bit_register(&src);

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

        self.write_16bit_register(&target, new_word);
    }

    fn add_8_bit_registers(
        &mut self,
        target: register::ID,
        src: register::ID,
        with_carry_flag: bool,
    ) {
        let target_value = self.read_register(&target);

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let src_value = self.read_register(&src);

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

        self.write_register(&target, new_target_value);
    }

    fn compare_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = self.read_register(&target);

        let src_value = self.read_register(&src);

        if target_value == src_value {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.set_sub_flag();

        if bit::is_half_borrow(target_value, src_value, false) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if target_value < src_value {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }
    }

    fn compare_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("invalid memory address read in compare_8_bit_memory (addr: {}), dumping cpu state...\n{:?}", addr, self),
        };

        if target_value == byte {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.set_sub_flag();

        if bit::is_half_borrow(target_value, byte, false) {
            self.set_half_carry_flag();
        } else {
            self.reset_half_carry_flag();
        }

        if target_value < byte {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }
    }

    fn add_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        with_carry_flag: bool,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
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

        self.write_register(&target, new_target_value);
    }

    fn sub_8_bit_registers(
        &mut self,
        target: register::ID,
        src: register::ID,
        with_carry_flag: bool,
    ) {
        let target_value = self.read_register(&target);

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        let src_value = self.read_register(&src);

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

        self.write_register(&target, new_target_value);
    }

    fn sub_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        with_carry_flag: bool,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        let carry: u8 = match self.test_carry_flag() && with_carry_flag {
            true => 0x01,
            false => 0x00,
        };

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
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

        self.write_register(&target, new_target_value);
    }

    fn and_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = self.read_register(&target);

        let src_value = self.read_register(&src);

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.set_half_carry_flag();

        let new_target_value = target_value & src_value;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&target, new_target_value);
    }

    fn and_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
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

        self.write_register(&target, new_target_value);
    }

    fn xor_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = self.read_register(&target);

        let src_value = self.read_register(&src);

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.reset_half_carry_flag();

        let new_target_value = target_value ^ src_value;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&target, new_target_value);
    }

    fn xor_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!(
                "invalid 8 bit xor operation: couldn't access byte at addr {:?}",
                addr
            ),
        };

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.reset_half_carry_flag();

        let new_target_value = target_value ^ byte;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&target, new_target_value);
    }

    fn or_8_bit_registers(&mut self, target: register::ID, src: register::ID) {
        let target_value = self.read_register(&target);

        let src_value = self.read_register(&src);

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.reset_half_carry_flag();

        let new_target_value = target_value | src_value;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&target, new_target_value);
    }

    fn or_8_bit_memory(
        &mut self,
        target: register::ID,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) {
        let target_value = self.read_register(&target);

        step_fn();
        let byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!(
                "invalid 8 bit or operation: couldn't access byte at addr {:?}",
                addr
            ),
        };

        self.reset_sub_flag();
        self.reset_carry_flag();
        self.reset_half_carry_flag();

        let new_target_value = target_value | byte;

        if new_target_value == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&target, new_target_value);
    }

    fn pop_stack_into_16_bit_register(
        &mut self,
        reg_id: register::ID16,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        step_fn: &mut impl FnMut(),
    ) {
        step_fn();
        let lo_byte = match memory.lock().unwrap().read(usize::from(self.sp)) {
            Some(byte) => byte,
            None => panic!("error occured when loading return address from stack pointer"),
        };

        self.sp = self.sp.wrapping_add(1);

        step_fn();
        let hi_byte = match memory.lock().unwrap().read(usize::from(self.sp)) {
            Some(byte) => byte,
            None => panic!("error occured when loading return address from stack pointer"),
        };

        self.sp = self.sp.wrapping_add(1);

        match reg_id {
            ID16::BC => {
                self.bc.lo = lo_byte;
                self.bc.hi = hi_byte;
            }
            ID16::DE => {
                self.de.lo = lo_byte;
                self.de.hi = hi_byte;
            }
            ID16::HL => {
                self.hl.lo = lo_byte;
                self.hl.hi = hi_byte;
            }
            ID16::AF => {
                self.af.lo = 0xF0 & lo_byte;
                self.af.hi = hi_byte;
            }
            ID16::SP => panic!("not supported"),
            ID16::PC => {
                self.pc = u16::from(hi_byte) << 8 | u16::from(lo_byte);
            }
        }
    }

    fn push_16bit_register_on_stack(
        &mut self,
        reg_id: register::ID16,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        step_fn: &mut impl FnMut(),
    ) {
        step_fn();
        let bytes = match reg_id {
            ID16::BC => self.bc.word().to_be_bytes(),
            ID16::DE => self.de.word().to_be_bytes(),
            ID16::HL => self.hl.word().to_be_bytes(),
            ID16::AF => self.af.word().to_be_bytes(),
            ID16::PC => self.pc.to_be_bytes(),
            ID16::SP => panic!("not supported"),
        };

        let hi_byte = bytes[0];
        let lo_byte = bytes[1];

        step_fn();
        self.sp = self.sp.wrapping_sub(1);
        memory.lock().unwrap().write(usize::from(self.sp), hi_byte);

        step_fn();
        self.sp = self.sp.wrapping_sub(1);
        memory.lock().unwrap().write(usize::from(self.sp), lo_byte);
    }

    fn jump_to_imm_address(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        condition: bool,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let lo_byte = match memory.lock().unwrap().read(usize::from(self.pc)) {
            Some(byte) => byte,
            None => panic!("error occured when loading lo byte address for non-zero jump"),
        };

        self.pc = self.pc.wrapping_add(1);

        step_fn();
        let hi_byte = match memory.lock().unwrap().read(usize::from(self.pc)) {
            Some(byte) => byte,
            None => panic!("error occured when loading hi byte address for non-zero jump"),
        };

        self.pc = self.pc.wrapping_add(1);

        if condition {
            self.pc = (u16::from(hi_byte) << 8) | u16::from(lo_byte);
            step_fn();

            return 16;
        }

        return 12;
    }

    fn call_to_imm_address(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        condition: bool,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let lo_byte = match memory.lock().unwrap().read(usize::from(self.pc)) {
            Some(byte) => byte,
            None => panic!("error occured when loading lo byte address for non-zero jump"),
        };

        self.pc = self.pc.wrapping_add(1);

        step_fn();
        let hi_byte = match memory.lock().unwrap().read(usize::from(self.pc)) {
            Some(byte) => byte,
            None => panic!("error occured when loading hi byte address for non-zero jump"),
        };

        self.pc = self.pc.wrapping_add(1);

        if condition {
            self.push_16bit_register_on_stack(register::ID16::PC, &memory, step_fn);
            self.pc = (u16::from(hi_byte) << 8) | u16::from(lo_byte);
            return 24;
        }

        return 12;
    }

    fn return_from_call_conditional(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        condition: bool,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();

        if condition {
            step_fn();
            let lo_byte = match memory.lock().unwrap().read(usize::from(self.sp)) {
                Some(byte) => byte,
                None => panic!("error occured when loading return address from stack pointer"),
            };

            self.sp = self.sp.wrapping_add(1);

            step_fn();
            let hi_byte = match memory.lock().unwrap().read(usize::from(self.sp)) {
                Some(byte) => byte,
                None => panic!("error occured when loading return address from stack pointer"),
            };

            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(hi_byte) << 8) | u16::from(lo_byte);

            step_fn();

            return 20;
        }

        return 8;
    }

    fn return_from_call(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        self.pop_stack_into_16_bit_register(register::ID16::PC, memory, step_fn);

        return 16;
    }

    fn rotate_8bit_register_left(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        if (byte & (1 << 7)) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte.rotate_left(1);

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_half_carry_flag();
        self.reset_sub_flag();

        self.write_register(&reg_id, byte);

        return 4;
    }

    fn rotate_8bit_memory_left(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        if (byte & (1 << 7)) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte.rotate_left(1);

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        return 12;
    }

    fn rotate_8bit_register_right(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        if (byte & 1) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte.rotate_right(1);

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_half_carry_flag();
        self.reset_sub_flag();

        self.write_register(&reg_id, byte);

        return 4;
    }

    fn rotate_8bit_memory_right(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        if (byte & 1) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte.rotate_right(1);

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        return 12;
    }

    fn read_register(&self, reg_id: &register::ID) -> u8 {
        match reg_id {
            ID::A => self.af.hi,
            ID::_F => self.af.lo,
            ID::B => self.bc.hi,
            ID::C => self.bc.lo,
            ID::D => self.de.hi,
            ID::E => self.de.lo,
            ID::H => self.hl.hi,
            ID::L => self.hl.lo,
        }
    }

    fn write_register(&mut self, reg_id: &register::ID, value: u8) {
        match reg_id {
            ID::A => self.af.hi = value,
            ID::_F => self.af.lo = value,
            ID::B => self.bc.hi = value,
            ID::C => self.bc.lo = value,
            ID::D => self.de.hi = value,
            ID::E => self.de.lo = value,
            ID::H => self.hl.hi = value,
            ID::L => self.hl.lo = value,
        }
    }

    fn read_16bit_register(&self, reg_id: &register::ID16) -> u16 {
        match reg_id {
            ID16::AF => self.af.word(),
            ID16::BC => self.bc.word(),
            ID16::DE => self.de.word(),
            ID16::HL => self.hl.word(),
            ID16::SP => self.sp,
            ID16::PC => self.pc,
        }
    }

    fn write_16bit_register(&mut self, reg_id: &register::ID16, value: u16) {
        match reg_id {
            ID16::AF => self.af.set_word(value),
            ID16::BC => self.bc.set_word(value),
            ID16::DE => self.de.set_word(value),
            ID16::HL => self.hl.set_word(value),
            ID16::SP => self.sp = value,
            ID16::PC => self.pc = value,
        }
    }

    fn rotate_8bit_register_left_carry(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        let msb = byte & (1 << 7);

        let old_carry: u8 = match self.test_carry_flag() {
            true => 1,
            false => 0,
        };

        if (msb) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (byte << 1) | old_carry;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        self.write_register(&reg_id, byte);

        return 4;
    }

    fn rotate_8bit_memory_left_carry(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        let msb = byte & (1 << 7);

        let old_carry: u8 = match self.test_carry_flag() {
            true => 1,
            false => 0,
        };

        if (msb) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (byte << 1) | old_carry;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        return 12;
    }

    fn rotate_8bit_register_right_carry(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        let lsb = byte & 1;

        let old_carry: u8 = match self.test_carry_flag() {
            true => 1 << 7,
            false => 0,
        };

        if (lsb) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (byte >> 1) | old_carry;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_half_carry_flag();
        self.reset_sub_flag();

        self.write_register(&reg_id, byte);

        return 4;
    }

    fn rotate_8bit_memory_right_carry(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        let lsb = byte & (1);

        let old_carry: u8 = match self.test_carry_flag() {
            true => 1,
            false => 0,
        };

        if (lsb) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (old_carry << 7) | (byte >> 1);

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_half_carry_flag();
        self.reset_sub_flag();

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        return 12;
    }

    fn shift_left_8bit_register_into_carry(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        if (byte & (1 << 7)) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte << 1;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&reg_id, byte);

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        return 4;
    }

    fn shift_left_8bit_memory_into_carry(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        if (byte & (1 << 7)) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = byte << 1;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        return 12;
    }

    fn shift_right_8bit_register_into_carry(&mut self, reg_id: register::ID) -> u32 {
        let mut byte = self.read_register(&reg_id);

        let old_msb = byte & 0x80;

        if (byte & 0x01) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (byte >> 1) | old_msb;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.write_register(&reg_id, byte);

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        return 4;
    }

    fn shift_right_8bit_memory_into_carry(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let mut byte = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        let old_msb = byte & 0x80;

        if (byte & 0x01) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        byte = (byte >> 1) | old_msb;

        if byte == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        step_fn();
        memory.lock().unwrap().write(addr, byte);

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        return 12;
    }

    fn swap_8bit_register(&mut self, reg_id: register::ID) -> u32 {
        let current = self.read_register(&reg_id);
        let result = ((current & 0x0F) << 4) | ((current & 0xF0) >> 4);
        self.write_register(&reg_id, result);

        if result == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_carry_flag();
        self.reset_half_carry_flag();
        self.reset_sub_flag();

        return 4;
    }

    fn swap_8bit_memory(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let current = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        let result = ((current & 0x0F) << 4) | ((current & 0xF0) >> 4);

        step_fn();
        memory.lock().unwrap().write(addr, result);

        if result == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_carry_flag();
        self.reset_half_carry_flag();
        self.reset_sub_flag();

        return 12;
    }

    fn shift_right_8bit_register(&mut self, reg_id: register::ID) -> u32 {
        let current = self.read_register(&reg_id);

        if current & (1 << 0) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let result = current >> 1;

        if result == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        self.write_register(&reg_id, result);

        return 4;
    }

    fn shift_right_8bit_memory(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let current = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        if current & (1 << 0) > 0 {
            self.set_carry_flag();
        } else {
            self.reset_carry_flag();
        }

        let result = current >> 1;

        if result == 0x00 {
            self.set_zero_flag();
        } else {
            self.reset_zero_flag();
        }

        step_fn();
        memory.lock().unwrap().write(addr, result);

        self.reset_sub_flag();
        self.reset_half_carry_flag();

        return 12;
    }

    fn test_bit(&mut self, reg_id: register::ID, bit_position: u8) -> u32 {
        let current = self.read_register(&reg_id);

        if current & (1 << bit_position) > 0 {
            self.reset_zero_flag();
        } else {
            self.set_zero_flag();
        }

        self.reset_sub_flag();
        self.set_half_carry_flag();

        return 4;
    }

    fn test_bit_memory(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        bit_position: u8,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let current = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        step_fn();
        if current & (1 << bit_position) > 0 {
            self.reset_zero_flag();
        } else {
            self.set_zero_flag();
        }

        self.reset_sub_flag();
        self.set_half_carry_flag();

        return 12;
    }

    fn reset_bit(&mut self, reg_id: register::ID, bit_position: u8) -> u32 {
        let current = self.read_register(&reg_id);
        self.write_register(&reg_id, current & !(1 << bit_position));

        return 4;
    }

    fn reset_bit_memory(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        bit_position: u8,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let current = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        step_fn();
        memory
            .lock()
            .unwrap()
            .write(addr, current & !(1 << bit_position));

        return 12;
    }

    fn set_bit(&mut self, reg_id: register::ID, bit_position: u8) -> u32 {
        let current = self.read_register(&reg_id);
        self.write_register(&reg_id, current | (1 << bit_position));

        return 4;
    }

    fn set_bit_memory(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        addr: usize,
        bit_position: u8,
        step_fn: &mut impl FnMut(),
    ) -> u32 {
        step_fn();
        let current = match memory.lock().unwrap().read(addr) {
            Some(byte) => byte,
            None => panic!("TODO"),
        };

        step_fn();
        memory
            .lock()
            .unwrap()
            .write(addr, current | (1 << bit_position));

        return 12;
    }

    #[cfg(feature = "serial_debug")]
    fn serial_debug_output(memory: &Arc<sync::Mutex<memory::Memory>>) {
        use std::io::{self, Write};

        match memory
            .lock()
            .unwrap()
            .read(io_registers::SERIAL_TRANSFER_CONTROL_ADDR)
        {
            Some(byte) => {
                if byte == 0x81 {
                    match memory
                        .lock()
                        .unwrap()
                        .read(io_registers::SERIAL_TRANSFER_DATA_ADDR)
                    {
                        Some(byte) => {
                            print!("{}", byte as char);
                            io::stdout().flush().unwrap();
                            memory
                                .lock()
                                .unwrap()
                                .write(io_registers::SERIAL_TRANSFER_CONTROL_ADDR, 0x00);
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
}
