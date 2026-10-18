use super::{Apu, AUDIO_REGISTER_BASE, AUDIO_REGISTER_COUNT};
use crate::memory::io_registers;

/// Register values of a DMG that has just finished running its boot ROM.
const POST_BOOT_REGISTER_VALUES: [(usize, u8); 21] = [
    (io_registers::AUDIO_CH1_SWEEP_ADDR, 0x80),
    (io_registers::AUDIO_CH1_LENGTH_ADDR, 0xBF),
    (io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF3),
    (io_registers::AUDIO_CH1_WAV_LO_ADDR, 0xFF),
    (io_registers::AUDIO_CH1_WAV_HI_ADDR, 0xBF),
    (io_registers::AUDIO_CH2_LENGTH_ADDR, 0x3F),
    (io_registers::AUDIO_CH2_VOLUME_ADDR, 0x00),
    (io_registers::AUDIO_CH2_WAV_LO_ADDR, 0xFF),
    (io_registers::AUDIO_CH2_WAV_HI_ADDR, 0xBF),
    (io_registers::AUDIO_CH3_DAC_ENABLE_ADDR, 0x7F),
    (io_registers::AUDIO_CH3_LENGTH_ADDR, 0xFF),
    (io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x9F),
    (io_registers::AUDIO_CH3_WAV_LO_ADDR, 0xFF),
    (io_registers::AUDIO_CH3_WAV_HI_ADDR, 0xBF),
    (io_registers::AUDIO_CH4_LENGTH_ADDR, 0xFF),
    (io_registers::AUDIO_CH4_VOLUME_ADDR, 0x00),
    (io_registers::AUDIO_CH4_FREQ_ADDR, 0x00),
    (io_registers::AUDIO_CH4_CTRL_ADDR, 0xBF),
    (io_registers::AUDIO_GLOBAL_VOLUME_ADDR, 0x77),
    (io_registers::AUDIO_GLOBAL_PANNING_ADDR, 0xF3),
    (io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0xF1),
];

impl Apu {
    pub fn new() -> Self {
        Apu {
            registers: [0x00; AUDIO_REGISTER_COUNT],
            sample_buffer: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Apu::new();
    }

    /// Reads an audio register. `Memory` only routes addresses within
    /// 0xFF10 - 0xFF3F here.
    pub fn read(&self, addr: usize) -> u8 {
        self.registers[addr - AUDIO_REGISTER_BASE]
    }

    /// Writes an audio register. Register semantics (read/write masks,
    /// channel triggering, power control, ...) are added together with the
    /// channel implementations.
    pub fn write(&mut self, addr: usize, value: u8) {
        self.registers[addr - AUDIO_REGISTER_BASE] = value;
    }

    /// Advances the APU by one machine cycle (4 T-cycles), in lockstep with
    /// the timers and the PPU. `div_clock` is the system clock maintained by
    /// `Timers`; its bit 12 (DIV bit 4) drives the 512 Hz frame sequencer.
    pub fn step(&mut self, _div_clock: u16) {
        // Channel state machines and sample generation are implemented in a
        // later phase; wiring the APU into the system step loop first keeps
        // the integration verifiable on its own.
    }

    /// Hands the samples accumulated since the last frame boundary over to
    /// the front end, clearing the internal buffer.
    pub fn take_sample_buffer(&mut self) -> Vec<i16> {
        std::mem::take(&mut self.sample_buffer)
    }

    pub fn set_post_boot_rom_state(&mut self) {
        for (addr, value) in POST_BOOT_REGISTER_VALUES {
            self.registers[addr - AUDIO_REGISTER_BASE] = value;
        }
    }
}
