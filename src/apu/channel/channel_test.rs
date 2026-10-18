use super::{noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};
use crate::memory::io_registers;

#[test]
fn square_follows_the_configured_duty_pattern() {
    let mut channel = SquareChannel::new();

    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH1_LENGTH_ADDR, 0x80); // 50% duty
    channel.write(io_registers::AUDIO_CH1_WAV_LO_ADDR, 0xFF);
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x87); // frequency 0x7FF, trigger

    assert!(channel.is_active());

    let mut outputs = Vec::new();
    for _ in 0..8 {
        outputs.push(channel.output());

        // Frequency 0x7FF gives a period of 4 T-cycles.
        for _ in 0..4 {
            channel.tick();
        }
    }

    assert_eq!(outputs, [0, 0, 0, 0, 15, 15, 15, 15]);
}

#[test]
fn dac_off_disables_the_square_channel() {
    let mut channel = SquareChannel::new();

    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x80);
    assert!(channel.is_active());

    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0x00);
    assert!(!channel.is_active());
    assert_eq!(channel.output(), 0);
}

#[test]
fn envelope_clocks_are_applied_to_the_volume() {
    let mut channel = SquareChannel::new();

    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0x59); // volume 5, add, period 1
    channel.write(io_registers::AUDIO_CH1_LENGTH_ADDR, 0xC0); // 75% duty
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x80);

    // The first duty step of the 75% pattern is high.
    assert_eq!(channel.output(), 5);

    channel.clock_envelope();
    assert_eq!(channel.output(), 6);

    channel.clock_envelope();
    assert_eq!(channel.output(), 7);
}

#[test]
fn sweep_overflow_disables_the_channel_on_trigger() {
    let mut channel = SquareChannel::new_with_sweep();

    channel.write(io_registers::AUDIO_CH1_SWEEP_ADDR, 0x01); // add, shift 1
    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH1_WAV_LO_ADDR, 0xFF);
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x87); // frequency 0x7FF, trigger

    // 0x7FF + (0x7FF >> 1) overflows 11 bits, disabling the channel.
    assert!(!channel.is_active());
}

#[test]
fn sweep_clocks_can_overflow_the_frequency() {
    let mut channel = SquareChannel::new_with_sweep();

    channel.write(io_registers::AUDIO_CH1_SWEEP_ADDR, 0x11); // period 1, add, shift 1
    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH1_WAV_LO_ADDR, 0x00);
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x84); // frequency 0x400, trigger

    // The trigger calculation moves the frequency from 1024 to 1536.
    assert!(channel.is_active());

    // One sweep clock doubles it again: 1536 + 768 overflows and disables.
    channel.clock_sweep();
    assert!(!channel.is_active());
}

#[test]
fn length_expiry_disables_the_channel() {
    let mut channel = SquareChannel::new();

    channel.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH1_LENGTH_ADDR, 0x3F); // length 1
    channel.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0xC0); // length enable, trigger
    assert!(channel.is_active());

    channel.clock_length();
    assert!(!channel.is_active());
    assert_eq!(channel.output(), 0);
}

#[test]
fn noise_lfsr_drives_the_output() {
    let mut channel = NoiseChannel::new();

    channel.write(io_registers::AUDIO_CH4_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH4_FREQ_ADDR, 0x00); // divisor 8
    channel.write(io_registers::AUDIO_CH4_CTRL_ADDR, 0x80);
    assert!(channel.is_active());
    assert_eq!(channel.output(), 0);

    // The output stays low until bit 0 of the shift register turns 0, which
    // first happens after 15 shifts of 8 T-cycles each.
    for _ in 0..14 * 8 {
        channel.tick();
    }
    assert_eq!(channel.output(), 0);

    for _ in 0..8 {
        channel.tick();
    }
    assert_eq!(channel.output(), 15);
}

#[test]
fn noise_seven_bit_mode_has_a_short_period() {
    let mut channel = NoiseChannel::new();

    channel.write(io_registers::AUDIO_CH4_VOLUME_ADDR, 0xF0);
    channel.write(io_registers::AUDIO_CH4_FREQ_ADDR, 0x08); // 7-bit mode, divisor 8
    channel.write(io_registers::AUDIO_CH4_CTRL_ADDR, 0x80);

    let mut outputs = Vec::new();
    for _ in 0..(127 * 2) {
        outputs.push(channel.output());

        for _ in 0..8 {
            channel.tick();
        }
    }

    // The 7-bit LFSR has a period of 127 shifts.
    assert_eq!(outputs[..127], outputs[127..]);
}

#[test]
fn wave_channel_plays_the_high_nibble_first() {
    let mut channel = WaveChannel::new();

    channel.write_wave_ram(0xFF30, 0x1F);
    channel.write(io_registers::AUDIO_CH3_DAC_ENABLE_ADDR, 0x80);
    channel.write(io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x20); // 100%
    channel.write(io_registers::AUDIO_CH3_WAV_HI_ADDR, 0x80); // frequency 0, trigger

    assert_eq!(channel.output(), 1);

    // Frequency 0 gives a period of 4096 T-cycles.
    for _ in 0..4096 {
        channel.tick();
    }

    assert_eq!(channel.output(), 15);
}

#[test]
fn wave_output_level_follows_the_volume_code() {
    let mut channel = WaveChannel::new();

    channel.write_wave_ram(0xFF30, 0xFF);
    channel.write(io_registers::AUDIO_CH3_DAC_ENABLE_ADDR, 0x80);
    channel.write(io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x20);
    channel.write(io_registers::AUDIO_CH3_WAV_HI_ADDR, 0x80);
    assert_eq!(channel.output(), 15);

    channel.write(io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x40);
    assert_eq!(channel.output(), 7);

    channel.write(io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x60);
    assert_eq!(channel.output(), 3);

    channel.write(io_registers::AUDIO_CH3_OUTPUT_LVL_ADDR, 0x00);
    assert_eq!(channel.output(), 0);
}

#[test]
fn dac_off_disables_the_wave_channel() {
    let mut channel = WaveChannel::new();

    channel.write(io_registers::AUDIO_CH3_DAC_ENABLE_ADDR, 0x80);
    channel.write(io_registers::AUDIO_CH3_WAV_HI_ADDR, 0x80);
    assert!(channel.is_active());

    channel.write(io_registers::AUDIO_CH3_DAC_ENABLE_ADDR, 0x00);
    assert!(!channel.is_active());
    assert_eq!(channel.output(), 0);
}
