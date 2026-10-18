use crate::memory::io_registers;

use super::{Envelope, LengthCounter, DUTY_PATTERNS, MAX_FREQUENCY};

const TRIGGER_MASK: u8 = 1 << 7;
const LENGTH_ENABLE_MASK: u8 = 1 << 6;

/// Square wave channel, used for both channel 1 (which additionally has a
/// frequency sweep unit) and channel 2.
#[derive(Debug)]
pub struct SquareChannel {
    duty: u8,
    frequency: u16,
    timer: u16,
    duty_position: u8,
    enabled: bool,
    dac_enabled: bool,
    envelope: Envelope,
    length: LengthCounter,
    sweep: Option<Sweep>,
}

impl SquareChannel {
    pub fn new() -> Self {
        SquareChannel {
            duty: 0,
            frequency: 0,
            timer: 0,
            duty_position: 0,
            enabled: false,
            dac_enabled: false,
            envelope: Envelope::new(),
            length: LengthCounter::new(),
            sweep: None,
        }
    }

    /// Channel 1, which additionally has a frequency sweep unit.
    pub fn new_with_sweep() -> Self {
        SquareChannel {
            sweep: Some(Sweep::new()),
            ..Self::new()
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::AUDIO_CH1_SWEEP_ADDR => {
                if let Some(sweep) = self.sweep.as_mut() {
                    sweep.write(value);
                }
            }
            io_registers::AUDIO_CH1_LENGTH_ADDR | io_registers::AUDIO_CH2_LENGTH_ADDR => {
                self.duty = value >> 6;
                self.length.write(value);
            }
            io_registers::AUDIO_CH1_VOLUME_ADDR | io_registers::AUDIO_CH2_VOLUME_ADDR => {
                self.dac_enabled = self.envelope.write(value);

                if !self.dac_enabled {
                    self.enabled = false;
                }
            }
            io_registers::AUDIO_CH1_WAV_LO_ADDR | io_registers::AUDIO_CH2_WAV_LO_ADDR => {
                self.frequency = (self.frequency & 0x0700) | u16::from(value);
            }
            io_registers::AUDIO_CH1_WAV_HI_ADDR | io_registers::AUDIO_CH2_WAV_HI_ADDR => {
                self.frequency = (self.frequency & 0x00FF) | (u16::from(value & 0x07) << 8);
                self.length.set_enabled(value & LENGTH_ENABLE_MASK > 0);

                if value & TRIGGER_MASK > 0 {
                    self.trigger();
                }
            }
            _ => panic!("invalid square channel register write: 0x{:X}", addr),
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            self.timer = self.frequency_period();
            self.duty_position = (self.duty_position + 1) & 0x07;
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

    /// Clocked by the frame sequencer on steps 2 and 6. The sweep unit only
    /// exists on channel 1; this is a no-op for channel 2.
    pub fn clock_sweep(&mut self) {
        let Some(sweep) = self.sweep.as_mut() else {
            return;
        };

        if sweep.timer > 0 {
            sweep.timer -= 1;
        }

        if sweep.timer != 0 {
            return;
        }

        sweep.timer = if sweep.period == 0 { 8 } else { sweep.period };

        if !sweep.enabled || sweep.period == 0 {
            return;
        }

        let new_frequency = sweep.calculate();

        if !(0..=i32::from(MAX_FREQUENCY)).contains(&new_frequency) {
            self.enabled = false;
            return;
        }

        self.frequency = new_frequency as u16;
        sweep.shadow_frequency = new_frequency as u16;

        // Writing the new frequency back can itself overflow; the channel is
        // then disabled without storing the second result.
        if !(0..=i32::from(MAX_FREQUENCY)).contains(&sweep.calculate()) {
            self.enabled = false;
        }
    }

    pub fn output(&self) -> u8 {
        if !self.enabled {
            return 0;
        }

        let bit = (DUTY_PATTERNS[self.duty as usize] >> (7 - self.duty_position)) & 1;

        bit * self.envelope.volume()
    }

    pub fn is_active(&self) -> bool {
        self.enabled
    }

    fn trigger(&mut self) {
        self.enabled = self.dac_enabled;
        self.length.trigger();
        self.timer = self.frequency_period();
        self.envelope.trigger();
        self.duty_position = 0;

        if let Some(sweep) = self.sweep.as_mut() {
            sweep.shadow_frequency = self.frequency;
            sweep.timer = if sweep.period == 0 { 8 } else { sweep.period };
            sweep.enabled = sweep.period > 0 || sweep.shift > 0;

            if sweep.shift > 0 {
                let new_frequency = sweep.calculate();

                if (0..=i32::from(MAX_FREQUENCY)).contains(&new_frequency) {
                    self.frequency = new_frequency as u16;
                } else {
                    self.enabled = false;
                }
            }
        }
    }

    fn frequency_period(&self) -> u16 {
        (2048 - self.frequency) * 4
    }
}

/// Frequency sweep unit, only present on channel 1.
#[derive(Debug)]
struct Sweep {
    period: u8,
    negate: bool,
    shift: u8,
    shadow_frequency: u16,
    timer: u8,
    enabled: bool,
}

impl Sweep {
    fn new() -> Self {
        Sweep {
            period: 0,
            negate: false,
            shift: 0,
            shadow_frequency: 0,
            timer: 0,
            enabled: false,
        }
    }

    /// Applies an NR10 register write.
    fn write(&mut self, value: u8) {
        self.period = (value >> 4) & 0x07;
        self.negate = value & 0x08 > 0;
        self.shift = value & 0x07;
    }

    /// The new frequency a sweep calculation would produce. Can be negative
    /// when subtraction is enabled.
    fn calculate(&self) -> i32 {
        let delta = i32::from(self.shadow_frequency >> self.shift);

        if self.negate {
            i32::from(self.shadow_frequency) - delta
        } else {
            i32::from(self.shadow_frequency) + delta
        }
    }
}
