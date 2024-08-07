pub mod back_end;
pub mod front_end;

use crate::{
    joypad::{ActionButton, DirectionButton},
    ppu,
};
use back_end::Backend;
use front_end::Frontend;
use std::sync::mpsc;

pub fn new() -> (Frontend, Backend) {
    let (close_sender, close_receiver) = mpsc::sync_channel::<()>(1);
    let (ack_sender, ack_receiver) = mpsc::sync_channel::<()>(1);
    let (rom_data_sender, rom_data_receiver) = mpsc::sync_channel::<Vec<u8>>(1);
    let (frame_data_sender, frame_data_receiver) = mpsc::sync_channel::<
        [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
    >(1);
    let (skip_boot_rom_sender, skip_boot_rom_recv) = mpsc::sync_channel::<bool>(1);
    let (joypad_sender, joypad_recv) =
        mpsc::channel::<(Option<DirectionButton>, Option<ActionButton>)>();

    return (
        Frontend::new(
            close_sender,
            ack_receiver,
            rom_data_sender,
            frame_data_receiver,
            skip_boot_rom_sender,
            joypad_sender,
        ),
        Backend::new(
            close_receiver,
            ack_sender,
            rom_data_receiver,
            frame_data_sender,
            skip_boot_rom_recv,
            joypad_recv,
        ),
    );
}
