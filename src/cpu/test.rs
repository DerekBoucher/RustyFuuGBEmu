use crate::cpu;
use crate::cpu::register;
use crate::cpu::LR35902;

use crate::interface::mock;

pub struct TestCase {
    pub initial_state: fn() -> (LR35902, mock::Memory),
    pub expected_state: fn() -> (LR35902, mock::Memory),
    pub expected_cycles: u32,
    pub disable_pc_check: bool,
}

impl TestCase {
    pub fn run(&self, i: usize) {
        println!("subtest #{} results:", i);
        println!("---------------------");

        let initial_state = (self.initial_state)();
        let mut cpu_initial_state = initial_state.0;
        let mut memory_initial_state = initial_state.1;

        let expected_state = (self.expected_state)();
        let cpu_expected_state = expected_state.0;
        let memory_expected_state = expected_state.1;

        let initial_pc = cpu_initial_state.pc;

        let actual_cycles = cpu_initial_state.execute_next_opcode(&mut memory_initial_state);
        assert_eq!(actual_cycles, self.expected_cycles);
        assert_eq!(cpu_initial_state, cpu_expected_state);
        assert_eq!(memory_initial_state, memory_expected_state);

        if !self.disable_pc_check {
            assert_ne!(
                cpu_initial_state.pc, 0x0000,
                "pc should never be 0 after an opcode"
            );
            assert_ne!(
                initial_pc, cpu_initial_state.pc,
                "pc should have changed value after executing an op code"
            );
        }
    }
}

#[test]
fn reset_half_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= cpu::HALF_CARRY_FLAG_MASK;

    cpu.reset_half_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_half_carry_flag() {
    let mut cpu = LR35902::new();

    cpu.set_half_carry_flag();
    assert_eq!(cpu.af.lo, cpu::HALF_CARRY_FLAG_MASK);
}

#[test]
fn reset_carry_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= cpu::CARRY_FLAG_MASK;

    cpu.reset_carry_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_carry_flag() {
    let mut cpu = LR35902::new();

    cpu.set_carry_flag();
    assert_eq!(cpu.af.lo, cpu::CARRY_FLAG_MASK);
}

#[test]
fn reset_sub_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= cpu::SUB_FLAG_MASK;

    cpu.reset_sub_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_sub_flag() {
    let mut cpu = LR35902::new();

    cpu.set_sub_flag();
    assert_eq!(cpu.af.lo, cpu::SUB_FLAG_MASK);
}

#[test]
fn reset_zero_flag() {
    let mut cpu = LR35902::new();
    cpu.af.lo |= cpu::ZERO_FLAG_MASK;

    cpu.reset_zero_flag();
    assert_eq!(cpu.af.lo, 0);
}

#[test]
fn set_zero_flag() {
    let mut cpu = LR35902::new();

    cpu.set_zero_flag();
    assert_eq!(cpu.af.lo, cpu::ZERO_FLAG_MASK);
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

#[test]
fn add_8_bit_registers() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        src_reg: register::ID,
        with_carry_flag: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("when half carry occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x0F;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x10;
                cpu.set_half_carry_flag();
                cpu.bc.hi = 0x01;
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when carry occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0x02;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.bc.hi = 0x02;
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when carry occurs, resulting in a zero overflow"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                cpu.bc.hi = 0x01;
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when sub flag is set -> gets reset"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_zero_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("with carry flag, causes a carry"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_carry_flag();
                cpu.bc.hi = 0xFE;
                cpu.af.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0xFE;
                cpu.af.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: true,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.add_8_bit_registers(register::ID::A, tc.src_reg, tc.with_carry_flag);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn add_8_bit_memory() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
        with_carry_flag: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("when half carry occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x0F;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x10;
                cpu.set_half_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x01]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when carry occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x02]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when carry occurs and results in a 0 value"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x01]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when sub flag is set -> gets reset"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                return cpu;
            },
            memory: mock::Memory::new(vec![0x01]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("with carry flag, causes a carry"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_carry_flag();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x00]),
            addr: 0x0000,
            with_carry_flag: true,
        },
        TestCase {
            description: String::from(
                "with carry flag, but carry flag is 0, should not result in a carry",
            ),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            memory: mock::Memory::new(vec![0x00]),
            addr: 0x0000,
            with_carry_flag: true,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.add_8_bit_memory(register::ID::A, &tc.memory, tc.addr, tc.with_carry_flag);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn sub_8_bit_registers() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        src_reg: register::ID,
        with_carry_flag: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("when half borrow occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x10;
                cpu.bc.hi = 0x02;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x0E;
                cpu.bc.hi = 0x02;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when borrow occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0x01;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("results in a zero"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0xFF;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("with carry flag is set, causes a borrow"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_carry_flag();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.set_half_carry_flag();
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            src_reg: register::ID::B,
            with_carry_flag: true,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.sub_8_bit_registers(register::ID::A, tc.src_reg, tc.with_carry_flag);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn sub_8_bit_memory() {
    struct TestCase {
        description: String,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
        with_carry_flag: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from("when half borrow occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x10;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x0F]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when borrow occurs"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x10]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("when results in a zero value"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x10;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x10]),
            addr: 0x0000,
            with_carry_flag: false,
        },
        TestCase {
            description: String::from("with carry flag, causes a borrow"),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.set_carry_flag();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.set_half_carry_flag();
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x00]),
            addr: 0x0000,
            with_carry_flag: true,
        },
        TestCase {
            description: String::from(
                "with carry flag, but carry flag is 0, should not result in a carry",
            ),
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x00]),
            addr: 0x0000,
            with_carry_flag: true,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.sub_8_bit_memory(register::ID::A, &tc.memory, tc.addr, tc.with_carry_flag);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn and_8_bit_registers() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.bc.hi = 0x0F;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0x0F;
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                return cpu;
            },
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xF0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.bc.hi = 0xF0;
                cpu.set_half_carry_flag();
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.and_8_bit_registers(register::ID::A, register::ID::B);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn and_8_bit_memory() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x0F]),
            addr: 0x0000,
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.set_half_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0xF0]),
            addr: 0x0000,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.and_8_bit_memory(register::ID::A, &tc.memory, tc.addr);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn xor_8_bit_registers() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.bc.hi = 0x0F;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0x0F;
                return cpu;
            },
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xF0;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x0F;
                cpu.bc.hi = 0xF0;
                return cpu;
            },
        },
        TestCase {
            description: "zero flag case",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0xFF;
                cpu.set_zero_flag();
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.xor_8_bit_registers(register::ID::A, register::ID::B);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn xor_8_bit_memory() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            memory: mock::Memory::new(vec![0x0F]),
            addr: 0x0000,
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x0F;
                return cpu;
            },
            memory: mock::Memory::new(vec![0xF0]),
            addr: 0x0000,
        },
        TestCase {
            description: "zero flag case",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0xFF;
                cpu.set_zero_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0xFF]),
            addr: 0x0000,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.xor_8_bit_memory(register::ID::A, &tc.memory, tc.addr);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn or_8_bit_registers() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.bc.hi = 0x0F;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0x0F;
                return cpu;
            },
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xF0;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.hi = 0xF0;
                return cpu;
            },
        },
        TestCase {
            description: "zero flag case",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.bc.hi = 0x00;
                cpu.set_zero_flag();
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.or_8_bit_registers(register::ID::A, register::ID::B);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn or_8_bit_memory() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "ensure that hardcoded flag states are correct",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xF0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            memory: mock::Memory::new(vec![0x0F]),
            addr: 0x0000,
        },
        TestCase {
            description: "checking the and function works",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            memory: mock::Memory::new(vec![0xF0]),
            addr: 0x0000,
        },
        TestCase {
            description: "zero flag case",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_zero_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x00]),
            addr: 0x0000,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.or_8_bit_memory(register::ID::A, &tc.memory, tc.addr);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn compare_8_bit_registers() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        src: register::ID,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "when src value equals register A value",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.bc.lo = 0xFF;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            src: register::ID::C,
        },
        TestCase {
            description: "when src value triggers a half borrow",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFE;
                cpu.bc.lo = 0x0F;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFE;
                cpu.bc.lo = 0x0F;
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            src: register::ID::C,
        },
        TestCase {
            description: "when src value triggers a borrow",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x50;
                cpu.bc.lo = 0x60;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x50;
                cpu.bc.lo = 0x60;
                cpu.set_sub_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            src: register::ID::C,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.compare_8_bit_registers(register::ID::A, tc.src);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn compare_8_bit_memory() {
    struct TestCase<'a> {
        description: &'a str,
        initial_state: fn() -> LR35902,
        expected_state: fn() -> LR35902,
        memory: mock::Memory,
        addr: usize,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: "when src value equals register A value",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFF;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0xFF]),
            addr: 0x0000,
        },
        TestCase {
            description: "when src value triggers a half borrow",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFE;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFE;
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x0F]),
            addr: 0x0000,
        },
        TestCase {
            description: "when src value triggers a borrow",
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x50;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x50;
                cpu.set_sub_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            memory: mock::Memory::new(vec![0x60]),
            addr: 0x0000,
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let mut initial_state = (tc.initial_state)();
        let expected_state = (tc.expected_state)();

        initial_state.compare_8_bit_memory(register::ID::A, &tc.memory, tc.addr);

        assert_eq!(initial_state, expected_state)
    }
}

#[test]
fn push_16bit_register_on_stack() {
    {
        let mut cpu = LR35902::new();
        cpu.pc = 0x1FFF;
        cpu.sp = 0x0010;

        let mut memory = mock::Memory::new(vec![0x00; 0x10]);

        cpu.push_16bit_register_on_stack(register::ID16::PC, &mut memory);

        assert_eq!(cpu.sp, 0x000E);
        assert_eq!(memory.data[0x000F], 0x1F);
        assert_eq!(memory.data[0x000E], 0xFF);
    }
    {
        let mut cpu = LR35902::new();
        cpu.bc.set_word(0x1FFF);
        cpu.sp = 0x0010;

        let mut memory = mock::Memory::new(vec![0x00; 0x10]);

        cpu.push_16bit_register_on_stack(register::ID16::BC, &mut memory);

        assert_eq!(cpu.sp, 0x000E);
        assert_eq!(memory.data[0x000F], 0x1F);
        assert_eq!(memory.data[0x000E], 0xFF);
    }
    {
        let mut cpu = LR35902::new();
        cpu.de.set_word(0x1FFF);
        cpu.sp = 0x0010;

        let mut memory = mock::Memory::new(vec![0x00; 0x10]);

        cpu.push_16bit_register_on_stack(register::ID16::DE, &mut memory);

        assert_eq!(cpu.sp, 0x000E);
        assert_eq!(memory.data[0x000F], 0x1F);
        assert_eq!(memory.data[0x000E], 0xFF);
    }
    {
        let mut cpu = LR35902::new();
        cpu.hl.set_word(0x1FFF);
        cpu.sp = 0x0010;

        let mut memory = mock::Memory::new(vec![0x00; 0x10]);

        cpu.push_16bit_register_on_stack(register::ID16::HL, &mut memory);

        assert_eq!(cpu.sp, 0x000E);
        assert_eq!(memory.data[0x000F], 0x1F);
        assert_eq!(memory.data[0x000E], 0xFF);
    }
}

#[test]
fn process_interrupts() {
    struct TestCase {
        description: &'static str,
        setup: fn() -> (LR35902, mock::Memory),
        assert: fn(&LR35902, &mock::Memory),
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_ENABLE_REGISTER_ADDR] = cpu::V_BLANK_INTERRUPT_MASK;
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::V_BLANK_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, 0x0100);
                assert_eq!(cpu.sp, 0xFFFE);
                assert!(!cpu.interrupt_master_enable);
            },
            description:
                "interrupt master enable flag is disabled -> should not process any interrupts",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let data: Vec<u8> = vec![0x00; 0x10000];
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, 0x0100);
                assert_eq!(cpu.sp, 0xFFFE);
                assert!(cpu.interrupt_master_enable);
            },
            description: "no interrupt flags enabled -> should not process any interrupts",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::V_BLANK_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, 0x0100);
                assert_eq!(cpu.sp, 0xFFFE);
                assert!(cpu.interrupt_master_enable);
            },
            description: "v blank flag set but enable off -> should not process any interrupts",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_ENABLE_REGISTER_ADDR] = cpu::V_BLANK_INTERRUPT_MASK;
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::V_BLANK_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, cpu::V_BLANK_INTERRUPT_VECTOR);
                assert_eq!(cpu.sp, 0xFFFC);
                assert_eq!(memory.data[0xFFFE], 0x00);
                assert_eq!(memory.data[0xFFFD], 0x01);
                assert!(!cpu.interrupt_master_enable);
            },
            description: "v blank flag set and enable set -> should process v-blank interrupt",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::LCDC_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, 0x0100);
                assert_eq!(cpu.sp, 0xFFFE);
                assert!(cpu.interrupt_master_enable);
            },
            description: "LCDC flag set but enable off -> should not process any interrupts",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_ENABLE_REGISTER_ADDR] = cpu::LCDC_INTERRUPT_MASK;
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::LCDC_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, cpu::LCDC_INTERRUPT_VECTOR);
                assert_eq!(cpu.sp, 0xFFFC);
                assert_eq!(memory.data[0xFFFE], 0x00);
                assert_eq!(memory.data[0xFFFD], 0x01);
                assert!(!cpu.interrupt_master_enable);
            },
            description: "LCDC flag set and enable set -> should process LCDC interrupt",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::TIMER_OVERFLOW_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, 0x0100);
                assert_eq!(cpu.sp, 0xFFFE);
                assert!(cpu.interrupt_master_enable);
            },
            description: "timer flag set but enable off -> should not process any interrupts",
        },
        TestCase {
            setup: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.set_post_boot_rom_state();
                cpu.interrupt_master_enable = true;
                let mut data: Vec<u8> = vec![0x00; 0x10000];
                data[cpu::INTERRUPT_ENABLE_REGISTER_ADDR] = cpu::TIMER_OVERFLOW_INTERRUPT_MASK;
                data[cpu::INTERRUPT_FLAG_REGISTER_ADDR] = cpu::TIMER_OVERFLOW_INTERRUPT_MASK;
                let memory = mock::Memory::new(data);
                return (cpu, memory);
            },
            assert: |cpu, memory| {
                assert_eq!(cpu.pc, cpu::TIMER_OVERFLOW_INTERRUPT_VECTOR);
                assert_eq!(cpu.sp, 0xFFFC);
                assert_eq!(memory.data[0xFFFE], 0x00);
                assert_eq!(memory.data[0xFFFD], 0x01);
                assert!(!cpu.interrupt_master_enable);
            },
            description: "timer flag set and enable set -> should process timer overflow interrupt",
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        println!("{}", tc.description);
        let (mut cpu, mut memory) = (tc.setup)();
        cpu.process_interrupts(&mut memory);
        (tc.assert)(&cpu, &memory);
    }
}
