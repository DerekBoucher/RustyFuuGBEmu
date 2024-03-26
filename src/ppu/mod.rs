use crate::interface;

mod ppu;
mod stat;

#[derive(Debug)]
pub struct PPU {
    pixels: [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH]; interface::NATIVE_SCREEN_HEIGHT],
    scanline_counter: i32,
}
