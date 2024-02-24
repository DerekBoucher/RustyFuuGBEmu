use crate::interface;

/// Placeholder cartridge for when no cartridge is inserted.
#[derive(Debug)]
pub struct NullCartridge;

impl interface::Cartridge for NullCartridge {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn read(&self, _addr: usize) -> Option<u8> {
        log::error!("Tried to read from null cartridge");

        None
    }

    fn write(&mut self, _addr: usize, _val: u8) {
        log::error!("Tried to write to null cartridge");
    }
}
