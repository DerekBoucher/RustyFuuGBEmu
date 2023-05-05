#[path = "mock.rs"]
#[cfg(test)]
mod mock;

use std::vec;

use crate::cpu::*;

use super::Opcode;

struct TestCase {
    initial_state: fn() -> LR35902,
    expected_state: fn() -> LR35902,
    expected_cycles: u32,
}

impl TestCase {
    pub fn run(self) {
        let mut initial_state = (self.initial_state)();
        let expected_state = (self.expected_state)();
        assert_eq!(initial_state.execute_next_opcode(), self.expected_cycles);
        assert_eq!(initial_state, expected_state);
        assert_ne!(initial_state.pc, 0x0000);
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
    }
}

#[test]
fn _0x04() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.bc.hi = 0x10;
                cpu.af.lo |= 1 << 5;
                cpu.pc = 1;
                return cpu;
            },
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.bc.hi = 0x0F;
                cpu.af.lo |= 1 << 6;
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
                cpu.af.lo |= 1 << 5;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::IncB_0x04.into()]));
                cpu.pc = 1;
                cpu.bc.hi = 0x00;
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 5;
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
                cpu.af.lo |= 1 << 7;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for tc in test_cases {
        tc.run();
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
                cpu.af.lo |= 1 << 5;
                cpu.af.lo |= 1 << 6;
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
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 6;
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
                cpu.af.lo |= 1 << 7;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0x00;
                return cpu;
            },
            expected_cycles: 4,
        },
        TestCase {
            initial_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.bc.hi = 0x14;
                cpu.af.lo |= 1 << 5;
                cpu.af.lo |= 1 << 7;
                return cpu;
            },
            expected_state: || -> LR35902 {
                let mut cpu = LR35902::new(mock::Memory::new(vec![Opcode::DecB_0x05.into()]));
                cpu.pc = 0x0001;
                cpu.af.lo |= 1 << 6;
                cpu.bc.hi = 0x13;
                return cpu;
            },
            expected_cycles: 4,
        },
    ];

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
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

    for tc in test_cases {
        tc.run();
    }
}
