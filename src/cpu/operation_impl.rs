use crate::cpu::Operation;
use crate::cpu::LR35902;

#[path = "operation_impl_test.rs"]
#[cfg(test)]
mod tests;

/// Nop opcode does nothing
struct Nop;

impl Operation for Nop {
    fn op_code() -> u8 {
        0x00
    }

    fn execute(cpu: &mut LR35902) -> u32 {
        4
    }
}
