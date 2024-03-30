use crate::cpu::register;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Event {
    Register(RegisterAccess),
    Memory(MemoryAccess),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum RegisterAccess {
    Read(register::ID),
    Write(register::ID, u8),
    Read16(register::ID16),
    Write16(register::ID16, u16),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MemoryAccess {
    None,
    Read(u16, u8),
    Write(u16, u8),
}

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

    pub register_access: Vec<RegisterAccess>,
    pub memory_accesses: Vec<MemoryAccess>,
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
            register_access: Vec::new(),
            memory_accesses: Vec::new(),
        }
    }
}
