#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use std::vec;

use crate::cpu::*;

#[test]
fn _0x00() {
    let mut expected_state = LR35902::new(mock::Memory::new(vec![opcode::Nop::OPCODE]));
    expected_state.pc = 0x1;

    let mut cpu = LR35902::new(mock::Memory::new(vec![0x00]));
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 4);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x01() {
    let mut expected_state = LR35902::new(mock::Memory::new(vec![
        opcode::LdImm16IntoBC::OPCODE,
        0x7F,
        0x10,
    ]));
    expected_state.pc = 0x3;
    expected_state.bc.lo = 0x7F;
    expected_state.bc.hi = 0x10;

    let mut cpu = LR35902::new(mock::Memory::new(vec![
        opcode::LdImm16IntoBC::OPCODE,
        0x7F,
        0x10,
    ]));
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 12);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x02() {
    let mut expected_state = LR35902::new(mock::Memory::new(vec![
        opcode::LdAIntoMemoryBC::OPCODE,
        0x03,
    ]));
    expected_state.pc = 0x01;
    expected_state.bc.lo = 0x01;

    let mut cpu = LR35902::new(mock::Memory::new(vec![
        opcode::LdAIntoMemoryBC::OPCODE,
        0x00,
    ]));
    cpu.bc.lo = 0x01;
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 8);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x03() {
    let mut expected_state = LR35902::new(mock::Memory::new(vec![opcode::IncBC::OPCODE]));
    expected_state.pc = 0x01;
    expected_state.bc.hi = 0x01;
    expected_state.bc.lo = 0x00;

    let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncBC::OPCODE]));
    cpu.bc.hi = 0x00;
    cpu.bc.lo = 0xFF;
    let mut clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 8);
    assert_eq!(cpu, expected_state);

    expected_state.bc.hi = 0x00;
    expected_state.bc.lo = 0x00;

    cpu.bc.hi = 0xFF;
    cpu.bc.lo = 0xFF;
    cpu.pc = 0x0000;

    clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 8);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x04() {
    struct TestCase {
        expected_state: fn() -> LR35902,
        initial_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.bc.hi = 0x10;
                cpu.af.lo |= 1 << 5;
                cpu.pc = 1;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.bc.hi = 0x0F;
                cpu.af.lo |= 1 << 6;
                return cpu;
            },
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.pc = 1;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.af.lo |= 1 << 5;
                return cpu;
            },
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.pc = 1;
                cpu.bc.hi = 0x00;
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 5;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.bc.hi = 0xFF;
                return cpu;
            },
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.pc = 1;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::IncB::OPCODE]));
                cpu.af.lo |= 1 << 7;
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        let expected_state = (tc.expected_state)();
        let mut cpu = (tc.initial_state)();
        let clock_cycles = cpu.execute_next_opcode();

        assert_eq!(clock_cycles, 4);
        assert_eq!(cpu, expected_state);
    }
}

#[test]
fn _0x05() {
    struct TestCase {
        expected_state: fn() -> LR35902,
        initial_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.pc = 0x0001;
                cpu.af.lo |= 1 << 5;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0xFF;
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.pc = 0x0001;
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0x00;
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.pc = 0x0001;
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0x00;
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.bc.hi = 0x14;
                cpu.af.lo |= 1 << 5;
                cpu.af.lo |= 1 << 7;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::DecB::OPCODE]));
                cpu.pc = 0x0001;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0x13;
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 4);
        assert_eq!(cpu, (tc.expected_state)());
    }
}

#[test]
fn _0x06() {
    struct TestCase {
        expected_state: fn() -> LR35902,
        initial_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![opcode::LdImm8IntoB::OPCODE, 0xFF]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![0x05, 0xFF]));
            cpu.bc.hi = 0xFF;
            cpu.pc = 0x0002;
            return cpu;
        },
    }];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 8);
        assert_eq!(cpu, (tc.expected_state)());
    }
}

#[test]
fn _0x07() {
    struct TestCase {
        expected_state: fn() -> LR35902,
        initial_state: fn() -> LR35902,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    opcode::RotateLeftCarryIntoA::OPCODE,
                ]));
                cpu.af.hi = 0x01;
                cpu.af.lo |= lr35902::HALF_CARRY_FLAG_MASK
                    | lr35902::SUB_FLAG_MASK
                    | lr35902::ZERO_FLAG_MASK
                    | lr35902::CARRY_FLAG_MASK;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    opcode::RotateLeftCarryIntoA::OPCODE,
                ]));
                cpu.pc = 0x1;
                cpu.af.lo = 0x00;
                cpu.af.hi = 0x02;
                return cpu;
            },
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    opcode::RotateLeftCarryIntoA::OPCODE,
                ]));
                cpu.af.hi = 0x80;
                cpu.af.lo |= lr35902::HALF_CARRY_FLAG_MASK
                    | lr35902::SUB_FLAG_MASK
                    | lr35902::ZERO_FLAG_MASK;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    opcode::RotateLeftCarryIntoA::OPCODE,
                ]));
                cpu.pc = 0x1;
                cpu.af.lo |= lr35902::CARRY_FLAG_MASK;
                cpu.af.hi = 0x01;
                return cpu;
            },
        },
    ];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 4);
        assert_eq!(cpu, (tc.expected_state)());
    }
}

#[test]
fn _0x08() {
    struct TestCase {
        initial_state: fn() -> LR35902,
        assert_fn: fn(&LR35902),
    }

    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut memory: Vec<u8> = vec![0x0; 0x10000];
            memory[0x0] = opcode::LdSpInto16ImmAddress::OPCODE;
            memory[0x1] = 0xFF;
            memory[0x2] = 0x1F;
            let mut cpu = LR35902::new(mock::Memory::new(memory));
            cpu.sp = 0x1F37;
            return cpu;
        },
        assert_fn: |cpu| {
            assert_eq!(cpu.pc, 0x03);
            assert_eq!(cpu.sp, 0x1F37);
            assert_eq!(*cpu.memory.read(0x1FFF).unwrap(), 0x37);
            assert_eq!(*cpu.memory.read(0x2000).unwrap(), 0x1F);
        },
    }];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 20);
        (tc.assert_fn)(&cpu);
    }
}

#[test]
fn _0x09() {
    struct TestCase {
        initial_state: fn() -> LR35902,
        assert_fn: fn(&LR35902),
    }

    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![opcode::AddBCintoHL::OPCODE]));
            cpu.bc.set_word(0x0001);
            cpu.hl.set_word(0x0FFF);
            cpu.set_sub_flag();
            return cpu;
        },
        assert_fn: |cpu| {
            assert_eq!(cpu.test_sub_flag(), false);
            assert_eq!(cpu.pc, 0x01);
            assert_eq!(cpu.hl.word(), 0x1000);
            assert_eq!(cpu.test_half_carry_flag(), true);
        },
    }];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 8);
        (tc.assert_fn)(&cpu);
    }
}

#[test]
fn _0x0a() {
    struct TestCase {
        initial_state: fn() -> LR35902,
        assert_fn: fn(&LR35902),
    }

    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                opcode::LdMemoryBCIntoA::OPCODE,
                0x1F,
            ]));

            cpu.bc.set_word(0x0001);

            return cpu;
        },
        assert_fn: |cpu| {
            assert_eq!(cpu.af.hi, 0x1F);
            assert_eq!(cpu.pc, 0x0001);
        },
    }];

    for tc in test_cases {
        let mut cpu = (tc.initial_state)();
        assert_eq!(cpu.execute_next_opcode(), 8);
        (tc.assert_fn)(&cpu);
    }
}
