use crate::apu::Apu;
use crate::memory::io_registers;

fn powered_on_apu() -> Apu {
    let mut apu = Apu::new();
    apu.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0x80);

    apu
}

/// Advances the APU `m_cycles` machine cycles while tracking a fake system
/// clock, so that the DIV driven frame sequencer sees evenly spaced falling
/// edges.
fn advance(apu: &mut Apu, clock: &mut u16, m_cycles: u32) {
    for _ in 0..m_cycles {
        *clock = clock.wrapping_add(4);
        apu.step(*clock);
    }
}

#[test]
fn channel_writes_are_ignored_while_powered_off() {
    let mut apu = Apu::new();

    apu.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    apu.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x80);

    // Powering on afterwards must not expose the ignored writes.
    apu.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0x80);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF0);
}

#[test]
fn powering_off_resets_channels_and_keeps_wave_ram() {
    let mut apu = powered_on_apu();

    apu.write(0xFF30, 0xA5);
    apu.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    apu.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0x80);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF1);

    apu.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0x00);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0x70);
    assert_eq!(apu.read(0xFF30), 0xA5);

    // The channel state was cleared, so powering on leaves channel 1 off.
    apu.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0x80);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF0);
}

#[test]
fn frame_sequencer_clocks_length_counters_at_512_hz() {
    let mut apu = powered_on_apu();

    // Channel 1 with an enabled length counter of 1.
    apu.write(io_registers::AUDIO_CH1_VOLUME_ADDR, 0xF0);
    apu.write(io_registers::AUDIO_CH1_LENGTH_ADDR, 0x3F);
    apu.write(io_registers::AUDIO_CH1_WAV_HI_ADDR, 0xC0);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF1);

    // One frame sequencer period is 8192 T-cycles: 2047 machine cycles
    // leave the DIV bit 4 falling edge one step away.
    let mut clock: u16 = 0;
    advance(&mut apu, &mut clock, 2047);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF1);

    // The falling edge lands on step 0, clocking the length counter, which
    // expires and disables the channel.
    advance(&mut apu, &mut clock, 1);
    assert_eq!(apu.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR), 0xF0);
}
