#[path = "opcode_test.rs"]
#[cfg(test)]
mod test;

use crate::cpu::bit;
use crate::cpu::LR35902;

pub struct Nop;
impl Nop {
    pub const OPCODE: u8 = 0x00;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        4
    }
}

pub struct LdImm16IntoBC;
impl LdImm16IntoBC {
    pub const OPCODE: u8 = 0x01;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        cpu.bc.lo = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 16 into BC failed to fetch lo byte. Dumping cpu state...\n{:?}",
                cpu
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        cpu.bc.hi = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 16 into BC failed to fetch hi byte. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        12
    }
}

pub struct LdAIntoMemoryBC;
impl LdAIntoMemoryBC {
    pub const OPCODE: u8 = 0x02;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.memory.write(usize::from(cpu.bc.word()), cpu.af.hi);
        cpu.pc = cpu.pc.wrapping_add(1);
        8
    }
}

pub struct IncBC;
impl IncBC {
    pub const OPCODE: u8 = 0x03;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let mut word = cpu.bc.word();
        word = word.wrapping_add(1);

        cpu.bc.hi = word.to_be_bytes()[0];
        cpu.bc.lo = word.to_be_bytes()[1];

        8
    }
}

pub struct IncB;
impl IncB {
    pub const OPCODE: u8 = 0x04;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        if bit::is_half_carry(cpu.bc.hi, 0x01, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        cpu.bc.hi = cpu.bc.hi.wrapping_add(1);

        if cpu.bc.hi == 0x00 {
            cpu.set_zero_flag();
        } else {
            cpu.reset_zero_flag();
        }

        cpu.reset_sub_flag();

        4
    }
}

pub struct DecB;
impl DecB {
    pub const OPCODE: u8 = 0x05;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        if bit::is_half_borrow(cpu.bc.hi, 0x1, false) {
            cpu.set_half_carry_flag();
        } else {
            cpu.reset_half_carry_flag();
        }

        cpu.bc.hi = cpu.bc.hi.wrapping_sub(1);

        if cpu.bc.hi == 0x00 {
            cpu.set_zero_flag();
        } else {
            cpu.reset_zero_flag();
        }

        cpu.set_sub_flag();

        4
    }
}

pub struct LdImm8IntoB;
impl LdImm8IntoB {
    pub const OPCODE: u8 = 0x06;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let byte = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode load imm 8 into B failed to fetch byte in memory. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.bc.hi = byte;

        cpu.pc = cpu.pc.wrapping_add(1);

        8
    }
}

pub struct RotateLeftCarryIntoA;
impl RotateLeftCarryIntoA {
    pub const OPCODE: u8 = 0x07;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let leftmost_bit_a: bool = (cpu.af.hi & (1 << 7)) > 0;

        if leftmost_bit_a {
            cpu.set_carry_flag();
        } else {
            cpu.reset_carry_flag();
        }

        cpu.af.hi = cpu.af.hi.rotate_left(1);

        cpu.reset_half_carry_flag();
        cpu.reset_sub_flag();
        cpu.reset_zero_flag();

        4
    }
}

pub struct LdSpInto16ImmAddress;
impl LdSpInto16ImmAddress {
    pub const OPCODE: u8 = 0x08;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let lo_address_byte = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode 0x08 failed to load lo address byte from memory. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        let hi_address_byte = match cpu.memory.read(usize::from(cpu.pc)) {
            Some(byte) => *byte,
            None => panic!(
                "opcode 0x08 failed to load hi address byte from memory. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.pc = cpu.pc.wrapping_add(1);

        let mut addr = usize::from(u16::from(hi_address_byte) << 8 | u16::from(lo_address_byte));

        cpu.memory.write(addr, cpu.sp.to_be_bytes()[1]);
        addr += 1;
        cpu.memory.write(addr, cpu.sp.to_be_bytes()[0]);

        20
    }
}

pub struct AddBCintoHL;
impl AddBCintoHL {
    pub const OPCODE: u8 = 0x09;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.reset_sub_flag();

        cpu.pc = cpu.pc.wrapping_add(1);

        let new_hl = cpu.add_16_bit_registers(cpu.bc.word(), cpu.hl.word());

        cpu.hl.set_word(new_hl);

        8
    }
}

pub struct LdMemoryBCIntoA;
impl LdMemoryBCIntoA {
    pub const OPCODE: u8 = 0x0A;

    pub fn execute(cpu: &mut LR35902) -> u32 {
        cpu.pc = cpu.pc.wrapping_add(1);

        let value = match cpu.memory.read(usize::from(cpu.bc.word())) {
            Some(byte) => *byte,
            None => panic!(
                "opcode 0x0A failed to load byte from memory pointed to by BC. Dumping cpu state...\n{:?}",
                cpu,
            ),
        };

        cpu.af.hi = value;

        8
    }
}
