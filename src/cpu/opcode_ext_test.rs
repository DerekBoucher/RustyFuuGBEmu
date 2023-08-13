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
