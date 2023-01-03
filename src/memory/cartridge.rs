#[path = "cartridge_test.rs"]
#[cfg(test)]
mod cartridge_test;
use std::any::Any;

use crate::memory::Cartridge;

/// Module containing important addresses in the cartridge
/// header.
mod header {
    /// Location in the cartridge header where it
    /// declares the type of memory bank controller it has.
    pub const TYPE_ADDR: usize = 0x147;

    /// Location in the cartridge header where the number of
    /// ROM banks is declared.
    pub const ROM_SIZE_ADDR: usize = 0x148;

    /// Location in the cartridge header where the number of
    /// RAM banks is declared.
    pub const RAM_SIZE_ADDR: usize = 0x149;
}

mod mbc_id {
    pub const ROM_ONLY: u8 = 0x00;
    pub const MBC1: u8 = 0x01;
    pub const MBC1_RAM: u8 = 0x02;
}

/// Cartridge constructor which returns the appropriate
/// cartridge implementor at runtime.
pub fn new(data: Vec<u8>) -> Box<dyn Cartridge> {
    match data[header::TYPE_ADDR] {
        mbc_id::ROM_ONLY => return Box::new(RomOnly { data }),
        _ => panic!("unsupported cartridge type!"),
    }
}

/// Rom only type of cartridge has no memory bank
/// controller. Simplest form of the gameboy cart.
struct RomOnly {
    data: Vec<u8>,
}

/// Cartridge implementation for the Rom Only type.
impl Cartridge for RomOnly {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Read simply returns the data at the specified address.
    fn read(&self, addr: usize) -> Option<&u8> {
        return self.data.get(addr);
    }

    /// Write is a NOP for the RomOnly cartridge type
    fn write(&mut self, _addr: usize, _val: u8) {}
}
