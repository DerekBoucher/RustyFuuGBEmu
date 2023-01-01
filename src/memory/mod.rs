//! Module containing all logic relevant to the emulation of the
//! original Gameboy's random access and read only memory
mod memory_impl;

pub struct Memory {
    rom: [u8; 0x10000],
}
