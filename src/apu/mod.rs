mod channel;
mod processor;

use channel::{noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};

/// Audio Processing Unit.
///
/// Owns the audio related IO registers (0xFF10 - 0xFF3F) and produces the
/// stereo PCM sample batches consumed by the front end. `Memory` routes all
/// accesses in the register range to this component, the same way timer
/// register accesses are routed to `Timers`, while the component itself is
/// stepped from the same loop as the timers and the PPU so that it stays in
/// sync with the rest of the system.
#[derive(Debug)]
pub struct Apu {
    /// Channel 1: square wave with a frequency sweep unit.
    ch1: SquareChannel,

    /// Channel 2: square wave.
    ch2: SquareChannel,

    /// Channel 3: arbitrary waveform backed by wave RAM.
    ch3: WaveChannel,

    /// Channel 4: noise.
    ch4: NoiseChannel,

    /// NR50 master volume. Consumed by the mixer in a later phase.
    #[allow(dead_code)]
    nr50: u8,

    /// NR51 panning. Consumed by the mixer in a later phase.
    #[allow(dead_code)]
    nr51: u8,

    /// NR52 power control.
    power_on: bool,

    /// Current step (0 - 7) of the 512 Hz frame sequencer.
    frame_sequencer_step: u8,

    /// Previous sample of DIV bit 4 (system clock bit 12). The frame
    /// sequencer advances on the falling edge of this signal.
    last_div_bit_4: bool,

    /// Fixed point accumulator measuring the T-cycles elapsed since the last
    /// emitted sample, counting in units of `SAMPLE_RATE` per `CPU_FREQUENCY`.
    sample_accumulator: u32,

    /// Summed mixer output since the last emitted sample, and the number of
    /// T-cycles that sum covers.
    mixed_left_accumulator: i32,
    mixed_right_accumulator: i32,
    mixed_t_cycles: u32,

    /// Stereo samples accumulated since the last frame boundary. Samples are
    /// interleaved left/right 16-bit signed PCM values.
    sample_buffer: Vec<i16>,
}
