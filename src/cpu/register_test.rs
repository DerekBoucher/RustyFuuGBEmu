use crate::cpu::register::Register;

#[test]
fn new() {
    let register = Register::new();
    assert_eq!(register.hi, 0x00);
    assert_eq!(register.lo, 0x00);
}

#[test]
fn word() {
    let mut register = Register::new();
    register.hi = 0x7F;
    register.lo = 0xFF;
    assert_eq!(register.word(), 0x7FFF);
}
