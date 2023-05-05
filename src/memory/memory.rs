#[path = "memory_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::MemoryDriver;
use crate::memory::cartridge;
use crate::memory::Memory;

/// Module containing important addresses for
/// IO registers.
mod io_registers {
    const JOYPAD_ADDR: usize = 0xFF00;
    const SERIAL_TRANSFER_DATA_ADDR: usize = 0xFF01;
    const SERIAL_TRANSFER_CONTROL_ADDR: usize = 0xFF02;
    const TIMER_DIV_ADDR: usize = 0xFF04;
    const TIMER_COUNTER_ADDR: usize = 0xFF05;
    const TIMER_MOD_ADDR: usize = 0xFF06;
    const TIMER_CTRL_ADDR: usize = 0xFF07;
    const AUDIO_CH1_SWEEP_ADDR: usize = 0xFF10;
    const AUDIO_CH1_LENGTH_ADDR: usize = 0xFF11;
    const AUDIO_CH1_VOLUME_ADDR: usize = 0xFF12;
    const AUDIO_CH1_WAV_LO_ADDR: usize = 0xFF13;
    const AUDIO_CH1_WAV_HI_ADDR: usize = 0xFF14;
    const AUDIO_CH2_LENGTH_ADDR: usize = 0xFF16;
    const AUDIO_CH2_VOLUME_ADDR: usize = 0xFF17;
    const AUDIO_CH2_WAV_LO_ADDR: usize = 0xFF18;
    const AUDIO_CH2_WAV_HI_ADDR: usize = 0xFF19;
    const AUDIO_CH3_DAC_ENABLE_ADDR: usize = 0xFF1A;
    const AUDIO_CH3_LENGTH_ADDR: usize = 0xFF1B;
    const AUDIO_CH3_OUTPUT_LVL_ADDR: usize = 0xFF1C;
    const AUDIO_CH3_WAV_LO_ADDR: usize = 0xFF1D;
    const AUDIO_CH3_WAV_HI_ADDR: usize = 0xFF1E;
    const AUDIO_CH4_LENGTH_ADDR: usize = 0xFF20;
    const AUDIO_CH4_VOLUME_ADDR: usize = 0xFF21;
    const AUDIO_CH4_FREQ_ADDR: usize = 0xFF22;
    const AUDIO_CH4_CTRL_ADDR: usize = 0xFF23;
    const AUDIO_WAV_PATTERN_RAM_START_ADDR: usize = 0xFF30;
    const AUDIO_GLOBAL_CTRL_ADDR: usize = 0xFF26;
    const AUDIO_GLOBAL_PANNING_ADDR: usize = 0xFF25;
    const AUDIO_GLOBAL_VOLUME_ADDR: usize = 0xFF24;
    const LCD_CONTROL_ADDR: usize = 0xFF40;
    const LCD_STAT_ADDR: usize = 0xFF41;
    const LCD_SCY_ADDR: usize = 0xFF42;
    const LCD_SCX_ADDR: usize = 0xFF43;
    const LCD_LY_ADDR: usize = 0xFF44;
    const LCD_LYC_ADDR: usize = 0xFF45;
    const LCD_WINY_ADDR: usize = 0xFF4A;
    const LCD_WINX_ADDR: usize = 0xFF4B;
    pub const BOOT_ROM_DISABLE_ADDR: usize = 0xFF50;
}

impl Memory {
    const BOOT_ROM: [u8; 0x100] = [
        0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF,
        0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
        0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96,
        0x00, 0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22,
        0x23, 0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D,
        0x28, 0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0,
        0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20,
        0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62,
        0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0,
        0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F,
        0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23,
        0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83,
        0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E,
        0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
        0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x21,
        0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
        0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0,
        0x50,
    ];

    pub fn new(cartridge_data: Vec<u8>) -> Self {
        Self {
            cartridge: cartridge::new(cartridge_data),
            video_ram: [0x00; 0x2000],
            work_ram0: [0x00; 0x1000],
            work_ram1: [0x00; 0x1000],
            echo_ram: [0x00; 0x1E00],
            sprite_attributes: [0x00; 0xA0],
            io_registers: [0x00; 0x80],
            hi_ram: [0x00; 0x7F],
            interrupt_enable_register: 0x00,
        }
    }

    fn boot_rom_enabled(&self) -> bool {
        return self.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] == 0x00;
    }

    pub fn read(&self, addr: usize) -> Option<u8> {
        // If boot rom is enabled, the data should come from it.
        if addr < 0x100 && self.boot_rom_enabled() {
            return Some(Memory::BOOT_ROM[addr].clone());
        }

        // Cartridge ROM
        if addr < 0x8000 {
            return self.cartridge.read(addr);
        }

        // Video RAM
        if addr >= 0x8000 && addr < 0xA000 {
            return Some(self.video_ram[addr - 0x8000].clone());
        }

        // Cartridge RAM
        if addr >= 0xA000 && addr < 0xC000 {
            return self.cartridge.read(addr);
        }

        // Work RAM 0
        if addr >= 0xC000 && addr < 0xD000 {
            return Some(self.work_ram0[addr - 0xC000].clone());
        }

        // Work RAM 1
        if addr >= 0xD000 && addr < 0xE000 {
            return Some(self.work_ram1[addr - 0xD000].clone());
        }

        // Echo RAM
        if addr >= 0xE000 && addr < 0xFE00 {
            return self.read((addr - 0xE000) + 0xC000);
        }

        // OAM / Sprite attributes
        if addr >= 0xFE00 && addr < 0xFF00 {
            return Some(self.sprite_attributes[addr - 0xFE00].clone());
        }

        // IO Registers
        if addr >= 0xFF00 && addr < 0xFF80 {
            return Some(self.io_registers[addr - 0xFF00].clone());
        }

        // High RAM
        if addr >= 0xFF80 && addr < 0xFFFF {
            return Some(self.hi_ram[addr - 0xFF80].clone());
        }

        // Interupt enable register
        if addr == 0xFFFF {
            return Some(self.interrupt_enable_register.clone());
        }

        None
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        // Cartridge ROM
        if addr < 0x8000 {
            self.cartridge.write(addr, val)
        }

        // Video RAM
        if addr >= 0x8000 && addr < 0xA000 {
            self.video_ram[addr - 0x8000] = val;
        }

        // Cartridge RAM
        if addr >= 0xA000 && addr < 0xC000 {
            self.cartridge.write(addr, val);
        }

        // Work RAM 0
        if addr >= 0xC000 && addr < 0xD000 {
            self.work_ram0[addr - 0xC000] = val;
        }

        // Work RAM 1
        if addr >= 0xD000 && addr < 0xE000 {
            self.work_ram1[addr - 0xD000] = val;
        }

        // Echo RAM
        if addr >= 0xE000 && addr < 0xFE00 {
            self.write((addr - 0xE000) + 0xC000, val);
        }

        // OAM / Sprite attributes
        if addr >= 0xFE00 && addr < 0xFF00 {
            self.sprite_attributes[addr - 0xFE00] = val;
        }

        // IO Registers
        if addr >= 0xFF00 && addr < 0xFF80 {
            self.io_registers[addr - 0xFF00] = val;
        }

        // High RAM
        if addr >= 0xFF80 && addr < 0xFFFF {
            self.hi_ram[addr - 0xFF80] = val;
        }

        // Interupt enable register
        if addr == 0xFFFF {
            self.interrupt_enable_register = val;
        }
    }
}

impl MemoryDriver for Memory {
    fn read(&self, addr: usize) -> Option<u8> {
        return self.read(addr);
    }

    fn write(&mut self, addr: usize, val: u8) {
        return self.write(addr, val);
    }

    fn dump(&self) -> Vec<u8> {
        return vec![]; // TODO
    }
}
