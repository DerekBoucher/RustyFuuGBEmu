use glium::glutin::event::ElementState;

use crate::{
    joypad::{ActionButton, DirectionButton},
    ppu::{self, Pixel, NATIVE_SCREEN_HEIGHT, NATIVE_SCREEN_WIDTH},
};
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

pub struct Backend {
    close_receiver: Receiver<()>,
    ack_sender: SyncSender<()>,
    rom_data_receiver: Receiver<Vec<u8>>,
    frame_data_sender: SyncSender<[[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT]>,
    skip_boot_rom_recv: Receiver<bool>,
    joypad_recv: Receiver<(Option<DirectionButton>, Option<ActionButton>, ElementState)>,
    pause_recv: Receiver<bool>,
}

impl Backend {
    pub fn new(
        close_receiver: Receiver<()>,
        ack_sender: SyncSender<()>,
        rom_data_receiver: Receiver<Vec<u8>>,
        frame_data_sender: SyncSender<[[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT]>,
        skip_boot_rom_recv: Receiver<bool>,
        joypad_recv: Receiver<(Option<DirectionButton>, Option<ActionButton>, ElementState)>,
        pause_recv: Receiver<bool>,
    ) -> Self {
        return Self {
            close_receiver,
            ack_sender,
            rom_data_receiver,
            frame_data_sender,
            skip_boot_rom_recv,
            joypad_recv,
            pause_recv,
        };
    }

    pub fn should_pause(&self) -> bool {
        let result = self.pause_recv.try_recv();
        match result {
            Ok(should_pause) => should_pause,
            Err(err) => match err {
                TryRecvError::Empty => false,
                _ => panic!("error occurred receving pause signal: {:?}", err),
            },
        }
    }

    pub fn wait_pause_resume(&self) {
        let result = self.pause_recv.recv();
        match result {
            Ok(_) => {}
            Err(err) => panic!("error occurred receiving resume signal: {:?}", err),
        }

        return;
    }

    pub fn should_close(&self) -> bool {
        let should_close_result = self.close_receiver.try_recv();
        match should_close_result {
            Ok(_) => true,
            Err(err) => match err {
                TryRecvError::Empty => false,
                _ => panic!("error occured receiving close signal: {:?}", err),
            },
        }
    }

    pub fn should_set_skip_bootrom(&self) -> Option<bool> {
        match self.skip_boot_rom_recv.try_recv() {
            Ok(should_skip) => Some(should_skip),
            Err(err) => match err {
                TryRecvError::Empty => None,
                _ => panic!("error occured receiving skip boot rom signal: {:?}", err),
            },
        }
    }

    pub fn should_load_rom(&self) -> Option<Vec<u8>> {
        match self.rom_data_receiver.try_recv() {
            Ok(rom_data) => Some(rom_data),
            Err(err) => match err {
                TryRecvError::Empty => None,
                _ => panic!("error occured receiving rom data signal: {:?}", err),
            },
        }
    }

    pub fn send_frame_data_front_end(
        &self,
        frame_data: [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
    ) {
        match self.frame_data_sender.send(frame_data) {
            Ok(_) => {}
            Err(err) => panic!(
                "error occured when sending frame data to front end: {:?}",
                err
            ),
        }
    }

    pub fn ack_front_end(&self) {
        match self.ack_sender.send(()) {
            Ok(_) => {}
            Err(err) => panic!("error occurred when sending ack to front end: {:?}", err),
        }
    }

    pub fn recv_joypad_data(
        &self,
    ) -> (Option<DirectionButton>, Option<ActionButton>, ElementState) {
        match self.joypad_recv.try_recv() {
            Ok(data) => data,
            Err(err) => match err {
                TryRecvError::Empty => (None, None, ElementState::Released),
                _ => panic!("error receiving joypad data from front end: {:?}", err),
            },
        }
    }
}
