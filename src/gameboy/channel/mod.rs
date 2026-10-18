pub mod back_end;
pub mod front_end;

use crate::{
    audio::AudioData,
    joypad::{ActionButton, DirectionButton},
    ppu,
};
use back_end::Backend;
use front_end::Frontend;
use std::sync::mpsc;
use winit::event::ElementState;

pub fn new() -> (Frontend, Backend) {
    let (close_sender, close_receiver) = mpsc::sync_channel::<()>(1);
    let (ack_sender, ack_receiver) = mpsc::sync_channel::<()>(1);
    let (rom_data_sender, rom_data_receiver) = mpsc::sync_channel::<Vec<u8>>(1);
    let (frame_data_sender, frame_data_receiver) = mpsc::sync_channel::<
        [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
    >(1);
    let (audio_data_sender, audio_data_receiver) = mpsc::sync_channel::<AudioData>(1);
    let (skip_boot_rom_sender, skip_boot_rom_recv) = mpsc::sync_channel::<bool>(1);
    let (joypad_sender, joypad_recv) =
        mpsc::channel::<(Option<DirectionButton>, Option<ActionButton>, ElementState)>();
    let (pause_sender, pause_receiver) = mpsc::sync_channel::<bool>(1);

    (
        Frontend::new(
            close_sender,
            ack_receiver,
            rom_data_sender,
            frame_data_receiver,
            audio_data_receiver,
            skip_boot_rom_sender,
            joypad_sender,
            pause_sender,
        ),
        Backend::new(
            close_receiver,
            ack_sender,
            rom_data_receiver,
            frame_data_sender,
            audio_data_sender,
            skip_boot_rom_recv,
            joypad_recv,
            pause_receiver,
        ),
    )
}
