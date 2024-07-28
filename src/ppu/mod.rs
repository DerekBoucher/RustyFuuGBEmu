mod ppu;
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
    pub fn to_rgb(&self) -> (f64, f64, f64) {
        match self {
            Self::White => (250.0 / 255.0, 251.0 / 255.0, 246.0 / 255.0),
            Self::LightGray => (197.0 / 255.0, 182.0 / 255.0, 189.0 / 255.0),
            Self::DarkGray => (85.0 / 255.0, 89.0 / 255.0, 116.0 / 255.0),
            Self::Black => (14.0 / 255.0, 14.0 / 255.0, 26.0 / 255.0),
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
