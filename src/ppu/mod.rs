mod color;
mod ppu;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    pixels: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}
