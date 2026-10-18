use crate::memory::io_registers;

use super::{Envelope, LengthCounter};

const TRIGGER_MASK: u8 = 1 << 7;
const LENGTH_ENABLE_MASK: u8 = 1 << 6;
const WIDTH_MODE_MASK: u8 = 1 << 3;
const LFSR_RESET: u16 = 0x7FFF;

/// Noise channel (channel 4), driven by a linear feedback shift register.
#[derive(Debug)]
pub struct NoiseChannel {
    lfsr: u16,
    divisor_code: u8,
    shift: u8,
    width_mode: bool,
    timer: u32,
    enabled: bool,
    dac_enabled: bool,
    envelope: Envelope,
    length: LengthCounter,
}

impl NoiseChannel {
    pub fn new() -> Self {
        NoiseChannel {
            lfsr: LFSR_RESET,
            divisor_code: 0,
            shift: 0,
            width_mode: false,
            timer: 0,
            enabled: false,
            dac_enabled: false,
            envelope: Envelope::new(),
            length: LengthCounter::new(),
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::AUDIO_CH4_LENGTH_ADDR => self.length.write(value),
            io_registers::AUDIO_CH4_VOLUME_ADDR => {
                self.dac_enabled = self.envelope.write(value);

                if !self.dac_enabled {
                    self.enabled = false;
                }
            }
            io_registers::AUDIO_CH4_FREQ_ADDR => {
                self.shift = value >> 4;
                self.width_mode = value & WIDTH_MODE_MASK > 0;
                self.divisor_code = value & 0x07;
            }
            io_registers::AUDIO_CH4_CTRL_ADDR => {
                self.length.set_enabled(value & LENGTH_ENABLE_MASK > 0);

                if value & TRIGGER_MASK > 0 {
                    self.trigger();
                }
            }
            _ => panic!("invalid noise channel register write: 0x{:X}", addr),
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            self.timer = self.frequency_period();

            let feedback = (self.lfsr ^ (self.lfsr >> 1)) & 1;
            self.lfsr = (self.lfsr >> 1) | (feedback << 14);

            if self.width_mode {
                self.lfsr = (self.lfsr & !(1 << 6)) | (feedback << 6);
            }
        }
    }

    /// Clocked by the frame sequencer on steps 0, 2, 4 and 6.
    pub fn clock_length(&mut self) {
        if self.length.clock() {
            self.enabled = false;
        }
    }

    /// Clocked by the frame sequencer on step 7.
    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn output(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        let bit = !self.lfsr & 1;

        (bit as u8) * self.envelope.volume()
    }

    pub fn is_active(&self) -> bool {
        self.enabled
    }

    fn trigger(&mut self) {
        self.enabled = self.dac_enabled;
        self.length.trigger();
        self.timer = self.frequency_period();
        self.envelope.trigger();
        self.lfsr = LFSR_RESET;
    }

    fn frequency_period(&self) -> u32 {
        let divisor = if self.divisor_code == 0 {
            8
        } else {
            u32::from(self.divisor_code) * 16
        };

        divisor << self.shift
    }
}
