mod mbc1;
mod no_mbc;
mod null;

use std::any::Any;
use std::fmt::Debug;

use crate::cartridge::mbc1::MBC1;
use crate::cartridge::no_mbc::NoMBC;

/// Cartridge trait which serves as an interface to the various
/// types of memory bank controllers that Gameboy cartridges
/// can contain.
pub trait Interface: Any + Debug + Send {
    //fn as_any(&self) -> &dyn Any;
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
}

/// Module containing important addresses in the cartridge
/// header.
pub mod header {
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

pub mod rom_size_id {
    /// Byte identifier for cartridges that are 1MiB in size.
    /// Any value greater than this ID also means ROM sizes > 1MiB.
    pub const ONE_MEGABYTE: u8 = 0x05;
}

mod ram_size_id {
    pub const NO_RAM: u8 = 0x00;
    pub const _ONE_BANK: u8 = 0x02;
    pub const FOUR_BANKS: u8 = 0x03;
    pub const _HEX_BANKS: u8 = 0x04;
    pub const _OCTA_BANKS: u8 = 0x05;
}

mod mbc_id {
    pub const ROM_ONLY: u8 = 0x00;
    pub const MBC1: u8 = 0x01;
    pub const MBC1_RAM: u8 = 0x02;
    pub const MBC1_RAM_BATTERY: u8 = 0x03;
}

/// Cartridge constructor which returns the appropriate
/// cartridge implementation at runtime, depending on the value of the
/// ROM at memory location 0x147.
pub fn new(data: Vec<u8>) -> Box<dyn Interface> {
    match data[header::TYPE_ADDR] {
        mbc_id::ROM_ONLY => {
            log::debug!("Cartridge type: ROM_ONLY");
            return NoMBC::new(data);
        }
        mbc_id::MBC1 | mbc_id::MBC1_RAM | mbc_id::MBC1_RAM_BATTERY => {
            log::debug!("Cartridge type: MBC1");
            return MBC1::new(data);
        }
        _ => {
            panic!("Unsupported cartridge type");
        }
    }
}

pub fn default() -> Box<dyn Interface> {
    return Box::new(null::NullCartridge {});
}
