use crate::cpu::opcode::*;
use crate::cpu::MemoryDriver;
use crate::cpu::Register;
use crate::cpu::LR35902;

impl PartialEq for LR35902 {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
            && self.sp == other.sp
            && self.af.word() == other.af.word()
            && self.bc.word() == other.bc.word()
            && self.de.word() == other.de.word()
            && self.hl.word() == other.hl.word()
    }
}

impl LR35902 {
    pub fn new(memory_driver: Box<dyn MemoryDriver>) -> Self {
        Self {
            af: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0x0000,
            pc: 0x0000,
            memory: memory_driver,
        }
    }

    pub fn execute_next_opcode(&mut self) -> u32 {
        let op = match self.memory.read(usize::from(self.pc)) {
            Some(x) => *x,
            None => panic!(
                "memory returned empty value when attempting to fetch op code. Dumping cpu state...\n
                {:?}", self
            ),
        };

        match op {
            Nop::OPCODE => Nop::execute(self),
            LdImm16IntoBC::OPCODE => LdImm16IntoBC::execute(self),
            LdAIntoMemoryBC::OPCODE => LdAIntoMemoryBC::execute(self),
            IncBC::OPCODE => IncBC::execute(self),
            _ => 0,
        }
    }
}
