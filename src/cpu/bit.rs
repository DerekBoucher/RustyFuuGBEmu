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
