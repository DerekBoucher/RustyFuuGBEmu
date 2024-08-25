use glium::glutin::event::ElementState;
use queues::{queue, IsQueue, Queue};

use crate::joypad::{ActionButton, DirectionButton};
use crate::{cartridge, interrupt, timers};
use std::sync;
use std::{fmt::Debug, sync::Arc};

const OAM_TRANSFER_CYCLES: u32 = 160;

/// Struct emulating the DMG Gameboy's memory behaviour.
/// This struct controls the access behaviour whenever the CPU
/// makes reads or writes to the memory.
#[derive(Debug)]
pub struct Memory {
    /// Cartridge data.
    /// Mapped into memory locations 0x0000 - 0x7FFF.
    cartridge: Box<dyn cartridge::Interface>,

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
    _echo_ram: [u8; 0x1E00],

    /// Sprite data.
    /// Occupies memory locations 0xFE00 ~ 0xFE9F.
    oam: [u8; 0xA0],

    /// IO Registers such as interupts, controls, etc...
    /// Occupies memory locations 0xFF00 ~ 0xFF7F.
    io_registers: [u8; 0x80],

    /// High RAM used by the CPU.
    /// Occupies memory locations 0xFF80 ~ 0xFFFE.
    hi_ram: [u8; 0x7F],

    /// Flag indicating whether a DMA transfer is in progress.
    oam_dma_transfer_in_progress: bool,

    /// Number of cycles completed during a DMA transfer.
    oam_dma_transfer_cycles_completed: u32,
    oam_hi_byte: u8,

    timer_ref: Arc<sync::Mutex<timers::Timers>>,

    interrupt_bus_ref: Arc<sync::Mutex<interrupt::Bus>>,

    joypad_dir_queue: Queue<(DirectionButton, ElementState)>,
    joypad_action_queue: Queue<(ActionButton, ElementState)>,

    joypad_direction_buffer: u8,
    joypad_action_buffer: u8,
}

/// Module containing important addresses for
/// IO registers.
pub mod io_registers {
    pub const JOYPAD_ADDR: usize = 0xFF00;
    pub const SERIAL_TRANSFER_DATA_ADDR: usize = 0xFF01;
    pub const SERIAL_TRANSFER_CONTROL_ADDR: usize = 0xFF02;
    pub const TIMER_DIV_ADDR: usize = 0xFF04;
    pub const TIMER_COUNTER_ADDR: usize = 0xFF05;
    pub const TIMER_MOD_ADDR: usize = 0xFF06;
    pub const TIMER_CTRL_ADDR: usize = 0xFF07;
    pub const INTERRUPT_FLAG_REGISTER_ADDR: usize = 0xFF0F;
    pub const AUDIO_CH1_SWEEP_ADDR: usize = 0xFF10;
    pub const AUDIO_CH1_LENGTH_ADDR: usize = 0xFF11;
    pub const AUDIO_CH1_VOLUME_ADDR: usize = 0xFF12;
    pub const AUDIO_CH1_WAV_LO_ADDR: usize = 0xFF13;
    pub const AUDIO_CH1_WAV_HI_ADDR: usize = 0xFF14;
    pub const AUDIO_CH2_LENGTH_ADDR: usize = 0xFF16;
    pub const AUDIO_CH2_VOLUME_ADDR: usize = 0xFF17;
    pub const AUDIO_CH2_WAV_LO_ADDR: usize = 0xFF18;
    pub const AUDIO_CH2_WAV_HI_ADDR: usize = 0xFF19;
    pub const AUDIO_CH3_DAC_ENABLE_ADDR: usize = 0xFF1A;
    pub const AUDIO_CH3_LENGTH_ADDR: usize = 0xFF1B;
    pub const AUDIO_CH3_OUTPUT_LVL_ADDR: usize = 0xFF1C;
    pub const AUDIO_CH3_WAV_LO_ADDR: usize = 0xFF1D;
    pub const AUDIO_CH3_WAV_HI_ADDR: usize = 0xFF1E;
    pub const AUDIO_CH4_LENGTH_ADDR: usize = 0xFF20;
    pub const AUDIO_CH4_VOLUME_ADDR: usize = 0xFF21;
    pub const AUDIO_CH4_FREQ_ADDR: usize = 0xFF22;
    pub const AUDIO_CH4_CTRL_ADDR: usize = 0xFF23;
    pub const _AUDIO_WAV_PATTERN_RAM_START_ADDR: usize = 0xFF30;
    pub const AUDIO_GLOBAL_CTRL_ADDR: usize = 0xFF26;
    pub const AUDIO_GLOBAL_PANNING_ADDR: usize = 0xFF25;
    pub const AUDIO_GLOBAL_VOLUME_ADDR: usize = 0xFF24;
    pub const LCD_CONTROL_ADDR: usize = 0xFF40;
    pub const LCD_STAT_ADDR: usize = 0xFF41;
    pub const LCD_SCY_ADDR: usize = 0xFF42;
    pub const LCD_SCX_ADDR: usize = 0xFF43;
    pub const LCD_LY_ADDR: usize = 0xFF44;
    pub const LCD_LYC_ADDR: usize = 0xFF45;
    pub const OAM_DMA_TRANSFER_ADDR: usize = 0xFF46;
    pub const LCD_PALETTE_ADDR: usize = 0xFF47;
    pub const LCD_WINY_ADDR: usize = 0xFF4A;
    pub const LCD_WINX_ADDR: usize = 0xFF4B;
    pub const BOOT_ROM_DISABLE_ADDR: usize = 0xFF50;
    pub const INTERRUPT_ENABLE_REGISTER_ADDR: usize = 0xFFFF;
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

    pub fn new(
        cartridge: Box<dyn cartridge::Interface>,
        timer_ref: Arc<sync::Mutex<timers::Timers>>,
        interrupt_bus_ref: Arc<sync::Mutex<interrupt::Bus>>,
    ) -> Self {
        Self {
            cartridge,
            video_ram: [0x00; 0x2000],
            work_ram0: [0x00; 0x1000],
            work_ram1: [0x00; 0x1000],
            _echo_ram: [0x00; 0x1E00],
            oam: [0x00; 0xA0],
            io_registers: [0x00; 0x80],
            hi_ram: [0x00; 0x7F],
            oam_dma_transfer_cycles_completed: 0,
            oam_dma_transfer_in_progress: false,
            oam_hi_byte: 0,
            timer_ref,
            interrupt_bus_ref,
            joypad_dir_queue: queue![],
            joypad_action_queue: queue![],
            joypad_action_buffer: 0x0F,
            joypad_direction_buffer: 0x0F,
        }
    }

    pub fn default(
        timer_ref: Arc<sync::Mutex<timers::Timers>>,
        interrupt_bus_ref: Arc<sync::Mutex<interrupt::Bus>>,
    ) -> Self {
        Self {
            cartridge: cartridge::default(),
            video_ram: [0x00; 0x2000],
            work_ram0: [0x00; 0x1000],
            work_ram1: [0x00; 0x1000],
            _echo_ram: [0x00; 0x1E00],
            oam: [0x00; 0xA0],
            io_registers: [0x00; 0x80],
            hi_ram: [0x00; 0x7F],
            oam_dma_transfer_cycles_completed: 0,
            oam_dma_transfer_in_progress: false,
            oam_hi_byte: 0,
            timer_ref,
            interrupt_bus_ref,
            joypad_dir_queue: queue![],
            joypad_action_queue: queue![],
            joypad_action_buffer: 0x0F,
            joypad_direction_buffer: 0x0F,
        }
    }

    fn boot_rom_enabled(&self) -> bool {
        return self.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] == 0x00;
    }

    pub fn step_dma(&mut self) {
        if !self.oam_dma_transfer_in_progress {
            return;
        }

        for _ in 0..4 {
            let sprite_data_addr = self.oam_dma_transfer_cycles_completed as usize;

            let data_addr: usize = (self.oam_hi_byte as usize) << 8 | sprite_data_addr;

            self.oam[sprite_data_addr] = self.dma_read(data_addr).unwrap();
            self.oam_dma_transfer_cycles_completed += 1;
        }

        if self.oam_dma_transfer_cycles_completed >= OAM_TRANSFER_CYCLES {
            log::trace!("OAM DMA transfer completed");
            self.oam_dma_transfer_in_progress = false;
            self.oam_dma_transfer_cycles_completed = 0;
        }
    }

    fn write_io_registers(&mut self, addr: usize, val: u8) {
        match addr {
            io_registers::INTERRUPT_FLAG_REGISTER_ADDR => {
                self.interrupt_bus_ref.lock().unwrap().write(addr, val);
            }

            io_registers::OAM_DMA_TRANSFER_ADDR => {
                log::trace!("OAM DMA transfer initiated");
                self.oam_dma_transfer_in_progress = true;

                if val > 0xF1 {
                    panic!("Invalid DMA transfer source address");
                }

                self.oam_hi_byte = val;
            }
            io_registers::TIMER_MOD_ADDR
            | io_registers::TIMER_COUNTER_ADDR
            | io_registers::TIMER_DIV_ADDR
            | io_registers::TIMER_CTRL_ADDR => {
                self.timer_ref.lock().unwrap().write(addr, val);
            }

            io_registers::JOYPAD_ADDR => {
                self.handle_joypad_write(val);
            }
            io_registers::BOOT_ROM_DISABLE_ADDR => {
                self.io_registers[addr - 0xFF00] = val;
            }
            _ => self.io_registers[addr - 0xFF00] = val,
        }
    }

    pub fn write_joypad_queue(
        &mut self,
        direction_press: Option<DirectionButton>,
        action_press: Option<ActionButton>,
        input_state: ElementState,
    ) {
        if direction_press.is_some() {
            match self
                .joypad_dir_queue
                .add((direction_press.unwrap(), input_state))
            {
                Ok(_) => {}
                Err(err) => panic!(
                    "error occurred queuing direction press from joypad: {:?}",
                    err
                ),
            }

            self.interrupt_bus_ref
                .lock()
                .unwrap()
                .request(interrupt::Interrupt::Joypad);

            return;
        }

        if action_press.is_some() {
            match self
                .joypad_action_queue
                .add((action_press.unwrap(), input_state))
            {
                Ok(_) => {}
                Err(err) => panic!("error occurred queuing action press from joypad: {:?}", err),
            }

            self.interrupt_bus_ref
                .lock()
                .unwrap()
                .request(interrupt::Interrupt::Joypad);

            return;
        }
    }

    fn handle_joypad_write(&mut self, val: u8) {
        let action_read = val & (1 << 5) == 0;
        let direction_read = val & (1 << 4) == 0;

        let hi_nibble: u8 = (val | 0xC0) & 0xF0;
        let mut lo_nibble: u8 = 0x0F;

        if action_read {
            match self.joypad_action_queue.remove() {
                Ok((button, input_state)) => {
                    if input_state == ElementState::Pressed {
                        lo_nibble &= button.to_u8();
                    } else {
                        lo_nibble |= !button.to_u8();
                    }

                    self.joypad_action_buffer = lo_nibble & 0x0F;

                    log::trace!(
                        "action press: {:b}, state: {:?}",
                        hi_nibble | self.joypad_action_buffer,
                        input_state
                    );
                }
                Err(_) => {}
            };
        }

        if direction_read {
            match self.joypad_dir_queue.remove() {
                Ok((button, input_state)) => {
                    if input_state == ElementState::Pressed {
                        lo_nibble &= button.to_u8();
                    } else {
                        lo_nibble |= !button.to_u8();
                    }

                    self.joypad_direction_buffer = lo_nibble & 0x0F;

                    log::trace!(
                        "direction press: {:b}, state: {:?}",
                        hi_nibble | self.joypad_direction_buffer,
                        input_state
                    );
                }
                Err(_) => {}
            };
        }

        self.io_registers[io_registers::JOYPAD_ADDR - 0xFF00] = hi_nibble & 0xF0;
    }

    pub fn set_post_boot_rom_state(&mut self) {
        let offset: usize = 0xFF00;

        // Disable boot rom
        self.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - offset] = 0x01;

        self.io_registers[io_registers::JOYPAD_ADDR - offset] = 0xCF;
        self.io_registers[io_registers::SERIAL_TRANSFER_DATA_ADDR - offset] = 0x00;
        self.io_registers[io_registers::SERIAL_TRANSFER_CONTROL_ADDR - offset] = 0x7E;
        self.io_registers[io_registers::TIMER_DIV_ADDR - offset] = 0xAB;
        self.io_registers[io_registers::TIMER_COUNTER_ADDR - offset] = 0x00;
        self.io_registers[io_registers::TIMER_MOD_ADDR - offset] = 0x00;
        self.io_registers[io_registers::TIMER_CTRL_ADDR - offset] = 0xF8;
        self.io_registers[io_registers::AUDIO_CH1_SWEEP_ADDR - offset] = 0x80;
        self.io_registers[io_registers::AUDIO_CH1_LENGTH_ADDR - offset] = 0xBF;
        self.io_registers[io_registers::AUDIO_CH1_VOLUME_ADDR - offset] = 0xF3;
        self.io_registers[io_registers::AUDIO_CH1_WAV_LO_ADDR - offset] = 0xFF;
        self.io_registers[io_registers::AUDIO_CH1_WAV_HI_ADDR - offset] = 0xBF;
        self.io_registers[io_registers::AUDIO_CH2_LENGTH_ADDR - offset] = 0x3F;
        self.io_registers[io_registers::AUDIO_CH2_VOLUME_ADDR - offset] = 0x00;
        self.io_registers[io_registers::AUDIO_CH2_WAV_LO_ADDR - offset] = 0xFF;
        self.io_registers[io_registers::AUDIO_CH2_WAV_HI_ADDR - offset] = 0xBF;
        self.io_registers[io_registers::AUDIO_CH3_DAC_ENABLE_ADDR - offset] = 0x7F;
        self.io_registers[io_registers::AUDIO_CH3_LENGTH_ADDR - offset] = 0xFF;
        self.io_registers[io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR - offset] = 0x9F;
        self.io_registers[io_registers::AUDIO_CH3_WAV_LO_ADDR - offset] = 0xFF;
        self.io_registers[io_registers::AUDIO_CH3_WAV_HI_ADDR - offset] = 0xBF;
        self.io_registers[io_registers::AUDIO_CH4_LENGTH_ADDR - offset] = 0xFF;
        self.io_registers[io_registers::AUDIO_CH4_VOLUME_ADDR - offset] = 0x00;
        self.io_registers[io_registers::AUDIO_CH4_FREQ_ADDR - offset] = 0x00;
        self.io_registers[io_registers::AUDIO_CH4_CTRL_ADDR - offset] = 0xBF;
        self.io_registers[io_registers::AUDIO_GLOBAL_VOLUME_ADDR - offset] = 0x77;
        self.io_registers[io_registers::AUDIO_GLOBAL_PANNING_ADDR - offset] = 0xF3;
        self.io_registers[io_registers::AUDIO_GLOBAL_CTRL_ADDR - offset] = 0xF1;
        self.io_registers[io_registers::LCD_CONTROL_ADDR - offset] = 0x91;
        self.io_registers[io_registers::LCD_STAT_ADDR - offset] = 0x81;
        self.io_registers[io_registers::LCD_SCY_ADDR - offset] = 0x00;
        self.io_registers[io_registers::LCD_SCX_ADDR - offset] = 0x00;
        self.io_registers[io_registers::LCD_LY_ADDR - offset] = 0x91;
        self.io_registers[io_registers::LCD_LYC_ADDR - offset] = 0x00;
        self.io_registers[0xFF46 - offset] = 0xFF;
        self.io_registers[io_registers::LCD_PALETTE_ADDR - offset] = 0xFC;
        self.io_registers[0xFF48 - offset] = 0x00;
        self.io_registers[0xFF49 - offset] = 0x00;
        self.io_registers[io_registers::LCD_WINY_ADDR - offset] = 0x00;
        self.io_registers[io_registers::LCD_WINX_ADDR - offset] = 0x00;
        self.io_registers[0xFF4D - offset] = 0xFF;
        self.io_registers[0xFF4F - offset] = 0xFF;
        self.io_registers[0xFF51 - offset] = 0xFF;
        self.io_registers[0xFF52 - offset] = 0xFF;
        self.io_registers[0xFF53 - offset] = 0xFF;
        self.io_registers[0xFF54 - offset] = 0xFF;
        self.io_registers[0xFF55 - offset] = 0xFF;
        self.io_registers[0xFF56 - offset] = 0xFF;
        self.io_registers[0xFF68 - offset] = 0xFF;
        self.io_registers[0xFF69 - offset] = 0xFF;
        self.io_registers[0xFF6A - offset] = 0xFF;
        self.io_registers[0xFF6B - offset] = 0xFF;
        self.io_registers[0xFF70 - offset] = 0xFF;
    }

    pub fn dma_read(&self, addr: usize) -> Option<u8> {
        if addr < 0x100 && self.boot_rom_enabled() {
            return Some(Memory::BOOT_ROM[addr]);
        }

        // Cartridge ROM
        if addr < 0x8000 {
            return self.cartridge.read(addr);
        }

        // Video RAM
        if addr >= 0x8000 && addr < 0xA000 {
            return Some(self.video_ram[addr - 0x8000]);
        }

        // Cartridge RAM
        if addr >= 0xA000 && addr < 0xC000 {
            return self.cartridge.read(addr);
        }

        // Work RAM 0
        if addr >= 0xC000 && addr < 0xD000 {
            return Some(self.work_ram0[addr - 0xC000]);
        }

        // Work RAM 1
        if addr >= 0xD000 && addr < 0xE000 {
            return Some(self.work_ram1[addr - 0xD000]);
        }

        // Echo RAM
        if addr >= 0xE000 && addr < 0xFE00 {
            return self.dma_read((addr - 0xE000) + 0xC000);
        }

        // OAM / Sprite attributes
        if addr >= 0xFE00 && addr < 0xFEA0 {
            return Some(self.oam[addr - 0xFE00]);
        }

        if addr >= 0xFEA0 && addr < 0xFF00 {
            return Some(0xFF);
        }

        // IO Registers
        if addr >= 0xFF00 && addr < 0xFF80 {
            return match addr {
                io_registers::TIMER_DIV_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_COUNTER_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_MOD_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_CTRL_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),

                io_registers::INTERRUPT_FLAG_REGISTER_ADDR => {
                    Some(self.interrupt_bus_ref.lock().unwrap().read(addr))
                }
                _ => Some(self.io_registers[addr - 0xFF00]),
            };
        }

        // High RAM
        if addr >= 0xFF80 && addr < 0xFFFF {
            return Some(self.hi_ram[addr - 0xFF80]);
        }

        // Interupt enable register
        if addr == 0xFFFF {
            return Some(self.interrupt_bus_ref.lock().unwrap().read(addr));
        }

        panic!("invalid dma read address specified: {}", addr);
    }

    pub fn read(&mut self, addr: usize) -> Option<u8> {
        if self.oam_dma_transfer_in_progress {
            // Only High RAM is accessible during an oam dma transfer.
            if addr >= 0xFF80 && addr < 0xFFFF {
                return Some(self.hi_ram[addr - 0xFF80]);
            }

            // Else, return dummy data
            return Some(0xFF);
        }

        // If boot rom is enabled, the data should come from it.
        if addr < 0x100 && self.boot_rom_enabled() {
            return Some(Memory::BOOT_ROM[addr]);
        }

        // Cartridge ROM
        if addr < 0x8000 {
            return self.cartridge.read(addr);
        }

        // Video RAM
        if addr >= 0x8000 && addr < 0xA000 {
            return Some(self.video_ram[addr - 0x8000]);
        }

        // Cartridge RAM
        if addr >= 0xA000 && addr < 0xC000 {
            return self.cartridge.read(addr);
        }

        // Work RAM 0
        if addr >= 0xC000 && addr < 0xD000 {
            return Some(self.work_ram0[addr - 0xC000]);
        }

        // Work RAM 1
        if addr >= 0xD000 && addr < 0xE000 {
            return Some(self.work_ram1[addr - 0xD000]);
        }

        // Echo RAM
        if addr >= 0xE000 && addr < 0xFE00 {
            return self.read((addr - 0xE000) + 0xC000);
        }

        // OAM / Sprite attributes
        if addr >= 0xFE00 && addr < 0xFEA0 {
            return Some(self.oam[addr - 0xFE00]);
        }

        if addr >= 0xFEA0 && addr < 0xFF00 {
            return Some(0xFF);
        }

        // IO Registers
        if addr >= 0xFF00 && addr < 0xFF80 {
            return match addr {
                io_registers::TIMER_DIV_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_COUNTER_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_MOD_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),
                io_registers::TIMER_CTRL_ADDR => Some(self.timer_ref.lock().unwrap().read(addr)),

                io_registers::INTERRUPT_FLAG_REGISTER_ADDR => {
                    Some(self.interrupt_bus_ref.lock().unwrap().read(addr))
                }
                io_registers::JOYPAD_ADDR => self.handle_joypad_read(),
                _ => Some(self.io_registers[addr - 0xFF00]),
            };
        }

        // High RAM
        if addr >= 0xFF80 && addr < 0xFFFF {
            return Some(self.hi_ram[addr - 0xFF80]);
        }

        // Interupt enable register
        if addr == 0xFFFF {
            return Some(self.interrupt_bus_ref.lock().unwrap().read(addr));
        }

        panic!("invalid read address specified: {}", addr);
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        if self.oam_dma_transfer_in_progress {
            // Only High RAM is accessible during an oam dma transfer.
            if addr >= 0xFF80 && addr < 0xFFFF {
                self.hi_ram[addr - 0xFF80] = val;
            }

            return;
        }

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
        // Not writable directly, needs to be performed via a OAM DMA transfer.
        if addr >= 0xFE00 && addr < 0xFF00 {
            //return;
        }

        // IO Registers
        if addr >= 0xFF00 && addr < 0xFF80 {
            self.write_io_registers(addr, val);
        }

        // High RAM
        if addr >= 0xFF80 && addr < 0xFFFF {
            self.hi_ram[addr - 0xFF80] = val;
        }

        // Interupt enable register
        if addr == 0xFFFF {
            self.interrupt_bus_ref.lock().unwrap().write(addr, val);
        }
    }

    pub fn reset(&mut self, cartridge: Box<dyn cartridge::Interface>) {
        *self = Memory::new(
            cartridge,
            self.timer_ref.clone(),
            self.interrupt_bus_ref.clone(),
        );
    }

    fn handle_joypad_read(&self) -> Option<u8> {
        let hi_nibble = self.io_registers[io_registers::JOYPAD_ADDR - 0xFF00] & 0xF0;
        let action_read = hi_nibble & (1 << 5) == 0;
        let direction_read = hi_nibble & (1 << 4) == 0;

        if action_read {
            return Some(hi_nibble | self.joypad_action_buffer);
        }

        if direction_read {
            return Some(hi_nibble | self.joypad_direction_buffer);
        }

        Some(hi_nibble | 0x0F)
    }
}
