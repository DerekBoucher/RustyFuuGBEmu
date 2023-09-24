use crate::{memory, ppu::*};
use std::collections::HashMap;

#[path = "ppu_test.rs"]
#[cfg(test)]
mod test;

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            pixels: [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT],
            draw_counter: 0,
        }
    }

    // Tiles on GB are represented in VRAM as 16 bytes. They are located in the address range
    // 0x8000-0x97FF. In total, there are 384 possible tiles that can be defined in that area.

    // Each tile is made up of a 8x8 pixel grid, where each pixel's color is encoded as 2 bits.
    // This means that each pixel can only ever be 4 distinct colors (4 shades of gray on the original hardware).

    // There are two types of tiles:
    // 1 - Background tiles
    // 2 - Window tiles

    // Background tiles make up the background environment, and typically have lower precedence then the window tiles.
    // Window tiles have precedence over background tiles, when enabled.
    fn render_tiles(&mut self, memory: &impl memory::Interface) {
        let current_scanline = memory.read(LY_ADDRESS).unwrap();
        let lcdc = memory.read(LCDC_ADDRESS).unwrap();
        let scroll_x = memory.read(SCROLL_X_ADDRESS).unwrap();
        let scroll_y = memory.read(SCROLL_Y_ADDRESS).unwrap();
        let win_x = memory.read(WIN_X_ADDRESS).unwrap().wrapping_sub(7); // TODO: Explain the sub 7
        let win_y = memory.read(WIN_Y_ADDRESS).unwrap();
        let color_palette = memory.read(PALETTE_ADDR).unwrap();

        // Base address for the tile data in VRAM.
        // This area of memory contains the actual pixel color encodings to render tiles.
        // Each contiguous 16 bytes represent a tile. Each tile is indexed by their ID, which is simply the order
        // in which they appear in this VRAM area. Example: 0x8000-0x800F -> Tile ID #1, 0x8010-0x801F -> Tile ID #2, etc...
        let (tile_data_ptr, uses_unsigned_id) = determine_tile_data_address(lcdc);

        // Base address for the tile map in VRAM.
        // The tile map is made up of a 32x32 grid of "tile ID's". These tell
        // the PPU which tile data to use to make up the current frame.
        // This also tells the PPU how much of an offset it needs to calculate when accessing the tile data section.
        let tile_map_ptr = determine_tile_map_address(lcdc);

        // The y-coordinate of the current scanline we are rendering.
        // The scroll-y value here allows for simulating a 'scrolling' effect when the frames
        // are rendered.

        // TODO: determine if the window logic is correct
        let mut pixel_y: u8 = scroll_y.wrapping_add(current_scanline);
        if window_enabled(lcdc) {
            pixel_y = current_scanline.wrapping_sub(win_y);
        }

        // The row of the tile to render.
        // Since the tile map is made up of a 32x32 grid of tiles, this value is determined by
        // dividing the current pixel's y position by 8 -> gives us the byte offset within a tile, and then
        // multiplying by 32 -> to advance the memory pointer to the correct row of tiles within the 32 rows.
        let tile_row: usize = (pixel_y.wrapping_div(8).wrapping_mul(32)).into();

        // Main loop through each 160 pixels of the current scanline we are rendering
        for pixel in 0..SCREEN_WIDTH as u8 {
            let mut pixel_x: u8 = pixel.wrapping_add(scroll_x);

            if window_enabled(lcdc) && (pixel >= win_x) {
                pixel_x = pixel.wrapping_sub(win_x);
            }

            let tile_column: usize = (pixel_x / 8).into();
            let tile_id_address: usize = tile_map_ptr + tile_column + tile_row;
            let mut tile_id: usize = memory.read(tile_id_address).unwrap().into();
            let tile_line_offset: usize = ((pixel_y % 8) * 2).into();
            let mut tile_data_address: usize = tile_data_ptr + (tile_id * 16);

            if !uses_unsigned_id && (tile_id & 0x80) > 0 {
                tile_id = !tile_id;
                tile_id = tile_id.wrapping_add(1);
                tile_data_address = tile_data_ptr - (tile_id as usize * 16);
            }

            let data1 = memory.read(tile_data_address + tile_line_offset).unwrap();
            let data2 = memory
                .read(tile_data_address + tile_line_offset + 1)
                .unwrap();

            let current_bit_position: usize = 7 - ((pixel as usize + scroll_x as usize) % 8);

            let mut pixel_color_encoding: u8 = 0x00;

            // The second byte representing the tile row
            // is for the most significant bit of the 2-bit encoding
            if data2 & (1 << current_bit_position) > 0 {
                pixel_color_encoding |= 0b10;
            }

            // Likewise, the first byte contains the least significant bit of the
            // 2-bit encoding.
            if data1 & (1 << current_bit_position) > 0 {
                pixel_color_encoding |= 0b01;
            }

            // Depending on the current color palette that is in the PALETTE register,
            // these encoding translate to different colors / shades of gray.
            let palette_00 = (color_palette) & 0b11;
            let palette_01 = (color_palette >> 2) & 0b11;
            let palette_10 = (color_palette >> 4) & 0b11;
            let palette_11 = (color_palette >> 6) & 0b11;

            let mut palette_map: HashMap<u8, u8> = HashMap::new();
            palette_map.insert(0b00, palette_00);
            palette_map.insert(0b01, palette_01);
            palette_map.insert(0b10, palette_10);
            palette_map.insert(0b11, palette_11);

            let pixel_color: Color;
            match palette_map.get(&pixel_color_encoding).unwrap() {
                0b00 => pixel_color = Color::White,
                0b01 => pixel_color = Color::LightGray,
                0b10 => pixel_color = Color::DarkGray,
                0b11 => pixel_color = Color::Black,
                _ => panic!(
                    "invalid color encoding when rendering scanline: {}",
                    pixel_color_encoding
                ),
            }

            self.pixels[current_scanline as usize][pixel as usize] = pixel_color;
        }
    }
}

// If the 4th bit (starting from the right, 0 based) of the LCDC register is '1', then the
// background and window tile data are located at the base address of 0x8000 and the addressing uses an unsigned 8-bit integer.
// If the bit is '0', then the base address is 0x9000, and the addressing should use a signed 8-bit integer.
fn determine_tile_data_address(lcdc: u8) -> (usize, bool) {
    if (lcdc & (1 << 4)) > 0 {
        return (0x8000, true);
    }

    return (0x9000, false);
}

// If the 3rd bit (starting from the right, 0 based) of the LCDC register is '1', then the
// tile mappings are located at the base address of 0x9C00.
// If the bit is '0', then the base address is 0x9800.
fn determine_tile_map_address(lcdc: u8) -> usize {
    if (lcdc & (1 << 3)) > 0 {
        return 0x9C00;
    }

    return 0x9800;
}

// If the 5th bit (starting from the right, 0 based) of the LCDC register is '1', then the
// window should be rendered. Else it should be ignored.
fn window_enabled(lcdc: u8) -> bool {
    return (lcdc & (1 << 5)) > 0;
}