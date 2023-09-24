#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod ppu;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const LY_ADDRESS: usize = 0xFF44;
const LCDC_ADDRESS: usize = 0xFF40;
const SCROLL_X_ADDRESS: usize = 0xFF43;
const SCROLL_Y_ADDRESS: usize = 0xFF42;
const WIN_X_ADDRESS: usize = 0xFF4B;
const WIN_Y_ADDRESS: usize = 0xFF4A;
const PALETTE_ADDR: usize = 0xFF47;

#[derive(Debug)]
struct Ppu {
    pixels: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    draw_counter: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}
