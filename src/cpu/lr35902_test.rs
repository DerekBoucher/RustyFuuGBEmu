#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use crate::cpu::lr35902::LR35902;
use crate::cpu::{lr35902, register};

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

#[test]
fn test_half_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.set_half_carry_flag();
    assert_eq!(cpu.test_half_carry_flag(), true);
}

#[test]
fn test_carry_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.set_carry_flag();
    assert_eq!(cpu.test_carry_flag(), true);
}

#[test]
fn test_sub_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.set_sub_flag();
    assert_eq!(cpu.test_sub_flag(), true);
}

#[test]
fn test_zero_flag() {
    let mut cpu = LR35902::new(mock::Memory::new(vec![]));
    cpu.set_zero_flag();
    assert_eq!(cpu.test_zero_flag(), true);
}

#[test]
fn add_16_bit_registers() {
    struct TestCase {
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x1000);
                cpu.de.set_word(0x000F);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x100F);
                cpu.de.set_word(0x000F);
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x0FFF);
                cpu.de.set_word(0x0001);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x1000);
                cpu.de.set_word(0x0001);
                cpu.set_half_carry_flag();
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x0FFF);
                cpu.de.set_word(0xF001);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![]));
                cpu.bc.set_word(0x0000);
                cpu.de.set_word(0xF001);
                cpu.set_half_carry_flag();
                cpu.set_carry_flag();
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        cpu.add_16_bit_registers(register::ID16::BC, register::ID16::DE);
        assert_eq!(cpu, (tc.expected_state)());
    }
}
