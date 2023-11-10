use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use crate::gameboy::Controller;

impl Controller {
    pub fn new() -> (
        Self,
        Receiver<()>,
        Receiver<()>,
        Sender<()>,
        Receiver<Vec<u8>>,
    ) {
        let (close_sender, close_receiver) = mpsc::channel();
        let (pause_sender, pause_receiver) = mpsc::sync_channel(1);
        let (ack_sender, ack_receiver) = mpsc::channel();
        let (rom_data_sender, rom_data_receiver) = mpsc::channel::<Vec<u8>>();

        return (
            Self {
                close_sender: close_sender,
                pause_sender: pause_sender,
                ack_receiver: ack_receiver,
                rom_data_sender: rom_data_sender,

                paused: false,
            },
            close_receiver,
            pause_receiver,
            ack_sender,
            rom_data_receiver,
        );
    }

    pub fn close(&self) {
        self.close_sender.send(()).unwrap();
    }

    pub fn join(&self) {
        match self.ack_receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => (),
            Err(err) => {
                log::error!("gb thread did not close in time: {:?}", err);
            }
        }
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
