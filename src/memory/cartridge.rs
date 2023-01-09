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

mod rom_size_id {
    /// Byte identifier for cartridges that are 1MiB in size.
    /// Any value greater than this ID also means ROM sizes > 1MiB.
    pub const ONE_MEGABYTE: u8 = 0x05;

    pub fn get_bank_mask(bank_id: u8) -> u8 {
        match bank_id {
            0x00 => return 0x1,
            0x01 => return 0x3,
            0x02 => return 0x7,
            0x03 => return 0xF,
            0x04 => return 0x1F,
            0x05 => return 0x3F,
            0x06 => return 0x7F,
            0x07 => return 0xFF,
            _ => return 0xFF,
        }
    }
}

mod ram_size_id {
    pub const NO_RAM: u8 = 0x00;
    pub const ONE_BANK: u8 = 0x02;
    pub const FOUR_BANKS: u8 = 0x03;
    pub const HEX_BANKS: u8 = 0x04;
    pub const OCTA_BANKS: u8 = 0x05;
}

mod mbc_id {
    pub const ROM_ONLY: u8 = 0x00;
    pub const MBC1: u8 = 0x01;
    pub const MBC1_RAM: u8 = 0x02;
    pub const MBC1_RAM_BATTERY: u8 = 0x03;
}

/// Cartridge constructor which returns the appropriate
/// cartridge implementor at runtime.
pub fn new(data: Vec<u8>) -> Box<dyn Cartridge> {
    match data[header::TYPE_ADDR] {
        mbc_id::ROM_ONLY => return RomOnly::new(data),
        mbc_id::MBC1 | mbc_id::MBC1_RAM | mbc_id::MBC1_RAM_BATTERY => return MBC1::new(data),
        _ => panic!("unsupported cartridge type!"),
    }
}

/// Rom only type of cartridge has no memory bank
/// controller. Simplest form of the gameboy cart.
struct RomOnly {
    data: Vec<u8>,
    ram_bank: [u8; 0x2000],
}

/// Cartridge implementation for the Rom Only type.
impl Cartridge for RomOnly {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn read(&self, addr: usize) -> Option<&u8> {
        if addr < 0x8000 {
            return self.data.get(addr);
        }

        // Ram bank
        if addr >= 0xA000 && addr < 0xC000 {
            return Some(&self.ram_bank[addr - 0xA000]);
        }

        None
    }

    fn write(&mut self, addr: usize, val: u8) {
        if addr >= 0xA000 && addr < 0xC000 {
            self.ram_bank[addr - 0xA000] = val;
        }
    }
}

/// Constructor for RomOnly
impl RomOnly {
    fn new(data: Vec<u8>) -> Box<RomOnly> {
        Box::new(RomOnly {
            data,
            ram_bank: [0x0; 0x2000],
        })
    }
}

/// MBC1 type of cartridge has a memory bank controller
/// which swaps out the exposed memory that the cpu sees
/// in memory locations 0x4000 ~ 0x7FFF.
struct MBC1 {
    rom: Vec<u8>,
    ram_enabled: bool,
    rom_bank_select_register: u8,
    ram_bank_select_register: u8,
    banking_mode: bool,
    ram_banks: [[u8; 0x2000]; 4],
}

/// Constructor for MBC1
impl MBC1 {
    pub const SIMPLE_BANKING_MODE: bool = false;
    pub const RAM_BANKING_MODE: bool = true;

    fn new(data: Vec<u8>) -> Box<Self> {
        Box::new(MBC1 {
            rom: data,
            ram_enabled: false,
            rom_bank_select_register: 0x00,
            ram_bank_select_register: 0x00,
            banking_mode: false,
            ram_banks: [[0x00; 0x2000]; 4],
        })
    }

    fn supports_ram(&self) -> bool {
        if self.rom[header::RAM_SIZE_ADDR] == ram_size_id::NO_RAM {
            return false;
        }

        return self.rom[header::TYPE_ADDR] == mbc_id::MBC1_RAM
            || self.rom[header::TYPE_ADDR] == mbc_id::MBC1_RAM_BATTERY;
    }
}

impl Cartridge for MBC1 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn read(&self, addr: usize) -> Option<&u8> {
        if addr < 0x4000 {
            // If the ROM size is greater than 1MiB, then the cartridge
            // might be using the RAM select register's 2-bits as an extension
            // to bank this area of memory as well. That is, only if banking mode
            // is set to RAM_BANKING_MODE.
            if self.rom[header::ROM_SIZE_ADDR] >= rom_size_id::ONE_MEGABYTE
                && self.banking_mode == MBC1::RAM_BANKING_MODE
            {
                // TODO: MBC1M Multicart implementation: https://gbdev.io/pandocs/MBC1.html#00003fff--rom-bank-x0-read-only
                let bank_number: usize = usize::from((self.ram_bank_select_register & 0x3) << 5);
                let translated_addr: usize = (0x4000 * bank_number) + addr;
                return Some(&self.rom[translated_addr]);
            }

            return Some(&self.rom[addr]);
        }

        // ROM Banks 0x01 - 0x7F. See https://gbdev.io/pandocs/MBC1.html#40007fff--rom-bank-01-7f-read-only for more details.
        if addr >= 0x4000 && addr < 0x8000 {
            let mut translated_addr = addr - 0x4000;
            let mut bank_number: usize = usize::from(self.rom_bank_select_register);
            if bank_number == 0x0 {
                bank_number += 1;
            }

            if self.banking_mode == MBC1::RAM_BANKING_MODE
                && self.rom[header::ROM_SIZE_ADDR] >= rom_size_id::ONE_MEGABYTE
            {
                bank_number |= usize::from(self.ram_bank_select_register << 5);
            }

            translated_addr += usize::from(bank_number * 0x4000);
            return Some(&self.rom[translated_addr]);
        }

        // RAM banks
        if addr >= 0xA000 && addr < 0xC000 {
            if !self.supports_ram() || !self.ram_enabled {
                return Some(&0xFF);
            }

            let translated_addr: usize = addr - 0xA000;
            let mut bank_number: usize = 0x0;

            if self.rom[header::RAM_SIZE_ADDR] >= ram_size_id::FOUR_BANKS {
                bank_number = usize::from(self.ram_bank_select_register & 0b00000011);
            }

            return Some(&self.ram_banks[bank_number][translated_addr]);
        }

        None
    }

    fn write(&mut self, addr: usize, val: u8) {
        if addr < 0x2000 {
            if val & 0x0F == 0x0A {
                self.ram_enabled = true;
                return;
            }

            self.ram_enabled = false;
        }

        if addr >= 0x2000 && addr < 0x4000 {
            self.rom_bank_select_register =
                (val & 0x1F) & rom_size_id::get_bank_mask(self.rom[header::ROM_SIZE_ADDR]);
        }

        if addr >= 0x4000 && addr < 0x6000 {
            self.ram_bank_select_register = val & 0x3;
        }

        if addr >= 0x6000 && addr < 0x8000 {
            if val & 0x1 == 0x0 {
                self.banking_mode = MBC1::SIMPLE_BANKING_MODE;
                return;
            }

            self.banking_mode = MBC1::RAM_BANKING_MODE;
        }

        // RAM banks
        if addr >= 0xA000 && addr < 0xC000 {
            if !self.supports_ram() || !self.ram_enabled {
                return;
            }

            let translated_addr: usize = addr - 0xA000;
            let mut bank_number: usize = 0x0;

            if self.rom[header::RAM_SIZE_ADDR] >= ram_size_id::FOUR_BANKS {
                bank_number = usize::from(self.ram_bank_select_register & 0b00000011);
            }

            self.ram_banks[bank_number][translated_addr] = val;
        }
    }
}
