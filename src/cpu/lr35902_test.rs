#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use crate::cpu::lr35902;
use crate::cpu::lr35902::LR35902;

#[test]
fn reset_half_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.af.lo |= lr35902::HALF_CARRY_FLAG_MASK;

    cpu.reset_half_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_half_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));

    cpu.set_half_carry_flag();
    assert_eq!(cpu.af.lo, lr35902::HALF_CARRY_FLAG_MASK);
}

#[test]
fn reset_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.af.lo |= lr35902::CARRY_FLAG_MASK;

    cpu.reset_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));

    cpu.set_carry_flag();
    assert_eq!(cpu.af.lo, lr35902::CARRY_FLAG_MASK);
}

#[test]
fn reset_sub_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.af.lo |= lr35902::SUB_FLAG_MASK;

    cpu.reset_sub_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_sub_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));

    cpu.set_sub_flag();
    assert_eq!(cpu.af.lo, lr35902::SUB_FLAG_MASK);
}

#[test]
fn reset_zero_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.af.lo |= lr35902::ZERO_FLAG_MASK;

    cpu.reset_zero_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_zero_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));

    cpu.set_zero_flag();
    assert_eq!(cpu.af.lo, lr35902::ZERO_FLAG_MASK);
}
