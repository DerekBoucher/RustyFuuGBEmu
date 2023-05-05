#[path = "mbc1_test.rs"]
#[cfg(test)]
mod test;

use crate::memory::cartridge;
use crate::memory::Cartridge;
use std::any::Any;

/// MBC1 type of cartridge has a memory bank controller
/// which swaps out the exposed memory that the cpu sees
/// in memory locations 0x4000 ~ 0x7FFF.
#[derive(Debug)]
pub struct MBC1 {
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

    pub fn new(data: Vec<u8>) -> Box<Self> {
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
        if self.rom[cartridge::header::RAM_SIZE_ADDR] == cartridge::ram_size_id::NO_RAM {
            return false;
        }

        return self.rom[cartridge::header::TYPE_ADDR] == cartridge::mbc_id::MBC1_RAM
            || self.rom[cartridge::header::TYPE_ADDR] == cartridge::mbc_id::MBC1_RAM_BATTERY;
    }
}

impl Cartridge for MBC1 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn read(&self, addr: usize) -> Option<u8> {
        if addr < 0x4000 {
            // If the ROM size is greater than 1MiB, then the cartridge
            // might be using the RAM select register's 2-bits as an extension
            // to bank this area of memory as well. That is, only if banking mode
            // is set to RAM_BANKING_MODE.
            if self.rom[cartridge::header::ROM_SIZE_ADDR] >= cartridge::rom_size_id::ONE_MEGABYTE
                && self.banking_mode == MBC1::RAM_BANKING_MODE
            {
                // TODO: MBC1M Multicart implementation: https://gbdev.io/pandocs/MBC1.html#00003fff--rom-bank-x0-read-only
                let bank_number: usize = usize::from((self.ram_bank_select_register & 0x3) << 5);
                let translated_addr: usize = (0x4000 * bank_number) + addr;
                return Some(self.rom[translated_addr].clone());
            }

            return Some(self.rom[addr].clone());
        }

        // ROM Banks 0x01 - 0x7F. See https://gbdev.io/pandocs/MBC1.html#40007fff--rom-bank-01-7f-read-only for more details.
        if addr >= 0x4000 && addr < 0x8000 {
            let mut translated_addr = addr - 0x4000;
            let mut bank_number: usize = usize::from(self.rom_bank_select_register);
            if bank_number == 0x0 {
                bank_number += 1;
            }

            if self.banking_mode == MBC1::RAM_BANKING_MODE
                && self.rom[cartridge::header::ROM_SIZE_ADDR]
                    >= cartridge::rom_size_id::ONE_MEGABYTE
            {
                bank_number |= usize::from(self.ram_bank_select_register << 5);
            }

            translated_addr += usize::from(bank_number * 0x4000);
            return Some(self.rom[translated_addr].clone());
        }

        // RAM banks
        if addr >= 0xA000 && addr < 0xC000 {
            if !self.supports_ram() || !self.ram_enabled {
                return Some(0xFF);
            }

            let translated_addr: usize = addr - 0xA000;
            let mut bank_number: usize = 0x0;

            if self.rom[cartridge::header::RAM_SIZE_ADDR] >= cartridge::ram_size_id::FOUR_BANKS {
                bank_number = usize::from(self.ram_bank_select_register & 0b00000011);
            }

            return Some(self.ram_banks[bank_number][translated_addr].clone());
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
            self.rom_bank_select_register = (val & 0x1F)
                & cartridge::rom_size_id::get_bank_mask(self.rom[cartridge::header::ROM_SIZE_ADDR]);
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

            if self.rom[cartridge::header::RAM_SIZE_ADDR] >= cartridge::ram_size_id::FOUR_BANKS {
                bank_number = usize::from(self.ram_bank_select_register & 0b00000011);
            }

            self.ram_banks[bank_number][translated_addr] = val;
        }
    }
}
