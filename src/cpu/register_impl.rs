use crate::cpu::Register;

#[path = "register_impl_test.rs"]
#[cfg(test)]
mod tests;

impl Register {
    pub fn word(&self) -> u16 {
        let hi: u16 = self.hi.into();
        let lo: u16 = self.lo.into();
        return (hi << 8) | lo;
    }
}
