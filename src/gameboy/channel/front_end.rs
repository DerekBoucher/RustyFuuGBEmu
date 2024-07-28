use crate::ppu::{self, Pixel};
use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::time::Duration;

pub struct Frontend {
    close_sender: mpsc::SyncSender<()>,
    ack_receiver: mpsc::Receiver<()>,
    rom_data_sender: mpsc::SyncSender<Vec<u8>>,
    frame_data_receiver:
        mpsc::Receiver<[[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT]>,
    skip_boot_rom_sender: mpsc::SyncSender<bool>,
}

impl Frontend {
    pub fn new(
        close_sender: mpsc::SyncSender<()>,
        ack_receiver: mpsc::Receiver<()>,
        rom_data_sender: mpsc::SyncSender<Vec<u8>>,
        frame_data_receiver: mpsc::Receiver<
            [[Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
        >,
        skip_boot_rom_sender: mpsc::SyncSender<bool>,
    ) -> Self {
        return Self {
            close_sender,
            ack_receiver,
            rom_data_sender,
            frame_data_receiver,
            skip_boot_rom_sender,
        };
    }

    pub fn send_close_back_end(&self) {
        match self.close_sender.send(()) {
            Ok(_) => {}
            Err(err) => panic!("error occurred sending close signal to back end: {:?}", err),
        }
    }

    pub fn join_back_end(&self) -> Result<(), RecvTimeoutError> {
        self.ack_receiver.recv_timeout(Duration::from_secs(5))
    }

    pub fn send_rom_data_back_end(&self, rom_data: Vec<u8>) {
        match self.rom_data_sender.send(rom_data) {
            Ok(_) => {}
            Err(err) => panic!("error occured when sending rom data to back end: {:?}", err),
        }
    }

    pub fn send_set_skip_boot_rom_back_end(&self, skip_boot_rom: bool) {
        match self.skip_boot_rom_sender.send(skip_boot_rom) {
            Ok(_) => {}
            Err(err) => panic!("error occured when sending rom data to back end: {:?}", err),
        }
    }

    pub fn should_render_screen(
        &self,
    ) -> Option<[[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT]> {
        match self.frame_data_receiver.try_recv() {
            Ok(frame) => Some(frame),
            Err(TryRecvError::Empty) => None,
            _ => None,
        }
    }
}
