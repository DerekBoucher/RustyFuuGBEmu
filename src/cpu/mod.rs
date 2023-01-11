#![allow(dead_code)]
#![allow(unused_variables)]

//! Module containing all logic relevant to the emulation of the original
//! Gameboy's CPU (Sharp LR35902)
mod lr35902;
mod opcode;
mod register;

use std::fmt::Debug;

/// Bit mask for the zero flag
const ZERO_FLAG_MASK: u8 = 1 << 7;

/// Bit mask for the sub flag
const SUB_FLAG_MASK: u8 = 1 << 6;

/// Bit mask for the half carry flag
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;

/// Bit mask for the carry flag
const CARRY_FLAG_MASK: u8 = 1 << 4;

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

    memory: Box<dyn MemoryDriver>,
}

pub trait MemoryDriver: Debug {
    fn read(&self, addr: usize) -> Option<&u8>;
    fn write(&mut self, addr: usize, val: u8);
}
