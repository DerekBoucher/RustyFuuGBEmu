use crate::memory::io_registers::{
    LCD_CONTROL_ADDR, LCD_PALETTE_ADDR, LCD_SCX_ADDR, LCD_SCY_ADDR, LCD_WINX_ADDR, LCD_WINY_ADDR,
};
use crate::memory::Memory;
use crate::ppu::Pixel;
use core::panic;
use egui::{
    self, containers, style::Margin, Color32, ColorImage, Context, Label, RichText, TextureHandle,
    Vec2,
};
use std::sync::{Arc, Mutex};

pub struct Ui {
    show: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self { show: false }
    }
    pub fn show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn render(&mut self, ctx: &egui::Context, memory_ref: &Arc<Mutex<Memory>>) {
        let _ = egui::SidePanel::new(egui::panel::Side::Right, egui::Id::new("vram_viewer"))
            .min_width(650.0)
            .resizable(true)
            .show_animated(ctx, self.show, |ui| {
                let _ = containers::Frame::default()
                    .inner_margin(Margin::symmetric(5.0, 0.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("close").clicked() {
                                self.show = false;
                            }
                        });

                        let memory = memory_ref.lock().unwrap();
                        let vram = memory.borrow_vram();
                        let io_reg = memory.borrow_io_reg();

                        let _ = egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            ui.add_space(10.0);
                            self.render_lcdc_view(ui, &io_reg);

                            ui.add_space(10.0);
                            self.render_tile_data_view(ctx, ui, &vram, &io_reg);

                            ui.add_space(10.0);
                            self.render_tile_map_view(ctx, ui, &vram, &io_reg);

                            ui.add_space(10.0);
                            self.render_window_tile_map_view(ctx, ui, &vram, &io_reg);
                        });
                    });
            });
    }

    fn render_lcdc_view(&mut self, parent: &mut egui::Ui, io_reg: &[u8; 0x80]) {
        parent.push_id("lcdc_view", |ui| {
            let lcdc = io_reg[LCD_CONTROL_ADDR - 0xFF00];

            let bg_win_enabled: &str = match lcdc & (1 << 0) > 0 {
                true => "Enabled",
                false => "Disabled",
            };

            let obj_enabled: &str = match lcdc & (1 << 1) > 0 {
                true => "Enabled",
                false => "Disabled",
            };

            let obj_size: &str = match lcdc & (1 << 2) > 0 {
                true => "8x16",
                false => "8x8",
            };

            let bg_win_tile_map_range: &str = match lcdc & (1 << 3) > 0 {
                true => "0x9C00-0x9FFF",
                false => "0x9800-0x9BFF",
            };

            let addressing_mode: &str = match lcdc & (1 << 4) > 0 {
                true => "Unsigned / BG & Window Range: 0x8000 - 0x8FFF",
                false => "Signed / BG & Window Range: 0x8800 - 0x97FF",
            };

            let window_enabled: &str = match lcdc & (1 >> 5) > 0 {
                true => "Enabled",
                false => "Disabled",
            };

            let window_tile_map_range: &str = match lcdc & (1 >> 6) > 0 {
                true => "0x9C00-0x9FFF",
                false => "0x9800-0x9BFF",
            };

            let lcd_enabled: &str = match lcdc & (1 << 7) > 0 {
                true => "Enabled",
                false => "Disabled",
            };

            ui.label(RichText::new("LCDC Register").size(24.0));
            ui.separator();
            ui.label(format!("0 - BG & Win: {}", bg_win_enabled));
            ui.label(format!("1 - Objects: {}", obj_enabled));
            ui.label(format!("2 - Object Size: {}", obj_size));
            ui.label(format!("3 - Tile Map Range: {}", bg_win_tile_map_range));
            ui.label(format!("4 - Addressing mode: {}", addressing_mode));
            ui.label(format!("5 - Window: {}", window_enabled));
            ui.label(format!(
                "6 - Window Tile Map Range: {}",
                window_tile_map_range
            ));
            ui.label(format!("7 - LCD / PPU: {}", lcd_enabled));
        });
    }

    fn render_window_tile_map_view(
        &mut self,
        ctx: &Context,
        parent: &mut egui::Ui,
        vram: &[u8; 0x2000],
        io_reg: &[u8; 0x80],
    ) {
        let winx = io_reg[LCD_WINX_ADDR - 0xFF00].wrapping_sub(7);
        let winy = io_reg[LCD_WINY_ADDR - 0xFF00];
        let lcdc = io_reg[LCD_CONTROL_ADDR - 0xFF00];

        parent.push_id("win_tile_map_view", |ui| {
            ui.label(RichText::new("Window Tile Mapping").size(24.0));
            ui.separator();

            ui.label(format!("Window X: {}", winx));
            ui.label(format!("Window Y: {}", winy));

            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);
            for y in 0..33 {
                ui.horizontal(|ui| {
                    for x in 0..33 {
                        if x == 0 && y == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(""));
                            continue;
                        }

                        if x == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(format!("{}", y)));
                            continue;
                        }

                        if y == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(format!("{}", x)));
                            continue;
                        }

                        let base_addr = match lcdc & (1 << 6) > 0 {
                            true => 0x9C00,
                            false => 0x9800,
                        };
                        let offset: usize = ((y - 1) * 32) + (x - 1);

                        let tile_id = vram[(base_addr + offset) - 0x8000] as usize;
                        let texture = Self::render_tile(ctx, &vram, &io_reg, tile_id);
                        ui.image(&texture, Vec2::new(24.0, 24.0));
                    }
                });
            }
        });
    }

    fn render_tile_map_view(
        &mut self,
        ctx: &Context,
        parent: &mut egui::Ui,
        vram: &[u8; 0x2000],
        io_reg: &[u8; 0x80],
    ) {
        let lcdc = io_reg[LCD_CONTROL_ADDR - 0xFF00];
        let scx = io_reg[LCD_SCX_ADDR - 0xFF00];
        let scy = io_reg[LCD_SCY_ADDR - 0xFF00];

        parent.push_id("tile_map_view", |ui| {
            ui.label(egui::RichText::new("Background Tile Mapping").size(24.0));
            ui.separator();

            ui.label(format!("Scroll X: {}", scx));
            ui.label(format!("Scroll Y: {}", scy));

            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);

            for y in 0..33 {
                ui.horizontal(|ui| {
                    for x in 0..33 {
                        if x == 0 && y == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(""));
                            continue;
                        }

                        if x == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(format!("{}", y)));
                            continue;
                        }

                        if y == 0 {
                            ui.add_sized(Vec2::new(24.0, 24.0), Label::new(format!("{}", x)));
                            continue;
                        }

                        let base_addr = match lcdc & (1 << 3) > 0 {
                            true => 0x9C00,
                            false => 0x9800,
                        };
                        let offset: usize = ((y - 1) * 32) + (x - 1);

                        let tile_id = vram[(base_addr + offset) - 0x8000] as usize;
                        let texture = Self::render_tile(ctx, &vram, &io_reg, tile_id);
                        ui.image(&texture, Vec2::new(24.0, 24.0));
                    }
                });
            }
        });
    }

    fn render_tile_data_view(
        &mut self,
        ctx: &Context,
        parent: &mut egui::Ui,
        vram: &[u8; 0x2000],
        io_reg: &[u8; 0x80],
    ) {
        parent.push_id("tile_data_view", |ui| {
            ui.label(egui::RichText::new("Tile Data").size(24.0));
            ui.separator();

            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);

            for y in 0..25 {
                ui.horizontal(|ui| {
                    for x in 0..17 {
                        if x == 0 && y == 0 {
                            // White space
                            ui.add_sized(Vec2::new(24.0, 24.0), egui::Label::new(""));
                            continue;
                        }

                        if x == 0 {
                            ui.add_sized(
                                Vec2::new(24.0, 24.0),
                                Label::new(RichText::new(format!("{}", y)).color(Color32::WHITE)),
                            );
                            continue;
                        }

                        if y == 0 {
                            ui.add_sized(
                                Vec2::new(24.0, 24.0),
                                Label::new(RichText::new(format!("{}", x)).color(Color32::WHITE)),
                            );

                            continue;
                        }

                        let tile_id = ((y - 1) * 16) + (x - 1);

                        let texture = Self::render_tile(ctx, &vram, &io_reg, tile_id);
                        ui.image(&texture, Vec2::new(24.0, 24.0));
                    }
                });
            }
        });
    }

    fn render_tile(
        ctx: &Context,
        vram: &[u8; 0x2000],
        io_reg: &[u8; 0x80],
        tile_id: usize,
    ) -> TextureHandle {
        let mut tile_rgb: [u8; 8 * 8 * 3] = [0x0; 8 * 8 * 3];
        let mut iter: usize = 0;

        let lcdc = io_reg[LCD_CONTROL_ADDR - 0xFF00];
        let is_unsigned = lcdc & (1 << 4) > 0;

        let effective_addr: usize = match is_unsigned {
            true => 0x8000 + (tile_id * 16),
            false => match tile_id < 128 {
                true => 0x9000 + (tile_id * 16),
                false => 0x8800 + ((tile_id - 128) * 16),
            },
        };

        for i in (0..16).step_by(2) {
            let data1 = vram[(effective_addr + i) - 0x8000];
            let data2 = vram[(effective_addr + i + 1) - 0x8000];

            for j in (0..8).rev() {
                let mut pixel_color_encoding: u8 = 0x00;

                if data2 & (1 << j) > 0 {
                    pixel_color_encoding |= 0b10;
                }

                if data1 & (1 << j) > 0 {
                    pixel_color_encoding |= 0b01;
                }

                let color_palette = io_reg[LCD_PALETTE_ADDR - 0xFF00];

                let palette_00 = (color_palette) & 0b11;
                let palette_01 = (color_palette >> 2) & 0b11;
                let palette_10 = (color_palette >> 4) & 0b11;
                let palette_11 = (color_palette >> 6) & 0b11;

                let translated_color = match pixel_color_encoding {
                    0b00 => palette_00,
                    0b01 => palette_01,
                    0b10 => palette_10,
                    0b11 => palette_11,
                    _ => panic!("invalid pixel encoding"),
                };

                let pixel = match translated_color {
                    0b00 => Pixel::White,
                    0b01 => Pixel::LightGray,
                    0b10 => Pixel::DarkGray,
                    0b11 => Pixel::Black,
                    _ => panic!("invalid pixel encoding"),
                };

                let rgb = pixel.to_rgb_u8();
                tile_rgb[iter] = rgb.0;
                iter += 1;
                tile_rgb[iter] = rgb.1;
                iter += 1;
                tile_rgb[iter] = rgb.2;
                iter += 1;
            }
        }

        let image = ColorImage::from_rgb([8, 8], &tile_rgb);
        return ctx.load_texture(
            format!("tile_id_{}", tile_id),
            image,
            egui::TextureOptions::NEAREST,
        );
    }
}
