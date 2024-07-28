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
            Self::White => (255.0 / 255.0, 246.0 / 255.0, 210.0 / 255.0),
            Self::LightGray => (249.0 / 255.0, 168.0 / 255.0, 116.0 / 255.0),
            Self::DarkGray => (235.0 / 255.0, 106.0 / 255.0, 110.0 / 255.0),
            Self::Black => (123.0 / 255.0, 63.0 / 255.0, 87.0 / 255.0),
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
