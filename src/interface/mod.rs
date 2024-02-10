use core::fmt::Debug;

use core::marker::Send;
use std::any::Any;

#[cfg(test)]
pub mod mock;

/// Memory trait which serves as an interface to the various implementations
/// of the gameboy's memory.
pub trait Memory: Debug {
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
    fn dump(&self) -> Vec<u8>;
    fn update_timers(&mut self, cycles: u32);
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
pub trait PPU: Debug {
    fn step(&mut self, cycles: u32, memory: &mut impl Memory);
    fn set_lcdc_status(&mut self, memory: &mut impl Memory);
    fn render_tiles(&mut self, memory: &impl Memory);
}
