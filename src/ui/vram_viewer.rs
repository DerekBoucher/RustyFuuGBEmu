use std::sync::{Arc, Mutex};

use egui::{self, containers, style::Margin, ColorImage, Context, RichText, TextureHandle, Vec2};

use crate::{
    memory::{self, io_registers::LCD_CONTROL_ADDR, Memory},
    ppu::PPU,
};

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
            .exact_width(650.0)
            .show_animated(ctx, self.show, |ui| {
                let _ = containers::Frame::default()
                    .inner_margin(Margin::symmetric(5.0, 0.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("close").clicked() {
                                self.show = false;
                            }
                        });

                        let _ = egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            ui.add_space(10.0);
                            self.render_lcdc_view(ui, memory_ref);

                            ui.add_space(10.0);
                            self.render_tile_data_view(ctx, ui, memory_ref);

                            ui.add_space(10.0);
                            self.render_tile_map_view(ctx, ui, memory_ref);
                        });
                    });
            });
    }

    fn render_lcdc_view(&mut self, parent: &mut egui::Ui, memory_ref: &Arc<Mutex<Memory>>) {
        parent.push_id("lcdc_view", |ui| {
            let lcdc = memory_ref
                .lock()
                .unwrap()
                .dma_read(LCD_CONTROL_ADDR)
                .unwrap();

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

    fn render_tile_map_view(
        &mut self,
        ctx: &Context,
        parent: &mut egui::Ui,
        memory_ref: &Arc<Mutex<Memory>>,
    ) {
        parent.push_id("tile_map_view", |ui| {
            ui.label(egui::RichText::new("Tile Mapping").size(24.0));
            ui.separator();

            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);

            for y in 0..32 {
                ui.horizontal(|ui| {
                    for x in 0..32 {
                        let addr_offset: usize = (y * 32) + x;
                        let tile_id = memory_ref
                            .lock()
                            .unwrap()
                            .dma_read(0x9800 + addr_offset)
                            .unwrap();

                        let texture =
                            Self::render_tile(ctx, memory_ref, tile_id as usize, "mapping");
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
        memory_ref: &Arc<Mutex<Memory>>,
    ) {
        parent.push_id("tile_data_view", |ui| {
            ui.label(egui::RichText::new("Tile Data").size(24.0));
            ui.separator();

            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);

            for y in 0..24 {
                ui.horizontal(|ui| {
                    for x in 0..16 {
                        let tile_num = (y * 16) + x;

                        let texture = Self::render_tile(ctx, memory_ref, tile_num, "data");
                        ui.image(&texture, Vec2::new(24.0, 24.0));
                    }
                });
            }
        });
    }

    fn render_tile(
        ctx: &Context,
        memory_ref: &Arc<Mutex<Memory>>,
        tile_num: usize,
        id_suffix: &str,
    ) -> TextureHandle {
        let mut tile_rgb: [u8; 8 * 8 * 3] = [0x0; 8 * 8 * 3];
        let mut iter: usize = 0;

        for i in (0..16).step_by(2) {
            let data1 = memory_ref
                .lock()
                .unwrap()
                .dma_read(0x8000 + (tile_num * 16) + i)
                .unwrap();

            let data2 = memory_ref
                .lock()
                .unwrap()
                .dma_read(0x8000 + (tile_num * 16) + i + 1)
                .unwrap();

            for j in (0..8).rev() {
                let mut pixel_color_encoding: u8 = 0x00;

                if data2 & (1 << j) > 0 {
                    pixel_color_encoding |= 0b10;
                }

                if data1 & (1 << j) > 0 {
                    pixel_color_encoding |= 0b01;
                }

                let pixel = PPU::determine_pixel_rgb(
                    memory_ref,
                    memory::io_registers::LCD_PALETTE_ADDR,
                    pixel_color_encoding,
                );

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
            format!("tile_id_{}_{}", tile_num, id_suffix),
            image,
            egui::TextureOptions::NEAREST,
        );
    }
}
