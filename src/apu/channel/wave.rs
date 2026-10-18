use crate::memory::io_registers;

use super::LengthCounter;

const TRIGGER_MASK: u8 = 1 << 7;
const LENGTH_ENABLE_MASK: u8 = 1 << 6;
const DAC_ENABLE_MASK: u8 = 1 << 7;
const WAVE_RAM_LEN: usize = 16;

/// Wave channel (channel 3), backed by its own wave RAM.
#[derive(Debug)]
pub struct WaveChannel {
    wave_ram: [u8; WAVE_RAM_LEN],
    position: u8,
    frequency: u16,
    timer: u16,
    volume_code: u8,
    enabled: bool,
    dac_enabled: bool,
    length: LengthCounter,
}

impl WaveChannel {
    pub fn new() -> Self {
        WaveChannel {
            wave_ram: [0; WAVE_RAM_LEN],
            position: 0,
            frequency: 0,
            timer: 0,
            volume_code: 0,
            enabled: false,
            dac_enabled: false,
            length: LengthCounter::new(),
        }
    }

    /// Resets the channel to its power-on state, preserving wave RAM (which
    /// is not cleared when the APU is powered off on the DMG).
    pub fn reset(&mut self) {
        let wave_ram = self.wave_ram;
        *self = WaveChannel::new();
        self.wave_ram = wave_ram;
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::AUDIO_CH3_DAC_ENABLE_ADDR => {
                self.dac_enabled = value & DAC_ENABLE_MASK > 0;

                if !self.dac_enabled {
                    self.enabled = false;
                }
            }
            io_registers::AUDIO_CH3_LENGTH_ADDR => self.length.write(value),
            io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR => self.volume_code = (value >> 5) & 0x03,
            io_registers::AUDIO_CH3_WAV_LO_ADDR => {
                self.frequency = (self.frequency & 0x0700) | u16::from(value);
            }
            io_registers::AUDIO_CH3_WAV_HI_ADDR => {
                self.frequency = (self.frequency & 0x00FF) | (u16::from(value & 0x07) << 8);
                self.length.set_enabled(value & LENGTH_ENABLE_MASK > 0);

                if value & TRIGGER_MASK > 0 {
                    self.trigger();
                }
            }
            _ => panic!("invalid wave channel register write: 0x{:X}", addr),
        }
    }

    pub fn read_wave_ram(&self, addr: usize) -> u8 {
        self.wave_ram[addr - io_registers::AUDIO_WAV_PATTERN_RAM_START_ADDR]
    }

    pub fn write_wave_ram(&mut self, addr: usize, value: u8) {
        self.wave_ram[addr - io_registers::AUDIO_WAV_PATTERN_RAM_START_ADDR] = value;
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            self.timer = self.frequency_period();
            self.position = (self.position + 1) & 0x1F;
        }
    }

    /// Clocked by the frame sequencer on steps 0, 2, 4 and 6.
    pub fn clock_length(&mut self) {
        if self.length.clock() {
            self.enabled = false;
        }
    }

    pub fn output(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        match self.volume_code {
            0 => 0,
            code => self.sample() >> (code - 1),
        }
    }

    pub fn is_active(&self) -> bool {
        self.enabled
    }

    fn trigger(&mut self) {
        self.enabled = self.dac_enabled;
        self.length.trigger();
        self.timer = self.frequency_period();
        self.position = 0;
    }

    /// The nibble at the current playback position.
    fn sample(&self) -> u8 {
        let byte = self.wave_ram[(self.position >> 1) as usize];

        if self.position & 1 == 0 {
            byte >> 4
        } else {
            byte & 0x0F
        }
    }

    fn frequency_period(&self) -> u16 {
        (2048 - self.frequency) * 2
    }
}
