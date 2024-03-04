use crate::interface;
use crate::interface::mock;
use crate::memory;
use crate::ppu;

#[test]
fn render_tiles() {
    let mut ppu = ppu::Ppu::new();

    // Setup the VRAM data
    let mut data: Vec<u8> = vec![0x00; 0x10000];
    data[memory::io_registers::LCD_LY_ADDR] = 0x00; // current scanline
    data[memory::io_registers::LCD_CONTROL_ADDR] = 1 << 4; // Unsigned addressing, window disabled.
    data[memory::io_registers::LCD_SCX_ADDR] = 0x00; // No x-scroll
    data[memory::io_registers::LCD_SCY_ADDR] = 0x00; // No y-scroll
    data[memory::io_registers::LCD_WINX_ADDR] = 0x00; // No win-x
    data[memory::io_registers::LCD_WINY_ADDR] = 0x00; // No win-y
    data[memory::io_registers::LCD_PALETTE_ADDR] = 0b11100100; // Normal color palette

    // Only define the top line of a single tile, for test simplicity.
    // (1 tile is 16 bytes wide, with each 2 consecutive bytes representing a line).
    // Only the top line of the tile is defined, again,
    // for simplicity. The top line of the tile will alternate
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

    let mut expected_scanline: [interface::Pixel; interface::NATIVE_SCREEN_WIDTH] =
        [interface::Pixel::White; interface::NATIVE_SCREEN_WIDTH];
    for pixel in 0..interface::NATIVE_SCREEN_WIDTH {
        match pixel % 4 {
            0 => expected_scanline[pixel] = interface::Pixel::White,
            1 => expected_scanline[pixel] = interface::Pixel::LightGray,
            2 => expected_scanline[pixel] = interface::Pixel::DarkGray,
            3 => expected_scanline[pixel] = interface::Pixel::Black,
            _ => {}
        }
    }

    assert_eq!(ppu.pixels[0], expected_scanline)
}
