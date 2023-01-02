#![allow(dead_code)]
#![allow(unused_variables)]

//! Module containing all logic relevant to the emulation of the original
//! Gameboy's CPU (Sharp LR35902)
use crate::memory::Memory;

mod cpu_impl;
mod operation_impl;
mod register_impl;

/// Represents a byte addressable word register found
/// inside the Sharp LR35902
#[derive(Debug, PartialEq)]
struct Register {
    hi: u8,
    lo: u8,
}

/// Struct representing the Sharp LR35902 CPU found inside the original
/// DMG Gameboy hardware
#[derive(Debug, PartialEq)]
pub struct LR35902 {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,

    memory: Memory,
}

/// Operation trait to be implemented by all of the op codes.
/// In a nutshell, the LR35902 struct is blind to each op code's
/// modifications, and lets them perform the neccessary changes on the CPU's
/// behalf
trait Operation {
    fn op_code() -> u8;
    fn execute(cpu: &mut LR35902) -> u32;
}

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

/// Enumeration of all possible errors that the LR35902 methods can run into
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Occurs when invalid operands are provided to a load method
    InvalidLoadOperands,
}
