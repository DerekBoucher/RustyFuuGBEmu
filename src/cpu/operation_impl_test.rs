#[cfg(test)]
mod opcodes {
    use crate::cpu::operation_impl::Nop;
    use crate::cpu::{Operation, LR35902};

    #[test]
    fn nop() {
        let original_state = LR35902::new();
        let mut cpu = LR35902::new();

        assert_eq!(Nop::op_code(), 0x00, "op code for NOP should be 0x00");
        assert_eq!(
            Nop::execute(&mut cpu),
            4,
            "NOP op code takes 4 clock cycles"
        );
        assert_eq!(
            cpu, original_state,
            "cpu state should not have been modified"
        );
    }
}
