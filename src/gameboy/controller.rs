use super::Controller;
use crossbeam::channel::{self, RecvTimeoutError};
use std::time::Duration;

#[path = "controller_test.rs"]
#[cfg(test)]
mod test;

impl Controller {
    pub fn new() -> (
        Self,
        channel::Receiver<()>,
        channel::Receiver<()>,
        channel::Sender<()>,
        channel::Receiver<Vec<u8>>,
    ) {
        let (close_sender, close_receiver) = channel::unbounded();
        let (pause_sender, pause_receiver) = channel::bounded(1);
        let (ack_sender, ack_receiver) = channel::unbounded();
        let (rom_data_sender, rom_data_receiver) = channel::bounded::<Vec<u8>>(1);

        (
            Self {
                close_sender,
                pause_sender,
                ack_receiver,
                rom_data_sender,

                paused: false,
            },
            close_receiver,
            pause_receiver,
            ack_sender,
            rom_data_receiver,
        )
    }

    pub fn close(&self) {
        self.close_sender.send(()).unwrap();
    }

    pub fn join(&self) -> Result<(), RecvTimeoutError> {
        self.ack_receiver.recv_timeout(Duration::from_secs(5))
    }

    pub fn pause(&mut self) {
        if self.paused {
            log::debug!("gb emulation already paused");
            return;
        }

        self.paused = true;
        self.pause_sender.send(()).unwrap();
    }

    pub fn resume(&mut self) {
        if !self.paused {
            log::debug!("gb emulation already running");
            return;
        }

        self.paused = false;
        self.pause_sender.send(()).unwrap();
    }

    pub fn load_rom(&self, rom_data: Vec<u8>) {
        self.rom_data_sender.send(rom_data).unwrap();
    }
}
