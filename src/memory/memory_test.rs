use std::sync::{Arc, Mutex};

use crate::apu::Apu;
use crate::interrupt;
use crate::memory::{io_registers, Memory};
use crate::timers::Timers;

fn new_memory_with_apu() -> (Memory, Arc<Mutex<Apu>>) {
    let apu = Arc::new(Mutex::new(Apu::new()));
    let memory = Memory::default(
        Arc::new(Mutex::new(Timers::new())),
        apu.clone(),
        Arc::new(Mutex::new(interrupt::Bus::new())),
    );

    (memory, apu)
}

#[test]
fn audio_register_writes_reach_the_apu() {
    let (mut memory, apu) = new_memory_with_apu();

    memory.write(io_registers::AUDIO_GLOBAL_CTRL_ADDR, 0x80);
    let read_back = apu
        .lock()
        .unwrap()
        .read(io_registers::AUDIO_GLOBAL_CTRL_ADDR);
    assert_eq!(read_back, 0x80);

    memory.write(0xFF30, 0xAB);
    assert_eq!(apu.lock().unwrap().read(0xFF30), 0xAB);
}

#[test]
fn audio_register_reads_come_from_the_apu() {
    let (mut memory, apu) = new_memory_with_apu();

    apu.lock().unwrap().write(0xFF30, 0x5A);
    assert_eq!(memory.read(0xFF30), Some(0x5A));
    assert_eq!(memory.dma_read(0xFF30), Some(0x5A));
}

#[test]
fn audio_registers_are_set_post_boot() {
    let (mut memory, _) = new_memory_with_apu();

    memory.set_post_boot_rom_state();

    assert_eq!(
        memory.read(io_registers::AUDIO_GLOBAL_CTRL_ADDR),
        Some(0xF1)
    );
    assert_eq!(
        memory.read(io_registers::AUDIO_GLOBAL_VOLUME_ADDR),
        Some(0x77)
    );
    assert_eq!(memory.read(io_registers::AUDIO_CH1_SWEEP_ADDR), Some(0x80));
}
