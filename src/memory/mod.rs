//! Module containing all logic relevant to the emulation of the
//! original Gameboy's random access and read only memory
pub mod memory;

/// Struct emulating the DMG Gameboy's memory behaviour.
/// This struct controls the access behaviour whenever the CPU
/// makes reads or writes to the memory.
pub struct Memory {
    /// cartridge memory is mapped from address 0x0000 - 0x8000.
    /// Furthermore, cartridge memory is segmented in 4KB banks,
    /// where bank 0 is always mapped from 0x0000 - 0x3FFF, and
    /// bank n is mapped from address 0x4000 - 0x7FFF.
    cartridge: Vec<u8>,

    /// Video RAM where tile data is located.
    /// Occupies memory locations 0x8000 ~ 0x9FFF.
    video_ram: [u8; 0x2000],

    /// External RAM contained inside the cartridge.
    /// Mapped to memory locations 0xA000 ~ 0xBFFF.
    external_ram: [u8; 0x2000],

    /// General purpose RAM bank 0.
    /// Occupies memory locations 0xC000 ~ 0xCFFF.
    work_ram0: [u8; 0x1000],

    /// General purpose RAM bank 1.
    /// Occupies memory locations 0xD000 ~ 0xDFFF.
    work_ram1: [u8; 0x1000],

    /// Exact replica of contents of memory from 0xC000 ~ 0xDDFF.
    /// Not quite sure what the use of this is.
    /// Occupies memory locations 0xE000 ~ 0xFDFF.
    echo_ram: [u8; 0x1E00],

    /// Sprite data.
    /// Occupies memory locations 0xFE00 ~ 0xFE9F.
    sprite_attributes: [u8; 0xA0],

    /// IO Registers such as interupts, controls, etc...
    /// Occupies memory locations 0xFF00 ~ 0xFF7F.
    io_registers: [u8; 0x80],

    /// High RAM used by the CPU.
    /// Occupies memory locations 0xFF80 ~ 0xFFFE.
    hi_ram: [u8; 0x7E],

    /// Master interrupt enable register.
    /// Occupies a single byte of memory at location 0xFFFF.
    interrupt_enable_register: u8,
}
