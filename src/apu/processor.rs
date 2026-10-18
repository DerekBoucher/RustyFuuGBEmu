use super::channel::{noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};
use super::Apu;
use crate::cpu::CPU_FREQUENCY;
use crate::memory::io_registers;

/// Output sample rate of the generated PCM stream.
pub(crate) const SAMPLE_RATE: u32 = 48_000;

/// Number of T-cycles the APU is advanced by per `step` call (one machine
/// cycle).
const T_CYCLES_PER_STEP: u32 = 4;

/// Digital value of a channel's DAC input that maps to the center of the
/// output range.
const DAC_CENTER: i32 = 8;

/// Scale factor applied to a mixed channel sum to produce a 16-bit PCM
/// sample, leaving headroom below the full scale.
const OUTPUT_SCALE: i32 = 47;

const POWER_MASK: u8 = 1 << 7;
const STATUS_MASK: u8 = 0x70;

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
            ch1: SquareChannel::new_with_sweep(),
            ch2: SquareChannel::new(),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            nr50: 0,
            nr51: 0,
            power_on: false,
            frame_sequencer_step: 0,
            last_div_bit_4: false,
            sample_accumulator: 0,
            mixed_left_accumulator: 0,
            mixed_right_accumulator: 0,
            mixed_t_cycles: 0,
            sample_buffer: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Apu::new();
    }

    /// Reads an audio register. `Memory` only routes addresses within
    /// 0xFF10 - 0xFF3F here.
    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            io_registers::AUDIO_GLOBAL_CTRL_ADDR => self.read_global_ctrl(),
            io_registers::AUDIO_WAV_PATTERN_RAM_START_ADDR
                ..=io_registers::AUDIO_WAV_PATTERN_RAM_END_ADDR => self.ch3.read_wave_ram(addr),
            // On the DMG all remaining audio registers are write-only and
            // read back as 0xFF. The unused addresses (0xFF15, 0xFF1F and
            // 0xFF27 - 0xFF2F) do the same.
            _ => 0xFF,
        }
    }

    /// Writes an audio register. `Memory` only routes addresses within
    /// 0xFF10 - 0xFF3F here.
    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::AUDIO_GLOBAL_CTRL_ADDR => self.write_global_ctrl(value),
            // Wave RAM stays accessible while the APU is off on the DMG.
            io_registers::AUDIO_WAV_PATTERN_RAM_START_ADDR
                ..=io_registers::AUDIO_WAV_PATTERN_RAM_END_ADDR => {
                self.ch3.write_wave_ram(addr, value)
            }
            // While the APU is off all remaining registers are cleared and
            // ignore writes.
            _ if !self.power_on => {}
            io_registers::AUDIO_CH1_SWEEP_ADDR..=io_registers::AUDIO_CH1_WAV_HI_ADDR => {
                self.ch1.write(addr, value)
            }
            io_registers::AUDIO_CH2_LENGTH_ADDR..=io_registers::AUDIO_CH2_WAV_HI_ADDR => {
                self.ch2.write(addr, value)
            }
            io_registers::AUDIO_CH3_DAC_ENABLE_ADDR..=io_registers::AUDIO_CH3_WAV_HI_ADDR => {
                self.ch3.write(addr, value)
            }
            io_registers::AUDIO_CH4_LENGTH_ADDR..=io_registers::AUDIO_CH4_CTRL_ADDR => {
                self.ch4.write(addr, value)
            }
            io_registers::AUDIO_GLOBAL_VOLUME_ADDR => self.nr50 = value,
            io_registers::AUDIO_GLOBAL_PANNING_ADDR => self.nr51 = value,
            // Unused addresses (0xFF15, 0xFF1F, 0xFF27 - 0xFF2F) are not
            // writable on the DMG.
            _ => {}
        }
    }

    fn write_global_ctrl(&mut self, value: u8) {
        let powered_on = value & POWER_MASK > 0;

        if powered_on && !self.power_on {
            // Powering on: the registers were already cleared when power was
            // last cut, so there is nothing to restore.
            self.power_on = true;
        } else if !powered_on && self.power_on {
            self.power_off();
        }
    }

    /// Clears the APU registers and resets all channels and the frame
    /// sequencer. Wave RAM is preserved on the DMG.
    fn power_off(&mut self) {
        self.ch1 = SquareChannel::new_with_sweep();
        self.ch2 = SquareChannel::new();
        self.ch3.reset();
        self.ch4 = NoiseChannel::new();
        self.nr50 = 0;
        self.nr51 = 0;
        self.power_on = false;
        self.frame_sequencer_step = 0;
    }

    /// Advances the APU by one machine cycle (4 T-cycles), in lockstep with
    /// the timers and the PPU. `div_clock` is the system clock maintained by
    /// `Timers`; the falling edge of its bit 12 (DIV bit 4) clocks the
    /// 512 Hz frame sequencer.
    pub fn step(&mut self, div_clock: u16) {
        let div_bit_4 = div_clock & (1 << 12) > 0;

        if self.last_div_bit_4 && !div_bit_4 {
            self.tick_frame_sequencer();
        }

        self.last_div_bit_4 = div_bit_4;

        // The frequency timers of all four channels advance once per T-cycle.
        for _ in 0..4 {
            self.ch1.tick();
            self.ch2.tick();
            self.ch3.tick();
            self.ch4.tick();
        }

        self.integrate_mix();
    }

    /// Advances the frame sequencer one step. Length counters are clocked on
    /// steps 0, 2, 4 and 6, the sweep unit on steps 2 and 6 and the envelopes
    /// on step 7.
    fn tick_frame_sequencer(&mut self) {
        match self.frame_sequencer_step {
            0 | 4 => self.clock_length_counters(),
            2 | 6 => {
                self.clock_length_counters();
                self.ch1.clock_sweep();
            }
            7 => self.clock_envelopes(),
            _ => {}
        }

        self.frame_sequencer_step = (self.frame_sequencer_step + 1) & 0x07;
    }

    fn clock_length_counters(&mut self) {
        self.ch1.clock_length();
        self.ch2.clock_length();
        self.ch3.clock_length();
        self.ch4.clock_length();
    }

    fn clock_envelopes(&mut self) {
        self.ch1.clock_envelope();
        self.ch2.clock_envelope();
        self.ch4.clock_envelope();
    }

    /// Integrates the mixer output over the elapsed machine cycle and emits
    /// a sample whenever the fixed point accumulator crosses one sample
    /// period, yielding `SAMPLE_RATE` samples per second on average.
    fn integrate_mix(&mut self) {
        let (left, right) = self.mix();

        self.mixed_left_accumulator += i32::from(left) * T_CYCLES_PER_STEP as i32;
        self.mixed_right_accumulator += i32::from(right) * T_CYCLES_PER_STEP as i32;
        self.mixed_t_cycles += T_CYCLES_PER_STEP;

        self.sample_accumulator += SAMPLE_RATE * T_CYCLES_PER_STEP;

        while self.sample_accumulator >= CPU_FREQUENCY {
            self.sample_accumulator -= CPU_FREQUENCY;
            self.push_sample();
        }
    }

    /// Averages the mixed signal accumulated since the last sample and
    /// appends the stereo pair to the sample buffer.
    fn push_sample(&mut self) {
        let left = self.mixed_left_accumulator / self.mixed_t_cycles as i32;
        let right = self.mixed_right_accumulator / self.mixed_t_cycles as i32;

        self.mixed_left_accumulator = 0;
        self.mixed_right_accumulator = 0;
        self.mixed_t_cycles = 0;

        self.sample_buffer.push(left as i16);
        self.sample_buffer.push(right as i16);
    }

    /// Mixes the four channels into a stereo pair, applying the NR51 panning
    /// and the NR50 master volume.
    fn mix(&self) -> (i16, i16) {
        let channels = [
            (self.ch1.is_active(), self.ch1.output()),
            (self.ch2.is_active(), self.ch2.output()),
            (self.ch3.is_active(), self.ch3.output()),
            (self.ch4.is_active(), self.ch4.output()),
        ];

        let mut left = 0;
        let mut right = 0;

        for (index, &(active, output)) in channels.iter().enumerate() {
            // Disabled channels are silent; their DAC output is not modelled.
            if !active {
                continue;
            }

            let dac = i32::from(output) - DAC_CENTER;

            if self.nr51 & (1 << (4 + index)) > 0 {
                left += dac;
            }
            if self.nr51 & (1 << index) > 0 {
                right += dac;
            }
        }

        let left_volume = i32::from((self.nr50 >> 4) & 0x07) + 1;
        let right_volume = i32::from(self.nr50 & 0x07) + 1;

        (
            scale_sample(left * left_volume),
            scale_sample(right * right_volume),
        )
    }

    /// Hands the samples accumulated since the last frame boundary over to
    /// the front end, clearing the internal buffer.
    pub fn take_sample_buffer(&mut self) -> Vec<i16> {
        std::mem::take(&mut self.sample_buffer)
    }

    pub fn set_post_boot_rom_state(&mut self) {
        // Power the APU on first so the register writes below are accepted,
        // then apply the values the boot ROM leaves behind. The final table
        // entry also carries the NR52 power bit.
        self.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, POWER_MASK);

        for (addr, value) in POST_BOOT_REGISTER_VALUES {
            self.write(addr, value);
        }
    }

    fn read_global_ctrl(&self) -> u8 {
        let mut value = STATUS_MASK;

        if self.power_on {
            value |= POWER_MASK;
        }
        if self.ch1.is_active() {
            value |= 1 << 0;
        }
        if self.ch2.is_active() {
            value |= 1 << 1;
        }
        if self.ch3.is_active() {
            value |= 1 << 2;
        }
        if self.ch4.is_active() {
            value |= 1 << 3;
        }

        value
    }
}

fn scale_sample(value: i32) -> i16 {
    (value * OUTPUT_SCALE).clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16
}

#[path = "processor_test.rs"]
#[cfg(test)]
mod test;
