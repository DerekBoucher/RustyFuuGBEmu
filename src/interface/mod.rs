use core::fmt::Debug;
use core::marker::Send;
use std::any::Any;

#[cfg(test)]
pub mod mock;

/// Memory trait which serves as an interface to the various implementations
/// of the gameboy's memory.
pub trait Memory: Debug + Send {
    fn reset(&mut self, cartridge: Box<dyn Cartridge>);
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
    fn dump(&self) -> Vec<u8>;
    fn update_timers(&mut self, cycles: u32);
    fn set_post_boot_rom_state(&mut self);
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
    fn reset(&mut self);
    fn step(&mut self, cycles: u32, memory: &mut impl Memory);
    fn set_lcdc_status(&mut self, memory: &mut impl Memory);
    fn render_tiles(&mut self, memory: &impl Memory);
}

/// CPU trait that lets implementors of the LR35902 Sharp processing unit expose the necessary API
pub trait CPU: Debug + Send {
    fn reset(&mut self);
    fn execute_next_opcode(&mut self, memory: &mut impl Memory) -> u32;
    fn set_post_boot_rom_state(&mut self);
}
