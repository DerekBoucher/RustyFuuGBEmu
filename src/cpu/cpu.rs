const ZERO_FLAG_MASK: u8 = 1 << 7;
const SUB_FLAG_MASK: u8 = 1 << 6;
const HALF_CARRY_FLAG_MASK: u8 = 1 << 5;
const CARRY_FLAG_MASK: u8 = 1 << 4;

union Register {
    hi: u8,
    lo: u8,
    data: u16,
}

pub struct LR35902 {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,

    sp: u16,
    pc: u16,
}

impl LR35902 {}
