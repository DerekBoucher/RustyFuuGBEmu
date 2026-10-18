//! Shared building blocks for the four APU channels.
//!
//! The channel outputs (`output` methods) are consumed by the mixer, which
//! is implemented in a later phase.

#![allow(dead_code)] // TODO: remove once the mixer consumes the channel outputs.

pub mod noise;
pub mod square;
pub mod wave;

/// Duty cycle patterns for the square channels, from the first output bit
/// (most significant) to the last.
pub const DUTY_PATTERNS: [u8; 4] = [0b00000001, 0b00000011, 0b00001111, 0b11111100];

/// Maximum value of an 11-bit frequency register.
pub const MAX_FREQUENCY: u16 = 2047;

/// Value a length counter reloads to when triggered with a count of 0.
pub const MAX_LENGTH: u16 = 64;

/// Volume envelope shared by channels 1, 2 and 4.
#[derive(Debug)]
pub struct Envelope {
    initial_volume: u8,
    add: bool,
    period: u8,
    timer: u8,
    volume: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            initial_volume: 0,
            add: false,
            period: 0,
            timer: 0,
            volume: 0,
        }
    }

    /// Applies an NRx2 register write. Returns whether the channel's DAC
    /// remains enabled.
    pub fn write(&mut self, value: u8) -> bool {
        self.initial_volume = value >> 4;
        self.add = value & 0x08 > 0;
        self.period = value & 0x07;

        value & 0xF8 > 0
    }

    pub fn trigger(&mut self) {
        self.timer = self.period;
        self.volume = self.initial_volume;
    }

    /// Clocked by the frame sequencer, once per envelope step.
    pub fn clock(&mut self) {
        if self.period == 0 {
            return;
        }

        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            self.timer = self.period;

            if self.add && self.volume < 15 {
                self.volume += 1;
            } else if !self.add && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }
}

/// Length counter shared by all four channels.
#[derive(Debug)]
pub struct LengthCounter {
    value: u16,
    enabled: bool,
}

impl LengthCounter {
    pub fn new() -> Self {
        LengthCounter {
            value: 0,
            enabled: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.value = MAX_LENGTH - u16::from(value & 0x3F);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn trigger(&mut self) {
        if self.value == 0 {
            self.value = MAX_LENGTH;
        }
    }

    /// Clocked by the frame sequencer. Returns true once the counter has
    /// expired, at which point the channel has to be disabled.
    pub fn clock(&mut self) -> bool {
        if !self.enabled || self.value == 0 {
            return false;
        }

        self.value -= 1;
        self.value == 0
    }
}
