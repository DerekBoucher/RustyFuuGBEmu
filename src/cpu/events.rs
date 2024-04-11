use crate::cpu::register;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Event {
    Register(RegisterAccess),
    Memory(MemoryAccess),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum RegisterAccess {
    Unused,
    None,
    Read(register::ID),
    Write(register::ID, u8),
    Read16(register::ID16),
    Write16(register::ID16, u16),
}
const REGISTER_ACCESS_UNUSED: RegisterAccess = RegisterAccess::Unused;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MemoryAccess {
    Unused,
    None,
    Read(u16),
    Write(u16, u8),
}
const MEMORY_ACCESS_UNUSED: MemoryAccess = MemoryAccess::Unused;

const ACCESSES_LENGTH: usize = 4;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Trace {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub cpu_cycles: u64,
    pub opcode: u8,

    pub register_access: [RegisterAccess; ACCESSES_LENGTH],
    pub memory_accesses: [MemoryAccess; ACCESSES_LENGTH],
}

impl Trace {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cpu_cycles: 0,
            opcode: 0,
            register_access: [REGISTER_ACCESS_UNUSED; ACCESSES_LENGTH],
            memory_accesses: [MEMORY_ACCESS_UNUSED; ACCESSES_LENGTH],
        }
    }

    pub fn clear(&mut self) {
        self.a = 0;
        self.f = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
        self.sp = 0;
        self.pc = 0;
        self.cpu_cycles = 0;
        for i in 0..ACCESSES_LENGTH {
            self.register_access[i] = REGISTER_ACCESS_UNUSED;
            self.memory_accesses[i] = MEMORY_ACCESS_UNUSED;
        }
    }
}
