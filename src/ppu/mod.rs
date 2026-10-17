mod processor;
mod sprite;
mod stat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Pixel {
    pub fn to_rgb_u8(self) -> (u8, u8, u8) {
        match self {
            Self::White => (250, 251, 246),
            Self::LightGray => (197, 182, 189),
            Self::DarkGray => (85, 89, 116),
            Self::Black => (14, 14, 26),
        }
    }
}

pub const NATIVE_SCREEN_WIDTH: usize = 160;
pub const NATIVE_SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    pixels: [[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT],
    pixel_encodings: [[Pixel; NATIVE_SCREEN_WIDTH]; NATIVE_SCREEN_HEIGHT],
    scanline_counter: i32,
}
