#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod ppu;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
struct Ppu {
    pixels: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    draw_counter: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}
