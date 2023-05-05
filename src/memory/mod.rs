#![allow(dead_code)]
#![allow(unused_variables)]

//! Module containing all logic relevant to the emulation of the
//! original Gameboy's random access and read only memory
mod cartridge;
mod mbc1;
mod memory;
mod no_mbc;

use std::{any::Any, fmt::Debug};

/// Struct emulating the DMG Gameboy's memory behaviour.
/// This struct controls the access behaviour whenever the CPU
/// makes reads or writes to the memory.
#[derive(Debug)]
pub struct Memory {
    /// Cartridge data.
    /// Mapped into memory locations 0x0000 - 0x7FFF.
    cartridge: Box<dyn Cartridge>,

    /// Video RAM where tile data is located.
    /// Occupies memory locations 0x8000 ~ 0x9FFF.
    video_ram: [u8; 0x2000],

    /// General purpose RAM bank 0.
    /// Occupies memory locations 0xC000 ~ 0xCFFF.
    work_ram0: [u8; 0x1000],

    /// General purpose RAM bank 1.
    /// Occupies memory locations 0xD000 ~ 0xDFFF.
    work_ram1: [u8; 0x1000],

    /// Exact replica of contents of memory from 0xC000 ~ 0xDDFF.
    /// Not quite sure what the use of this is.
    /// Occupies memory locations 0xE000 ~ 0xFDFF.
    echo_ram: [u8; 0x1E00],

    /// Sprite data.
    /// Occupies memory locations 0xFE00 ~ 0xFE9F.
    sprite_attributes: [u8; 0xA0],

    /// IO Registers such as interupts, controls, etc...
    /// Occupies memory locations 0xFF00 ~ 0xFF7F.
    io_registers: [u8; 0x80],

    /// High RAM used by the CPU.
    /// Occupies memory locations 0xFF80 ~ 0xFFFE.
    hi_ram: [u8; 0x7F],

    /// Master interrupt enable register.
    /// Occupies a single byte of memory at location 0xFFFF.
    interrupt_enable_register: u8,
}

/// Cartridge trait which serves as an interface to the various
/// types of memory bank controllers that Gameboy cartridges
/// can contain.
pub trait Cartridge: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
}
