use core::panic;
use std::sync::{self, Arc};

use crate::{
    interrupt,
    memory::{
        self,
        io_registers::{self},
    },
    ppu::{
        self,
        stat::{self, StatUpdater},
        *,
    },
};

const LCDC_ENABLE_MASK: u8 = 1 << 7;
const LCDC_BG_WINDOW_ENABLE_MASK: u8 = 1 << 0;
const LCDC_OBJ_ENABLE_MASK: u8 = 1 << 1;

impl PPU {
    pub fn new() -> Self {
        PPU {
            pixels: [[ppu::Pixel::White; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT],
            pixel_encodings: [[ppu::Pixel::White; ppu::NATIVE_SCREEN_WIDTH];
                ppu::NATIVE_SCREEN_HEIGHT],
            scanline_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = PPU::new();
    }

    pub fn step_graphics(
        &mut self,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        interrupt_bus: &Arc<sync::Mutex<interrupt::Bus>>,
    ) {
        let lcdc = match memory
            .lock()
            .unwrap()
            .dma_read(io_registers::LCD_CONTROL_ADDR)
        {
            Some(value) => value,
            None => {
                log::error!("failed to read lcdc register");
                return;
            }
        };

        let stat = match memory.lock().unwrap().dma_read(io_registers::LCD_STAT_ADDR) {
            Some(value) => value,
            None => {
                log::error!("failed to read stat register");
                return;
            }
        };

        let current_scanline = match memory.lock().unwrap().dma_read(io_registers::LCD_LY_ADDR) {
            Some(value) => value,
            None => {
                log::error!("failed to read LY register");
                return;
            }
        };

        let ly = match memory.lock().unwrap().dma_read(io_registers::LCD_LY_ADDR) {
            Some(value) => value,
            None => {
                log::error!("failed to read LY register");
                return;
            }
        };

        let lyc = match memory.lock().unwrap().dma_read(io_registers::LCD_LYC_ADDR) {
            Some(value) => value,
            None => {
                log::error!("failed to read LYC register");
                return;
            }
        };

        self.set_lcdc_status(lcdc, stat, current_scanline, ly, lyc, memory, interrupt_bus);

        if lcdc & LCDC_ENABLE_MASK == 0 {
            return;
        }

        for _ in 0..4 {
            self.scanline_counter -= 1;

            if self.scanline_counter <= 0 {
                self.scanline_counter += stat::MAX_SCANLINE_COUNT;

                if current_scanline < 144 {
                    self.draw_scaline(lcdc, memory);
                    memory
                        .lock()
                        .unwrap()
                        .write(io_registers::LCD_LY_ADDR, ly.wrapping_add(1));
                    return;
                }

                // V-Blank period
                if current_scanline >= 144 && current_scanline < 154 {
                    interrupt_bus
                        .lock()
                        .unwrap()
                        .request(interrupt::Interrupt::VBlank);
                    memory
                        .lock()
                        .unwrap()
                        .write(io_registers::LCD_LY_ADDR, ly.wrapping_add(1));
                    return;
                }

                // Else, this means we've been through an entire frame cycle,
                // reset the LY register to 0.
                memory
                    .lock()
                    .unwrap()
                    .write(io_registers::LCD_LY_ADDR, 0x00);
            }
        }
    }

    fn render_sprites(&mut self, lcdc: u8, memory: &Arc<sync::Mutex<memory::Memory>>) {
        let sprite_8x16 = lcdc & (1 << 2) > 0;

        let sprites = sprite::process_from_memory(memory);

        let current_scanline = memory
            .lock()
            .unwrap()
            .dma_read(io_registers::LCD_LY_ADDR)
            .unwrap();

        for mut sprite in sprites {
            let sprite_height_pixel: u8;
            if sprite_8x16 {
                sprite_height_pixel = 16;
                sprite.pattern_number &= 0xFE;
            } else {
                sprite_height_pixel = 8;
            }

            // Only need to update pixels if the actual sprite falls within the current scanline
            if current_scanline >= sprite.get_y()
                && current_scanline < (sprite.get_y() + sprite_height_pixel)
            {
                let mut line: i32 = (current_scanline - sprite.get_y()) as i32;

                if sprite.is_y_flipped() {
                    line -= sprite_height_pixel as i32;
                    line *= -1;
                }

                line *= 2;

                let data_addr: usize =
                    (0x8000 + (sprite.get_pattern_number() as usize * 16) as usize) + line as usize;

                let data1 = memory.lock().unwrap().dma_read(data_addr).unwrap();
                let data2 = memory.lock().unwrap().dma_read(data_addr + 1).unwrap();

                for tile_pixel in (0..8).rev() {
                    let mut color_bit = tile_pixel;

                    if !sprite.is_x_flipped() {
                        color_bit -= 7;
                        color_bit *= -1;
                    }

                    let mut color_code: u8 = 0x00;
                    if data2 & (1 << color_bit) > 0 {
                        color_code |= 0x02;
                    }

                    if data1 & (1 << color_bit) > 0 {
                        color_code |= 0x01;
                    }

                    // If the current pixel color code for the given sprite is 0x00,
                    // then this is considered a "transparent" pixel, therefore nothing left to do.
                    if color_code == 0x00 {
                        continue;
                    }

                    // TODO: Document what this logic does
                    let mut color_palette_addr: usize = 0xFF48;

                    if sprite.get_attributes() & (1 << 4) > 0 {
                        color_palette_addr = 0xFF49;
                    }

                    let x: usize = sprite.get_x().wrapping_add(tile_pixel as u8) as usize;

                    // If the sprite's x pixel is out of the bounds of the screen, no reason to render it.
                    // Also applies if the current scanline is in v-blank mode.
                    if x > 159 || current_scanline > 143 {
                        continue;
                    }

                    // Determine if the background / window has priority over this sprite.
                    // If true, background pixel should not be overwritten, unless the pixel color
                    // at that coordinate is 0x00 (White).
                    if sprite.bg_has_priority()
                        && self.pixel_encodings[current_scanline as usize][x] != ppu::Pixel::White
                    {
                        continue;
                    }

                    let pixel_rgb =
                        PPU::determine_pixel_rgb(memory, color_palette_addr, color_code);

                    self.pixels[current_scanline as usize][x] = pixel_rgb;
                }
            }
        }
    }

    fn draw_scaline(&mut self, lcdc: u8, memory: &Arc<sync::Mutex<memory::Memory>>) {
        if lcdc & LCDC_BG_WINDOW_ENABLE_MASK > 0 {
            self.render_tiles(memory);
        }

        if lcdc & LCDC_OBJ_ENABLE_MASK > 0 {
            self.render_sprites(lcdc, memory);
        }
    }

    fn set_lcdc_status(
        &mut self,
        lcdc: u8,
        stat: u8,
        current_scanline: u8,
        ly: u8,
        lyc: u8,
        memory: &Arc<sync::Mutex<memory::Memory>>,
        interrupt_bus: &Arc<sync::Mutex<interrupt::Bus>>,
    ) {
        if lcdc & LCDC_ENABLE_MASK == 0 {
            self.scanline_counter = stat::MAX_SCANLINE_COUNT;

            // Reset the LY register
            memory
                .lock()
                .unwrap()
                .write(io_registers::LCD_LY_ADDR, 0x00);

            // Reset the STAT register to 1111 1100
            memory
                .lock()
                .unwrap()
                .write(io_registers::LCD_STAT_ADDR, stat & !stat::MODE_MASK);

            // Exit pre-emptively, since LCD is disabled
            return;
        }

        let (new_stat, requires_interrupt) = StatUpdater::new(stat)
            .process_vblank(current_scanline)
            .process_oam_search(self.scanline_counter)
            .process_pixel_transfer(self.scanline_counter)
            .process_hblank()
            .process_ly_lyc(ly, lyc)
            .build();

        memory
            .lock()
            .unwrap()
            .write(io_registers::LCD_STAT_ADDR, new_stat);
        if requires_interrupt {
            interrupt_bus
                .lock()
                .unwrap()
                .request(interrupt::Interrupt::LcdStat);
        }
    }

    pub fn determine_pixel_rgb(
        memory: &Arc<sync::Mutex<memory::Memory>>,
        color_palette_addr: usize,
        pixel_color_encoding: u8,
    ) -> ppu::Pixel {
        let color_palette = memory.lock().unwrap().dma_read(color_palette_addr).unwrap();

        // Depending on the current color palette that is in the PALETTE register,
        // these encoding translate to different colors / shades of gray.
        let palette_00 = (color_palette) & 0b11;
        let palette_01 = (color_palette >> 2) & 0b11;
        let palette_10 = (color_palette >> 4) & 0b11;
        let palette_11 = (color_palette >> 6) & 0b11;

        // The pixel value first needs to be mapped to the color palette
        // defined in the PALETTE register.
        let translated_color: u8 = match pixel_color_encoding {
            0b00 => palette_00,
            0b01 => palette_01,
            0b10 => palette_10,
            0b11 => palette_11,
            _ => panic!(
                "invalid color encoding when rendering scanline: {}",
                pixel_color_encoding
            ),
        };

        // Only then can we properly determine the actual pixel color to render.
        let pixel = match translated_color {
            0b00 => ppu::Pixel::White,
            0b01 => ppu::Pixel::LightGray,
            0b10 => ppu::Pixel::DarkGray,
            0b11 => ppu::Pixel::Black,
            _ => panic!(
                "invalid color encoding when rendering scanline: {}",
                pixel_color_encoding
            ),
        };

        return pixel;
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
    fn render_tiles(&mut self, memory: &Arc<sync::Mutex<memory::Memory>>) {
        let current_scanline = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_LY_ADDR)
            .unwrap();
        if current_scanline > 144 {
            return;
        }

        let lcdc = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_CONTROL_ADDR)
            .unwrap();
        let scroll_x = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_SCX_ADDR)
            .unwrap();
        let scroll_y = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_SCY_ADDR)
            .unwrap();
        let win_x = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_WINX_ADDR)
            .unwrap()
            .wrapping_sub(7); // TODO: Explain the sub 7
        let win_y = memory
            .lock()
            .unwrap()
            .dma_read(memory::io_registers::LCD_WINY_ADDR)
            .unwrap();

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

        let mut is_in_window_y = false;
        if window_enabled(lcdc) && current_scanline >= win_y {
            is_in_window_y = true;
        }

        // Main loop through each 160 pixels of the current scanline we are rendering
        for pixel_iter in 0..ppu::NATIVE_SCREEN_WIDTH as u8 {
            let mut pixel_y: u8 = current_scanline.wrapping_add(scroll_y);
            let mut pixel_x: u8 = pixel_iter.wrapping_add(scroll_x);

            // Check if our current coordinate is inside a window.
            // If true, then we need to render the window instead of the background.
            let mut is_in_window_x = false;
            if window_enabled(lcdc) && pixel_iter >= win_x {
                is_in_window_x = true;
            }

            if is_in_window_x && is_in_window_y {
                pixel_x = pixel_iter;
                pixel_y = current_scanline;
            }

            // The row of the tile to render.
            // Since the tile map is made up of a 32x32 grid of tiles, this value is determined by
            // dividing the current pixel's y position by 8 -> gives us the byte offset within a tile, and then
            // multiplying by 32 -> to advance the memory pointer to the correct row of tiles within the 32 rows.
            let tile_map_y: usize = (pixel_y as usize) / 8 * 32;

            let tile_map_x: usize = (pixel_x / 8).into();
            let tile_id_address: usize = tile_map_ptr + tile_map_x + tile_map_y;
            let mut tile_id = memory.lock().unwrap().dma_read(tile_id_address).unwrap();

            let tile_line_offset: usize = ((pixel_y % 8) * 2).into();
            let mut tile_data_address: usize = tile_data_ptr + (tile_id as usize * 16);

            if !uses_unsigned_id && (tile_id & 0x80) > 0 {
                tile_id = !tile_id;
                tile_id = tile_id.wrapping_add(1);
                tile_data_address = tile_data_ptr - (tile_id as usize * 16);
            }

            let data1 = memory
                .lock()
                .unwrap()
                .dma_read(tile_data_address + tile_line_offset)
                .unwrap();
            let data2 = memory
                .lock()
                .unwrap()
                .dma_read(tile_data_address + tile_line_offset + 1)
                .unwrap();

            let current_bit_position: usize = 7 - (pixel_x % 8) as usize;

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

            let pixel_color = PPU::determine_pixel_rgb(
                memory,
                memory::io_registers::LCD_PALETTE_ADDR,
                pixel_color_encoding,
            );

            self.pixel_encodings[current_scanline as usize][pixel_iter as usize] =
                match pixel_color_encoding {
                    0b00 => Pixel::White,
                    0b01 => Pixel::LightGray,
                    0b10 => Pixel::DarkGray,
                    0b11 => Pixel::Black,
                    _ => panic!("invalid pixel color data fetched from vram"),
                };
            self.pixels[current_scanline as usize][pixel_iter as usize] = pixel_color;
        }
    }

    pub fn get_frame_data(
        &self,
    ) -> [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT] {
        return self.pixels.clone();
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
