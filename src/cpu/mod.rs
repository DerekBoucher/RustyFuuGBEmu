#![allow(dead_code)]
#![allow(unused_variables)]

//! Module containing all logic relevant to the emulation of the original
//! Gameboy's CPU (Sharp LR35902)
mod bit;
mod lr35902;
mod opcode;
mod register;

use std::fmt::Debug;

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

    paused: bool,
}

pub trait MemoryDriver: Debug {
    fn read(&self, addr: usize) -> Option<&u8>;
    fn write(&mut self, addr: usize, val: u8);
}
