#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use crate::cpu::lr35902::LR35902;
use crate::cpu::{lr35902, register};

#[test]
fn reset_half_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= lr35902::HALF_CARRY_FLAG_MASK;

    cpu.reset_half_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_half_carry_flag() {
    let mut cpu = LR35902::new();

    cpu.set_half_carry_flag();
    assert_eq!(cpu.af.lo, lr35902::HALF_CARRY_FLAG_MASK);
}

#[test]
fn reset_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= lr35902::CARRY_FLAG_MASK;

    cpu.reset_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_carry_flag() {
    let mut cpu = LR35902::new();

    cpu.set_carry_flag();
    assert_eq!(cpu.af.lo, lr35902::CARRY_FLAG_MASK);
}

#[test]
fn reset_sub_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= lr35902::SUB_FLAG_MASK;

    cpu.reset_sub_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_sub_flag() {
    let mut cpu = LR35902::new();

    cpu.set_sub_flag();
    assert_eq!(cpu.af.lo, lr35902::SUB_FLAG_MASK);
}

#[test]
fn reset_zero_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= lr35902::ZERO_FLAG_MASK;

    cpu.reset_zero_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_zero_flag() {
    let mut cpu = LR35902::new();

    cpu.set_zero_flag();
    assert_eq!(cpu.af.lo, lr35902::ZERO_FLAG_MASK);
}

#[test]
fn test_half_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.set_half_carry_flag();
    assert_eq!(cpu.test_half_carry_flag(), true);
}

#[test]
fn test_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.set_carry_flag();
    assert_eq!(cpu.test_carry_flag(), true);
}

#[test]
fn test_sub_flag() {
    let mut cpu = LR35902::new();
    cpu.set_sub_flag();
    assert_eq!(cpu.test_sub_flag(), true);
}

#[test]
fn test_zero_flag() {
    let mut cpu = LR35902::new();
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
                let mut cpu = LR35902::new();
                cpu.bc.set_word(0x1000);
                cpu.de.set_word(0x000F);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.set_word(0x100F);
                cpu.de.set_word(0x000F);
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.set_word(0x0FFF);
                cpu.de.set_word(0x0001);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.set_word(0x1000);
                cpu.de.set_word(0x0001);
                cpu.set_half_carry_flag();
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.set_word(0x0FFF);
                cpu.de.set_word(0xF001);
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
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

#[test]
fn increment_8_bit_register() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        reg_id: register::ID,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("increment A"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                return cpu;
            },
            reg_id: register::ID::A,
        },
        TestCase {
            description: String::from("increment B"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                return cpu;
            },
            reg_id: register::ID::B,
        },
        TestCase {
            description: String::from("increment C"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                return cpu;
            },
            reg_id: register::ID::C,
        },
        TestCase {
            description: String::from("increment D"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                return cpu;
            },
            reg_id: register::ID::D,
        },
        TestCase {
            description: String::from("increment E"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                return cpu;
            },
            reg_id: register::ID::E,
        },
        TestCase {
            description: String::from("increment H"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                return cpu;
            },
            reg_id: register::ID::H,
        },
        TestCase {
            description: String::from("increment L"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("resets the sub flag"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("sets the zero flag and half carry"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("only sets the half carry"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x0F;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x10;
                cpu.set_half_carry_flag();
                return cpu;
            },
            reg_id: register::ID::L,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description.as_str());
        let mut cpu = (tc.initial_state)();
        cpu.increment_8_bit_register(tc.reg_id);
        assert_eq!(cpu, (tc.expected_state)());
    }
}

#[test]
fn decrement_8_bit_register() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        reg_id: register::ID,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("decrement A"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            reg_id: register::ID::A,
        },
        TestCase {
            description: String::from("decrement B"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            reg_id: register::ID::B,
        },
        TestCase {
            description: String::from("decrement C"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                cpu.bc.lo = 0xFF;
                return cpu;
            },
            reg_id: register::ID::C,
        },
        TestCase {
            description: String::from("decrement D"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.de.hi = 0xFF;
                return cpu;
            },
            reg_id: register::ID::D,
        },
        TestCase {
            description: String::from("decrement E"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.de.lo = 0xFF;
                return cpu;
            },
            reg_id: register::ID::E,
        },
        TestCase {
            description: String::from("decrement H"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.hl.hi = 0xFF;
                return cpu;
            },
            reg_id: register::ID::H,
        },
        TestCase {
            description: String::from("decrement L"),
            initial_state: || -> LR35902 {
                let cpu = LR35902::new();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.hl.lo = 0xFF;
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("sets the sub flag"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                cpu.hl.lo = 0x01;
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("sets the zero flag"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            reg_id: register::ID::L,
        },
        TestCase {
            description: String::from("sets the half carry"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x10;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x0F;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            reg_id: register::ID::L,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description.as_str());
        let mut cpu = (tc.initial_state)();
        cpu.decrement_8_bit_register(tc.reg_id);
        assert_eq!(cpu, (tc.expected_state)());
    }
}
