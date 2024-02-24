mod color;
mod ppu;
mod stat;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    pixels: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    scanline_counter: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}
