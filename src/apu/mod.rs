mod processor;

use crate::memory::io_registers;

/// Base address of the APU register block (NR10, the channel 1 sweep register).
const AUDIO_REGISTER_BASE: usize = io_registers::AUDIO_CH1_SWEEP_ADDR;

/// Address of the final APU register (the last byte of wave RAM).
const AUDIO_REGISTER_END: usize = io_registers::AUDIO_WAV_PATTERN_RAM_END_ADDR;

/// Number of registers making up the APU register block (0xFF10 - 0xFF3F).
const AUDIO_REGISTER_COUNT: usize = AUDIO_REGISTER_END - AUDIO_REGISTER_BASE + 1;

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
    /// Raw storage backing the audio registers, wave RAM included
    /// (0xFF30 - 0xFF3F).
    registers: [u8; AUDIO_REGISTER_COUNT],

    /// Stereo samples accumulated since the last frame boundary. Samples are
    /// interleaved left/right 16-bit signed PCM values.
    sample_buffer: Vec<i16>,
}
