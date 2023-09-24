use crate::memory::mock;
use crate::ppu::{
    self, Color, LCDC_ADDRESS, LY_ADDRESS, PALETTE_ADDR, SCREEN_WIDTH, SCROLL_X_ADDRESS,
    SCROLL_Y_ADDRESS, WIN_X_ADDRESS, WIN_Y_ADDRESS,
};

#[test]
fn render_tiles() {
    let mut ppu = ppu::Ppu::new();

    // Setup the VRAM data
    let mut data: Vec<u8> = vec![0x00; 0x10000];
    data[LY_ADDRESS] = 0x00; // current scanline
    data[LCDC_ADDRESS] = 1 << 4; // Unsigned addressing, window disabled.
    data[SCROLL_X_ADDRESS] = 0x00; // No x-scroll
    data[SCROLL_Y_ADDRESS] = 0x00; // No y-scroll
    data[WIN_X_ADDRESS] = 0x00; // No win-x
    data[WIN_Y_ADDRESS] = 0x00; // No win-y
    data[PALETTE_ADDR] = 0b11100100; // Normal color palette

    // Only define 1 tile, for test simplicity
    // (1 tile is 16 bytes wide).
    // Only the top row of the tile is defined, again,
    // for simplicity. The top row of the tile will alternate
    // white, light gray, dark gray, black, etc...
    data[0x8000] = 0b01010101;
    data[0x8001] = 0b00110011;

    // Populate the tile mapping such that tile ID 0
    // is constantly re-used
    for addr in 0..0x400 {
        data[addr + 0x9800] = 0x00;
    }

    let memory = mock::Memory::new(data);

    ppu.render_tiles(&memory);

    let mut expected_scanline: [Color; SCREEN_WIDTH] = [Color::White; SCREEN_WIDTH];
    for pixel in 0..SCREEN_WIDTH {
        match pixel % 4 {
            0 => expected_scanline[pixel] = Color::White,
            1 => expected_scanline[pixel] = Color::LightGray,
            2 => expected_scanline[pixel] = Color::DarkGray,
            3 => expected_scanline[pixel] = Color::Black,
            _ => {}
        }
    }

    assert_eq!(ppu.pixels[0], expected_scanline)
}
