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
        word: u16,
        added_word: u16,
        initial_state: fn() -> LR35902,
        assert_fn: fn(u16, &LR35902),
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            word: 0x1000,
            added_word: 0x000F,
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![0x07]));
                cpu.set_sub_flag();
                return cpu;
            },
            assert_fn: |result, cpu| {
                assert_eq!(result, 0x100F);
                assert_eq!(cpu.test_sub_flag(), false);
                assert_eq!(cpu.test_half_carry_flag(), false);
                assert_eq!(cpu.test_carry_flag(), false);
            },
        },
        TestCase {
            word: 0x0FFF,
            added_word: 0x0001,
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![0x07]));
                cpu.set_sub_flag();
                return cpu;
            },
            assert_fn: |result, cpu| {
                assert_eq!(result, 0x1000);
                assert_eq!(cpu.test_sub_flag(), false);
                assert_eq!(cpu.test_half_carry_flag(), true);
                assert_eq!(cpu.test_carry_flag(), false);
            },
        },
        TestCase {
            word: 0x0FFF,
            added_word: 0xF001,
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![0x07]));
                cpu.set_sub_flag();
                return cpu;
            },
            assert_fn: |result, cpu| {
                assert_eq!(result, 0x0);
                assert_eq!(cpu.test_sub_flag(), false);
                assert_eq!(cpu.test_half_carry_flag(), true);
                assert_eq!(cpu.test_carry_flag(), true);
            },
        },
    ];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        let result = cpu.add_16_bit_registers(tc.word, tc.added_word);
        (tc.assert_fn)(result, &cpu);
    }
}
