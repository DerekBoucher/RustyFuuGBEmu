#[path = "bit_test.rs"]
#[cfg(test)]
mod test;

pub fn is_half_carry(byte: u8, added_byte: u8, carry: bool) -> bool {
    let a: u32 = (byte & 0x0F).into();
    let b: u32 = (added_byte & 0x0F).into();
    let c: u32 = match carry {
        true => 0x01,
        false => 0x00,
    };

    if a + b + c > 0x0F {
        return true;
    }

    return false;
}

pub fn is_carry(byte: u8, added_byte: u8, with_carry: bool) -> bool {
    let a: u32 = byte.into();
    let b: u32 = added_byte.into();
    let c: u32 = match with_carry {
        true => 0x01,
        false => 0x00,
    };

    if a + b + c > 0xFF {
        return true;
    }

    return false;
}

pub fn is_half_borrow(byte: u8, subtracted_byte: u8, carry: bool) -> bool {
    let a: u32 = (byte & 0x0F).into();
    let b: u32 = (subtracted_byte & 0x0F).into();
    let c: u32 = match carry {
        true => 0x01,
        false => 0x00,
    };

    if a < (b + c) {
        return true;
    }

    return false;
}

pub fn is_half_carry_word(word: u16, added_word: u16, mask: u16, carry: bool) -> bool {
    let a: u32 = (word & mask).into();
    let b: u32 = (added_word & mask).into();
    let c: u32 = match carry {
        true => 0x1,
        false => 0x0,
    };

    if a + b + c > u32::from(mask) {
        return true;
    }

    return false;
}

pub fn two_compliment_byte(byte: u8) -> u8 {
    return (!byte).wrapping_add(1);
}

pub fn test_most_significant_bit(byte: u8) -> bool {
    return byte & (1 << 7) > 0;
}

pub fn is_borrow(byte: u8, subtracted_byte: u8, with_carry: bool) -> bool {
    let a: u32 = byte.into();
    let b: u32 = subtracted_byte.into();
    let c: u32 = match with_carry {
        true => 0x01,
        false => 0x00,
    };

    if a < (b + c) {
        return true;
    }

    return false;
}
