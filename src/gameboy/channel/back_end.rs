use std::cell::Cell;
use std::time::Duration;
use winit::event::ElementState;

use crate::{
    joypad::{ActionButton, DirectionButton},
    ppu::{self, Pixel, NATIVE_SCREEN_HEIGHT, NATIVE_SCREEN_WIDTH},
};
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TryRecvError};

// While paused, the back end checks for a close signal at this interval.
// std::sync::mpsc cannot wait on multiple channels at once, so polling is
// used to keep a paused back end responsive to shutdown requests.
const CLOSE_POLL_INTERVAL: Duration = Duration::from_millis(50);

pub struct Backend {
    close_receiver: Receiver<()>,
    close_requested: Cell<bool>,
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
            close_requested: Cell::new(false),
            ack_sender,
            rom_data_receiver,
            frame_data_sender,
            skip_boot_rom_recv,
            joypad_recv,
            pause_recv,
        };
    }

    pub fn should_pause(&self) -> bool {
        match self.pause_recv.try_recv() {
            Ok(should_pause) => should_pause,
            // No pause signal is pending. A disconnected channel means the
            // front end is gone, which can never send a pause signal.
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => false,
        }
    }

    // Blocks while the emulator is paused. Also watches for close requests so
    // that a paused emulator still shuts down when the front end exits.
    pub fn wait_pause_resume(&self) {
        loop {
            match self.pause_recv.recv_timeout(CLOSE_POLL_INTERVAL) {
                Ok(should_pause) => {
                    if !should_pause {
                        return;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if self.should_close() {
                        return;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
    }

    pub fn should_close(&self) -> bool {
        // Once a close has been observed it stays observed, so callers do not
        // need to worry about who consumed the close signal.
        if self.close_requested.get() {
            return true;
        }

        match self.close_receiver.try_recv() {
            Ok(_) => {
                self.close_requested.set(true);
                true
            }
            Err(TryRecvError::Empty) => false,
            // The front end is gone, so there is nothing left to emulate for.
            Err(TryRecvError::Disconnected) => {
                self.close_requested.set(true);
                true
            }
        }
    }

    pub fn should_set_skip_bootrom(&self) -> Option<bool> {
        match self.skip_boot_rom_recv.try_recv() {
            Ok(should_skip) => Some(should_skip),
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => None,
        }
    }

    pub fn should_load_rom(&self) -> Option<Vec<u8>> {
        match self.rom_data_receiver.try_recv() {
            Ok(rom_data) => Some(rom_data),
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => None,
        }
    }

    pub fn send_frame_data_front_end(
        &self,
        frame_data: [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
    ) {
        if let Err(err) = self.frame_data_sender.send(frame_data) {
            log::debug!(
                "error occurred when sending frame data to front end: {:?}",
                err
            );
        }
    }

    pub fn ack_front_end(&self) {
        if let Err(err) = self.ack_sender.send(()) {
            log::debug!("error occurred when sending ack to front end: {:?}", err);
        }
    }

    pub fn recv_joypad_data(
        &self,
    ) -> (Option<DirectionButton>, Option<ActionButton>, ElementState) {
        match self.joypad_recv.try_recv() {
            Ok(data) => data,
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => {
                (None, None, ElementState::Released)
            }
        }
    }
}
