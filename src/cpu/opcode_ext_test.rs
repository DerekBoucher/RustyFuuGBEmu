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
