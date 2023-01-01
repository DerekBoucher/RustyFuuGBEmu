//! Module containing all logic relevant to the emulation of the
//! original Gameboy's random access and read only memory
pub mod memory;

/// Struct emulating the DMG Gameboy's memory behaviour.
/// This struct controls the access behaviour whenever the CPU
/// makes reads or writes to the memory.
pub struct Memory {
    rom: [u8; 0x10000],
}
