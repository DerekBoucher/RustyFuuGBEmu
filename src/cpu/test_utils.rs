use crate::cpu::LR35902;
use crate::memory::mock;

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
