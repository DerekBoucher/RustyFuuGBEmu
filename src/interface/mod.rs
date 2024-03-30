use core::fmt::Debug;
use core::marker::Send;
use std::any::Any;

#[cfg(test)]
pub mod mock;

/// Memory trait which serves as an interface to the various implementations
/// of the gameboy's memory.
pub trait Memory: Debug + Send {
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
    fn dump(&self) -> Vec<u8>;
    fn set_post_boot_rom_state(&mut self);
    fn update_dma_transfer_cycles(&mut self, cycles: u32);
}

/// Timer trait which serves as an interface to the various timer implementations
pub trait Timers: Debug + Send {
    fn update(&mut self, cycles: u32, memory: &mut impl Memory, cpu: &mut impl CPU);
}

/// Cartridge trait which serves as an interface to the various
/// types of memory bank controllers that Gameboy cartridges
/// can contain.
pub trait Cartridge: Any + Debug + Send {
    fn as_any(&self) -> &dyn Any;
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
}

/// PPU (a.k.a. Pixel Processing Unit) trait which serves as an interface for
/// the implementations that prepare pixel data when a new frame is to be drawn.
pub trait PPU: Debug + Send {
    fn update_graphics(&mut self, cycles: u32, memory: &mut impl Memory, cpu: &mut impl CPU);
    fn get_frame_data(&self) -> [[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT];
}

/// CPU trait that lets implementors of the LR35902 Sharp processing unit expose the necessary API
pub trait CPU: Send {
    fn execute_next_opcode(&mut self, memory: &mut impl Memory) -> u32;
    fn set_post_boot_rom_state(&mut self);
    fn process_interrupts(&mut self, memory: &mut impl Memory, timers: &mut impl Timers);
    fn request_interrupt(&mut self, memory: &mut impl Memory, interrupt: Interrupt);
    fn is_halted(&self) -> bool;
    fn halt(&mut self, memory: &mut impl Memory);
}

pub enum Interrupt {
    VBlank,
    LCDC,
    TimerOverflow,
    _Serial,
    _Joypad,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Pixel {
    pub fn to_rgb(&self) -> (f32, f32, f32) {
        match self {
            Self::White => (1.0, 1.0, 1.0),
            Self::Black => (0.0, 0.0, 0.0),
            Self::LightGray => (0.75, 0.75, 0.75),
            Self::DarkGray => (0.83, 0.83, 0.83),
        }
    }
}

pub const NATIVE_SCREEN_WIDTH: usize = 160;
pub const NATIVE_SCREEN_HEIGHT: usize = 144;
