mod ppu;
mod stat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Pixel {
    pub fn to_rgb(&self) -> (f32, f32, f32) {
        match self {
            Self::White => (1.0, 1.0, 1.0),
            Self::Black => (0.0, 0.0, 0.0),
            Self::LightGray => (0.75, 0.75, 0.75),
            Self::DarkGray => (0.83, 0.83, 0.83),
        }
    }
}

pub const NATIVE_SCREEN_WIDTH: usize = 160;
pub const NATIVE_SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct PPU {
    pixels: [[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT],
    scanline_counter: i32,
}
