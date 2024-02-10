use core::fmt::Debug;

#[cfg(test)]
pub mod mock;

pub trait Memory: Debug {
    fn read(&self, addr: usize) -> Option<u8>;
    fn write(&mut self, addr: usize, val: u8);
    fn dump(&self) -> Vec<u8>;
    fn update_timers(&mut self, cycles: u32);
}
