use crate::cpu::LR35902;
use crate::memory::Memory;
use crate::ppu::Ppu;

pub struct Gameboy {
    cpu: LR35902,
    memory: Memory,
    ppu: Ppu,

    running: bool,
    require_render: bool,
}

impl Gameboy {}
