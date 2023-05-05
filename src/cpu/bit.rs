#[path = "bit_test.rs"]
#[cfg(test)]
mod test;

pub fn is_half_carry(byte: u8, added_byte: u8, carry: bool) -> bool {
    let a = byte & 0x0F;
    let b = added_byte & 0x0F;
    let c = match carry {
        true => 0x01,
        false => 0x00,
    };

    if a + b + c > 0x0F {
        return true;
    }

    return false;
}

pub fn is_half_borrow(byte: u8, subtracted_byte: u8, carry: bool) -> bool {
    let a = byte & 0x0F;
    let b = subtracted_byte & 0x0F;
    let c = match carry {
        true => 0x01,
        false => 0x00,
    };

    if a < (b + c) {
        return true;
    }

    return false;
}

pub fn is_half_carry_word(word: u16, added_word: u16, mask: u16, carry: bool) -> bool {
    let a = word & mask;
    let b = added_word & mask;
    let c = match carry {
        true => 0x1,
        false => 0x0,
    };

    if a + b + c > mask {
        return true;
    }

    return false;
}

pub fn two_compliment_byte(byte: u8) -> u8 {
    return (!byte) + 0x01;
}

pub fn test_most_significant_bit(byte: u8) -> bool {
    return byte & (1 << 7) > 0;
}
