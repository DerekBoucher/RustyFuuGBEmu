use crate::cpu::opcode::Opcode;
use crate::cpu::opcode_ext::ExtendedOpcode;
use crate::cpu::test_utils::*;
use crate::cpu::LR35902;
use crate::memory::mock;

#[test]
fn _0x00() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateBLeft_0x00.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateBLeft_0x00.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x01() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateCLeft_0x01.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateCLeft_0x01.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x02() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateDLeft_0x02.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateDLeft_0x02.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x03() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateELeft_0x03.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateELeft_0x03.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x04() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateHLeft_0x04.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateHLeft_0x04.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x05() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateLLeft_0x05.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateLLeft_0x05.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x06() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.set_word(0x0002);
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateMemoryHLLeft_0x06.into(),
                0x80,
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.set_word(0x0002);
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateMemoryHLLeft_0x06.into(),
                0x01,
            ]);

            return (cpu, memory);
        },
        expected_cycles: 16,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x07() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x80;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateALeft_0x07.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x01;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateALeft_0x07.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x08() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateBRight_0x08.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateBRight_0x08.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x09() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateCRight_0x09.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateCRight_0x09.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateDRight_0x0A.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateDRight_0x0A.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateERight_0x0B.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateERight_0x0B.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateHRight_0x0C.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateHRight_0x0C.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateLRight_0x0D.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateLRight_0x0D.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.set_word(0x0002);
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateMemoryHLRight_0x0E.into(),
                0x01,
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.set_word(0x0002);
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateMemoryHLRight_0x0E.into(),
                0x80,
            ]);

            return (cpu, memory);
        },
        expected_cycles: 16,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x0f() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x01;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateARight_0x0F.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x80;
            cpu.set_carry_flag();
            cpu.pc = 0x0002;
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::RotateARight_0x0F.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x10() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBLeftWithCarry_0x10.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBLeftWithCarry_0x10.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBLeftWithCarry_0x10.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBLeftWithCarry_0x10.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x11() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCLeftWithCarry_0x11.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCLeftWithCarry_0x11.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCLeftWithCarry_0x11.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCLeftWithCarry_0x11.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x12() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDLeftWithCarry_0x12.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDLeftWithCarry_0x12.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDLeftWithCarry_0x12.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDLeftWithCarry_0x12.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x13() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateELeftWithCarry_0x13.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateELeftWithCarry_0x13.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateELeftWithCarry_0x13.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateELeftWithCarry_0x13.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHLeftWithCarry_0x14.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHLeftWithCarry_0x14.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHLeftWithCarry_0x14.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHLeftWithCarry_0x14.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLLeftWithCarry_0x15.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLLeftWithCarry_0x15.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLLeftWithCarry_0x15.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLLeftWithCarry_0x15.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x16() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLLeftWithCarry_0x16.into(),
                    0x80,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLLeftWithCarry_0x16.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLLeftWithCarry_0x16.into(),
                    0x80,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLLeftWithCarry_0x16.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x17() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateALeftWithCarry_0x17.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateALeftWithCarry_0x17.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateALeftWithCarry_0x17.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateALeftWithCarry_0x17.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBRightWithCarry_0x18.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBRightWithCarry_0x18.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBRightWithCarry_0x18.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateBRightWithCarry_0x18.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x19() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCRightWithCarry_0x19.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCRightWithCarry_0x19.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCRightWithCarry_0x19.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateCRightWithCarry_0x19.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1a() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDRightWithCarry_0x1A.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDRightWithCarry_0x1A.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDRightWithCarry_0x1A.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateDRightWithCarry_0x1A.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1b() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateERightWithCarry_0x1B.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateERightWithCarry_0x1B.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateERightWithCarry_0x1B.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateERightWithCarry_0x1B.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1c() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHRightWithCarry_0x1C.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHRightWithCarry_0x1C.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHRightWithCarry_0x1C.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateHRightWithCarry_0x1C.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1d() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLRightWithCarry_0x1D.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLRightWithCarry_0x1D.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLRightWithCarry_0x1D.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateLRightWithCarry_0x1D.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1e() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLRightWithCarry_0x1E.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLRightWithCarry_0x1E.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLRightWithCarry_0x1E.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateMemoryHLRightWithCarry_0x1E.into(),
                    0x80,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x1f() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateARightWithCarry_0x1F.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateARightWithCarry_0x1F.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateARightWithCarry_0x1F.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x80;
                cpu.set_carry_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::RotateARightWithCarry_0x1F.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftBIntoCarry_0x20.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x21() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftCIntoCarry_0x21.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x22() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftDIntoCarry_0x22.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x23() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftEIntoCarry_0x23.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftHIntoCarry_0x24.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftLIntoCarry_0x25.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x26() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x80,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x02,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftMemoryHLIntoCarry_0x26.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x27() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x80;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftLeftAIntoCarry_0x27.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightBIntoCarry_0x28.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x29() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightCIntoCarry_0x29.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2a() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightDIntoCarry_0x2A.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2b() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightEIntoCarry_0x2B.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2c() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightHIntoCarry_0x2C.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2d() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightLIntoCarry_0x2D.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2e() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x80,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x40,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.set_word(0x0002);
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightMemoryHLIntoCarry_0x2E.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x2f() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_zero_flag();
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x80;
                cpu.set_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x40;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::ShiftRightAIntoCarry_0x2F.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x30() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapB_0x30.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapB_0x30.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapB_0x30.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapB_0x30.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x31() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapC_0x31.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapC_0x31.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapC_0x31.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapC_0x31.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x32() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapD_0x32.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapD_0x32.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapD_0x32.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapD_0x32.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x33() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapE_0x33.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapE_0x33.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapE_0x33.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapE_0x33.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapH_0x34.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapH_0x34.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapH_0x34.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapH_0x34.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
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
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapL_0x35.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapL_0x35.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapL_0x35.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapL_0x35.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x36() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapMemoryHL_0x36.into(),
                    0xFB,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapMemoryHL_0x36.into(),
                    0xBF,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapMemoryHL_0x36.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapMemoryHL_0x36.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x37() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xFB;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapA_0x37.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0xBF;
                cpu.pc = 0x0002;
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapA_0x37.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_carry_flag();
                cpu.set_half_carry_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapA_0x37.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::SwapA_0x37.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x38() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightB_0x38.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.hi = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightB_0x38.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x39() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightC_0x39.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.bc.lo = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightC_0x39.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3a() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightD_0x3A.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.hi = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightD_0x3A.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3b() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightE_0x3B.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.de.lo = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightE_0x3B.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3c() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightH_0x3C.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.hi = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightH_0x3C.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3d() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightL_0x3D.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightL_0x3D.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3e() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x02;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightMemoryHL_0x3E.into(),
                0x01,
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.hl.lo = 0x02;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightMemoryHL_0x3E.into(),
                0x00,
            ]);

            return (cpu, memory);
        },
        expected_cycles: 16,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x3f() {
    let test_cases: Vec<TestCase> = vec![TestCase {
        initial_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x01;
            cpu.set_half_carry_flag();
            cpu.set_sub_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightA_0x3F.into(),
            ]);

            return (cpu, memory);
        },
        expected_state: || -> (LR35902, mock::Memory) {
            let mut cpu = LR35902::new();
            cpu.af.hi = 0x00;
            cpu.pc = 0x0002;
            cpu.set_zero_flag();
            cpu.set_carry_flag();
            let memory = mock::Memory::new(vec![
                Opcode::ExtendedOpCode_0xCB.into(),
                ExtendedOpcode::ShiftRightA_0x3F.into(),
            ]);

            return (cpu, memory);
        },
        expected_cycles: 8,
        disable_pc_check: false,
    }];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x40() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_B_0x40.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_B_0x40.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_B_0x40.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_B_0x40.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x41() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_C_0x41.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_C_0x41.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_C_0x41.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.bc.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_C_0x41.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x42() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_D_0x42.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_D_0x42.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_D_0x42.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_D_0x42.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x43() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_E_0x43.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_E_0x43.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_E_0x43.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.de.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_E_0x43.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x44() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_H_0x44.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_H_0x44.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_H_0x44.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_H_0x44.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x45() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_L_0x45.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_L_0x45.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_L_0x45.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_L_0x45.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x46() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_MemoryHL_0x46.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_MemoryHL_0x46.into(),
                    0x01,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_MemoryHL_0x46.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.hl.lo = 0x02;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_MemoryHL_0x46.into(),
                    0x00,
                ]);

                return (cpu, memory);
            },
            expected_cycles: 16,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}

#[test]
fn _0x47() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.set_zero_flag();
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_A_0x47.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x01;
                cpu.pc = 0x0002;
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_A_0x47.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
        TestCase {
            initial_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.set_sub_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_A_0x47.into(),
                ]);

                return (cpu, memory);
            },
            expected_state: || -> (LR35902, mock::Memory) {
                let mut cpu = LR35902::new();
                cpu.af.hi = 0x00;
                cpu.pc = 0x0002;
                cpu.set_zero_flag();
                cpu.set_half_carry_flag();
                let memory = mock::Memory::new(vec![
                    Opcode::ExtendedOpCode_0xCB.into(),
                    ExtendedOpcode::TestBit0_A_0x47.into(),
                ]);

                return (cpu, memory);
            },
            expected_cycles: 8,
            disable_pc_check: false,
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        tc.run(i);
    }
}
