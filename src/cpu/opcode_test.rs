use crate::cpu::*;

#[derive(Debug)]
struct MockMemory {
    memory: Vec<u8>,
}

impl MockMemory {
    pub fn new(vec: Vec<u8>) -> Box<Self> {
        Box::new(MockMemory { memory: vec })
    }
}

impl MemoryDriver for MockMemory {
    fn read(&self, addr: usize) -> Option<&u8> {
        Some(&self.memory[addr])
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }
}

#[test]
fn _0x00() {
    let mut expected_state = LR35902::new(MockMemory::new(vec![0x00]));
    expected_state.pc = 0x1;

    let mut cpu = LR35902::new(MockMemory::new(vec![0x00]));
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 4);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x01() {
    let mut expected_state = LR35902::new(MockMemory::new(vec![0x01, 0x7F, 0x10]));
    expected_state.pc = 0x3;
    expected_state.bc.lo = 0x7F;
    expected_state.bc.hi = 0x10;

    let mut cpu = LR35902::new(MockMemory::new(vec![0x01, 0x7F, 0x10]));
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 12);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x02() {
    let mut expected_state = LR35902::new(MockMemory::new(vec![0x02, 0x03]));
    expected_state.pc = 0x01;
    expected_state.bc.lo = 0x01;

    let mut cpu = LR35902::new(MockMemory::new(vec![0x02, 0x00]));
    cpu.bc.lo = 0x01;
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 8);
    assert_eq!(cpu, expected_state);
}

#[test]
fn _0x03() {
    let mut expected_state = LR35902::new(MockMemory::new(vec![0x03]));
    expected_state.pc = 0x01;
    expected_state.bc.hi = 0x01;

    let mut cpu = LR35902::new(MockMemory::new(vec![0x03]));
    let clock_cycles = cpu.execute_next_opcode();

    assert_eq!(clock_cycles, 8);
    assert_eq!(cpu, expected_state);
}
