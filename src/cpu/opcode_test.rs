#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use super::Opcode;
use crate::cpu::*;
use std::vec;

struct TestCase {
    initial_state: fn() -> LR35902,
    expected_state: fn() -> LR35902,
    expected_cycles: u32,
}

impl TestCase {
    pub fn run(&self, i: usize) {
        println!("subtest #{} results:", i);
        println!("---------------------");

        let mut initial_state = (self.initial_state)();
        let initial_pc = initial_state.pc;
        let expected_state = (self.expected_state)();
        let actual_cycles = initial_state.execute_next_opcode();
        assert_eq!(actual_cycles, self.expected_cycles,);
        assert_eq!(initial_state, expected_state,);
        assert_ne!(
            initial_state.pc, 0x0000,
            "pc should never be 0 after an opcode"
        );
        assert_ne!(
            initial_pc, initial_state.pc,
            "pc should have changed value after executing an op code"
        );
    }
}

#[test]
fn _0x00() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            LR35902::new(mock::Memory::new(vec![Opcode::Nop_0x00.into()]))
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::Nop_0x00.into()]));
            cpu.pc = 0x1;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x01() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoBC_0x01.into(),
                0x7F,
                0x10,
            ]))
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoBC_0x01.into(),
                0x7F,
                0x10,
            ]));
            cpu.pc = 0x3;
            cpu.bc.lo = 0x7F;
            cpu.bc.hi = 0x10;

            return cpu;
        },
        expected_cycles: 12,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x02() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryBC_0x02.into(),
                0x00,
            ]));
            cpu.af.hi = 0xFF;
            cpu.bc.lo = 0x01;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryBC_0x02.into(),
                0xFF,
            ]));
            cpu.af.hi = 0xFF;
            cpu.pc = 0x01;
            cpu.bc.lo = 0x01;

            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x03() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncBC_0x03.into()]));
                cpu.bc.hi = 0x00;
                cpu.bc.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncBC_0x03.into()]));
                cpu.pc = 0x01;
                cpu.bc.hi = 0x01;
                cpu.bc.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncBC_0x03.into()]));
                cpu.bc.hi = 0xFF;
                cpu.bc.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncBC_0x03.into()]));
                cpu.pc = 0x01;
                cpu.bc.hi = 0x00;
                cpu.bc.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x04() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.bc.hi = 0x10;
                cpu.set_half_carry_flag();
                cpu.pc = 1;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.bc.hi = 0x0F;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.pc = 1;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.pc = 1;
                cpu.bc.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.pc = 1;
                cpu.bc.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x05() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.pc = 0x0001;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.bc.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.bc.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.bc.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.bc.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.bc.hi = 0x14;
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.pc = 0x0001;
                cpu.set_sub_flag();
                cpu.bc.hi = 0x13;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x06() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoB_0x06.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoB_0x06.into(),
                0xFF,
            ]));
            cpu.bc.hi = 0xFF;
            cpu.pc = 0x0002;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x07() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(vec![Opcode::RotateLeftIntoA_0x07.into()]));
                cpu.af.hi = 0x80;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(vec![Opcode::RotateLeftIntoA_0x07.into()]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x01;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(vec![Opcode::RotateLeftIntoA_0x07.into()]));
                cpu.af.hi = 0x40;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(vec![Opcode::RotateLeftIntoA_0x07.into()]));
                cpu.pc = 0x0001;
                cpu.af.hi = 0x80;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x08() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut memory: Vec<u8> = vec![0x0; 0x10];
            memory[0x0] = Opcode::LdSpInto16ImmAddress_0x08.into();
            memory[0x1] = 0x08;
            memory[0x2] = 0x00;
            let mut cpu = LR35902::new(mock::Memory::new(memory));
            cpu.sp = 0x1F3B;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut memory: Vec<u8> = vec![0x0; 0x10];
            memory[0x0] = Opcode::LdSpInto16ImmAddress_0x08.into();
            memory[0x1] = 0x08;
            memory[0x2] = 0x00;
            memory[0x0008] = 0x3B;
            memory[0x0009] = 0x1F;
            let mut cpu = LR35902::new(mock::Memory::new(memory));
            cpu.sp = 0x1F3B;
            cpu.pc = 0x0003;
            return cpu;
        },
        expected_cycles: 20,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x09() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddBCintoHL_0x09.into()]));
            cpu.bc.set_word(0x0001);
            cpu.hl.set_word(0x0FFF);
            cpu.set_sub_flag();
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddBCintoHL_0x09.into()]));
            cpu.bc.set_word(0x0001);
            cpu.hl.set_word(0x1000);
            cpu.reset_sub_flag();
            cpu.set_half_carry_flag();
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryBCIntoA_0x0A.into(),
                0x1F,
            ]));

            cpu.bc.set_word(0x0001);

            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryBCIntoA_0x0A.into(),
                0x1F,
            ]));

            cpu.bc.set_word(0x0001);
            cpu.af.hi = 0x1F;
            cpu.pc = 0x0001;

            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecBC_0x0B.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecBC_0x0B.into()]));
            cpu.bc.set_word(0xFFFF);
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncC_0x0C.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncC_0x0C.into()]));
            cpu.pc = 0x0001;
            cpu.bc.lo = 0x01;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecC_0x0D.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecC_0x0D.into()]));
            cpu.pc = 0x0001;
            cpu.bc.lo = 0xFF;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoC_0x0E.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoC_0x0E.into(),
                0xFF,
            ]));
            cpu.pc = 0x0002;
            cpu.bc.lo = 0xFF;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0f() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(
                        vec![Opcode::RotateRightIntoA_0x0F.into()],
                    ));
                cpu.af.hi = 0x01;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(
                        vec![Opcode::RotateRightIntoA_0x0F.into()],
                    ));
                cpu.pc = 0x0001;
                cpu.af.hi = 0x80;
                cpu.set_carry_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(
                        vec![Opcode::RotateRightIntoA_0x0F.into()],
                    ));
                cpu.af.hi = 0x02;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu =
                    LR35902::new(mock::Memory::new(
                        vec![Opcode::RotateRightIntoA_0x0F.into()],
                    ));
                cpu.pc = 0x0001;
                cpu.af.hi = 0x01;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x10() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::Stop_0x10.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::Stop_0x10.into()]));
            cpu.pc = 0x0001;
            cpu.paused = true;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x11() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoDE_0x11.into(),
                0x7F,
                0x10,
            ]))
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoDE_0x11.into(),
                0x7F,
                0x10,
            ]));
            cpu.pc = 0x3;
            cpu.de.lo = 0x7F;
            cpu.de.hi = 0x10;

            return cpu;
        },
        expected_cycles: 12,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x12() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryDE_0x12.into(),
                0x00,
            ]));
            cpu.af.hi = 0xFF;
            cpu.de.lo = 0x01;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryDE_0x12.into(),
                0xFF,
            ]));
            cpu.af.hi = 0xFF;
            cpu.de.lo = 0x01;
            cpu.pc = 0x01;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x13() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncDE_0x13.into()]));
                cpu.de.hi = 0x00;
                cpu.de.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncDE_0x13.into()]));
                cpu.pc = 0x01;
                cpu.de.hi = 0x01;
                cpu.de.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncDE_0x13.into()]));
                cpu.de.hi = 0xFF;
                cpu.de.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncDE_0x13.into()]));
                cpu.pc = 0x01;
                cpu.de.hi = 0x00;
                cpu.de.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x14() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.de.hi = 0x10;
                cpu.set_half_carry_flag();
                cpu.pc = 1;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.de.hi = 0x0F;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.pc = 1;
                cpu.de.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.pc = 1;
                cpu.de.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.de.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.pc = 1;
                cpu.de.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncD_0x14.into()]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x15() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.pc = 0x0001;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.de.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.de.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.de.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.de.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.de.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.de.hi = 0x14;
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecD_0x15.into()]));
                cpu.pc = 0x0001;
                cpu.set_sub_flag();
                cpu.de.hi = 0x13;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x16() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoD_0x16.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoD_0x16.into(),
                0xFF,
            ]));
            cpu.de.hi = 0xFF;
            cpu.pc = 0x0002;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x17() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.af.hi = 0x80;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.af.hi = 0xC0;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x80;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.af.hi = 0xC0;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x81;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.af.hi = 0x40;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateLeftWithCarryIntoA_0x17.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.af.hi = 0x80;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x18() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJump8_0x18.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJump8_0x18.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0002;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJump8_0x18.into(),
                    0x01,
                ]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJump8_0x18.into(),
                    0x01,
                ]));
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 12,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x19() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddDEintoHL_0x19.into()]));
            cpu.de.set_word(0x0001);
            cpu.hl.set_word(0x0FFF);
            cpu.set_sub_flag();
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddDEintoHL_0x19.into()]));
            cpu.de.set_word(0x0001);
            cpu.hl.set_word(0x1000);
            cpu.reset_sub_flag();
            cpu.set_half_carry_flag();
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryDEIntoA_0x1A.into(),
                0x1F,
            ]));

            cpu.de.set_word(0x0001);

            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryDEIntoA_0x1A.into(),
                0x1F,
            ]));

            cpu.de.set_word(0x0001);
            cpu.af.hi = 0x1F;
            cpu.pc = 0x0001;

            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecDE_0x1B.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecDE_0x1B.into()]));
            cpu.de.set_word(0xFFFF);
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncE_0x1C.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncE_0x1C.into()]));
            cpu.pc = 0x0001;
            cpu.de.lo = 0x01;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecE_0x1D.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecE_0x1D.into()]));
            cpu.pc = 0x0001;
            cpu.de.lo = 0xFF;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoE_0x1E.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoE_0x1E.into(),
                0xFF,
            ]));
            cpu.pc = 0x0002;
            cpu.de.lo = 0xFF;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1f() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.af.hi = 0x01;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.af.hi = 0x03;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x01;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.af.hi = 0x03;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                cpu.af.hi = 0x81;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.af.hi = 0x04;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RotateRightWithCarryIntoA_0x1F.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.af.hi = 0x02;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x20() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0002;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0x01,
                ]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0x01,
                ]));
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0xFF,
                ]));
                cpu.set_zero_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0003;
                cpu.set_zero_flag();
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0x01,
                ]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotZero8_0x20.into(),
                    0x01,
                ]));
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x21() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoHL_0x21.into(),
                0x7F,
                0x10,
            ]))
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoHL_0x21.into(),
                0x7F,
                0x10,
            ]));
            cpu.pc = 0x3;
            cpu.hl.lo = 0x7F;
            cpu.hl.hi = 0x10;

            return cpu;
        },
        expected_cycles: 12,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x22() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryHLPostInc_0x22.into(),
                0x00,
            ]));
            cpu.af.hi = 0xFF;
            cpu.hl.lo = 0x01;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryHLPostInc_0x22.into(),
                0xFF,
            ]));
            cpu.af.hi = 0xFF;
            cpu.hl.lo = 0x02;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x23() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncHL_0x23.into()]));
                cpu.hl.hi = 0x00;
                cpu.hl.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncHL_0x23.into()]));
                cpu.pc = 0x01;
                cpu.hl.hi = 0x01;
                cpu.hl.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncHL_0x23.into()]));
                cpu.hl.hi = 0xFF;
                cpu.hl.lo = 0xFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncHL_0x23.into()]));
                cpu.pc = 0x01;
                cpu.hl.hi = 0x00;
                cpu.hl.lo = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x24() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.hl.hi = 0x0F;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.hl.hi = 0x10;
                cpu.set_half_carry_flag();
                cpu.pc = 1;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.pc = 1;
                cpu.hl.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.pc = 1;
                cpu.hl.hi = 0x00;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.hl.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.pc = 1;
                cpu.hl.hi = 0x01;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncH_0x24.into()]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x25() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.pc = 0x0001;
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.hl.hi = 0xFF;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.hl.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.hl.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.hl.hi = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.pc = 0x0001;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                cpu.hl.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.hl.hi = 0x14;
                cpu.set_half_carry_flag();
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecH_0x25.into()]));
                cpu.pc = 0x0001;
                cpu.set_sub_flag();
                cpu.hl.hi = 0x13;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x26() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoH_0x26.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoH_0x26.into(),
                0xFF,
            ]));
            cpu.hl.hi = 0xFF;
            cpu.pc = 0x0002;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x27() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_carry_flag();
                cpu.af.hi = 0x60;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0xA0;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.af.hi = 0x00;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0x06;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0x0A;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0x10;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_sub_flag();
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0xA0;
                cpu.set_carry_flag();
                cpu.set_sub_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0xFA;
                cpu.set_sub_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0x01;
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DAA_0x27.into()]));
                cpu.af.hi = 0x01;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x28() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0x02,
                ]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0x02,
                ]));
                cpu.set_zero_flag();
                cpu.pc = 0x0004;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0xFF,
                ]));
                cpu.set_zero_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0x01,
                ]));
                cpu.set_zero_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpZero8_0x28.into(),
                    0x01,
                ]));
                cpu.set_zero_flag();
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 12,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x29() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddHLintoHL_0x29.into()]));
            cpu.hl.set_word(0x0FFF);
            cpu.set_sub_flag();
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddHLintoHL_0x29.into()]));
            cpu.hl.set_word(0x1FFE);
            cpu.reset_sub_flag();
            cpu.set_half_carry_flag();
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryHLIntoAPostInc_0x2A.into(),
                0x1F,
            ]));

            cpu.hl.set_word(0x0001);

            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryHLIntoAPostInc_0x2A.into(),
                0x1F,
            ]));

            cpu.hl.set_word(0x0002);
            cpu.af.hi = 0x1F;
            cpu.pc = 0x0001;

            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecHL_0x2B.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecHL_0x2B.into()]));
            cpu.hl.set_word(0xFFFF);
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncL_0x2C.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncL_0x2C.into()]));
            cpu.pc = 0x0001;
            cpu.hl.lo = 0x01;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecL_0x2D.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecL_0x2D.into()]));
            cpu.pc = 0x0001;
            cpu.hl.lo = 0xFF;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoL_0x2E.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoL_0x2E.into(),
                0xFF,
            ]));
            cpu.pc = 0x0002;
            cpu.hl.lo = 0xFF;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2f() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::ComplimentA_0x2F.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::ComplimentA_0x2F.into()]));
            cpu.pc = 0x0001;
            cpu.af.hi = 0xFF;
            cpu.set_sub_flag();
            cpu.set_half_carry_flag();
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x30() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0002;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0x01,
                ]));
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0x01,
                ]));
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0xFF,
                ]));
                cpu.set_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0003;
                cpu.set_carry_flag();
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0x01,
                ]));
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpNotCarry8_0x30.into(),
                    0x01,
                ]));
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x31() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoSP_0x31.into(),
                0x7F,
                0x10,
            ]))
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm16IntoSP_0x31.into(),
                0x7F,
                0x10,
            ]));
            cpu.pc = 0x3;
            cpu.sp = 0x107F;
            return cpu;
        },
        expected_cycles: 12,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x32() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryHLPostDec_0x32.into(),
                0x00,
            ]));
            cpu.af.hi = 0xFF;
            cpu.hl.lo = 0x01;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdAIntoMemoryHLPostDec_0x32.into(),
                0xFF,
            ]));
            cpu.af.hi = 0xFF;
            cpu.hl.lo = 0x00;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x33() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncSP_0x33.into()]));
                cpu.sp = 0xFFFF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncSP_0x33.into()]));
                cpu.pc = 0x01;
                cpu.sp = 0x00;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncSP_0x33.into()]));
                cpu.sp = 0x00FF;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncSP_0x33.into()]));
                cpu.pc = 0x01;
                cpu.sp = 0x0100;
                return cpu;
            },
            expected_cycles: 8,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x34() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0xFF,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0x00,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0x0F,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0x10,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_half_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0x00,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::IncMemoryHL_0x34.into(),
                    0x01,
                ]));
                cpu.hl.lo = 0x01;
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x35() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0x01,
                ]));
                cpu.hl.lo = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0x00,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0x10,
                ]));
                cpu.hl.lo = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0x0F,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0x00,
                ]));
                cpu.hl.lo = 0x01;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::DecMemoryHL_0x35.into(),
                    0xFF,
                ]));
                cpu.hl.lo = 0x01;
                cpu.set_sub_flag();
                cpu.set_half_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 12,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x36() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoMemoryHL_0x36.into(),
                0xFF,
                0x00,
            ]));
            cpu.hl.lo = 0x02;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoMemoryHL_0x36.into(),
                0xFF,
                0xFF,
            ]));
            cpu.hl.lo = 0x02;
            cpu.pc = 0x0002;
            return cpu;
        },
        expected_cycles: 12,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x37() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::SetCarryFlag_0x37.into()]));
            cpu.set_sub_flag();
            cpu.set_half_carry_flag();
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::SetCarryFlag_0x37.into()]));
            cpu.set_carry_flag();
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x38() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 8,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0x02,
                ]));
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0x02,
                ]));
                cpu.set_carry_flag();
                cpu.pc = 0x0004;
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0xFF,
                ]));
                cpu.set_carry_flag();
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    0x00,
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0xFF,
                ]));
                cpu.pc = 0x0002;
                cpu.set_carry_flag();
                return cpu;
            },
            expected_cycles: 12,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0x01,
                ]));
                cpu.set_carry_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::RelativeJumpCarry8_0x38.into(),
                    0x01,
                ]));
                cpu.set_carry_flag();
                cpu.pc = 0x0003;
                return cpu;
            },
            expected_cycles: 12,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x39() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddSPintoHL_0x39.into()]));
            cpu.hl.set_word(0x0FFF);
            cpu.sp = 0x0001;
            cpu.set_sub_flag();
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::AddSPintoHL_0x39.into()]));
            cpu.set_half_carry_flag();
            cpu.hl.set_word(0x1000);
            cpu.pc = 0x0001;
            cpu.sp = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryHLIntoAPostDec_0x3A.into(),
                0x1F,
            ]));

            cpu.hl.set_word(0x0001);

            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdMemoryHLIntoAPostDec_0x3A.into(),
                0x1F,
            ]));

            cpu.hl.set_word(0x0000);
            cpu.af.hi = 0x1F;
            cpu.pc = 0x0001;

            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecSP_0x3B.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecSP_0x3B.into()]));
            cpu.sp = 0xFFFF;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncA_0x3C.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncA_0x3C.into()]));
            cpu.pc = 0x0001;
            cpu.af.hi = 0x01;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecA_0x3D.into()]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecA_0x3D.into()]));
            cpu.pc = 0x0001;
            cpu.af.hi = 0xFF;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoA_0x3E.into(),
                0xFF,
            ]));
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![
                Opcode::LdImm8IntoA_0x3E.into(),
                0xFF,
            ]));
            cpu.pc = 0x0002;
            cpu.af.hi = 0xFF;
            return cpu;
        },
        expected_cycles: 8,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3f() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::ComplimentCarryFlag_0x3F.into(),
                ]));
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::ComplimentCarryFlag_0x3F.into(),
                ]));
                cpu.pc = 0x0001;
                cpu.set_carry_flag();
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::ComplimentCarryFlag_0x3F.into(),
                ]));
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![
                    Opcode::ComplimentCarryFlag_0x3F.into(),
                ]));
                cpu.pc = 0x0001;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x40() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdBIntoB_0x40.into()]));
            cpu.bc.hi = 0x40;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdBIntoB_0x40.into()]));
            cpu.bc.hi = 0x40;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x41() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdCIntoB_0x41.into()]));
            cpu.bc.lo = 0x40;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdCIntoB_0x41.into()]));
            cpu.bc.hi = 0x40;
            cpu.bc.lo = 0x40;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x42() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdDIntoB_0x42.into()]));
            cpu.de.hi = 0x40;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdDIntoB_0x42.into()]));
            cpu.bc.hi = 0x40;
            cpu.de.hi = 0x40;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x43() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdEIntoB_0x43.into()]));
            cpu.de.lo = 0x40;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdEIntoB_0x43.into()]));
            cpu.bc.hi = 0x40;
            cpu.de.lo = 0x40;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x44() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdHIntoB_0x44.into()]));
            cpu.hl.hi = 0x40;
            return cpu;
        },
        expected_state: || -> LR35902 {
            let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::LdHIntoB_0x44.into()]));
            cpu.bc.hi = 0x40;
            cpu.hl.hi = 0x40;
            cpu.pc = 0x0001;
            return cpu;
        },
        expected_cycles: 4,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}
