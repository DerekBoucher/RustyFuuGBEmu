use std::sync::{Arc, Mutex};

use egui::{self, ColorImage, Context, TextureHandle, Vec2};

use crate::{
    memory::{self, Memory},
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

    pub fn render(&mut self, egui_ctx: &egui::Context, memory_ref: &Arc<Mutex<Memory>>) {
        let _ = egui::SidePanel::new(egui::panel::Side::Right, egui::Id::new("vram_viewer"))
            .resizable(false)
            .exact_width(450.0)
            .show_animated(egui_ctx, self.show, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("close").clicked() {
                        self.show = false;
                    }
                });

                ui.add_space(10.0);

                self.render_tile_data_view(egui_ctx, ui, memory_ref);
            });
    }

    fn render_tile_data_view(
        &mut self,
        ctx: &Context,
        parent: &mut egui::Ui,
        memory_ref: &Arc<Mutex<Memory>>,
    ) {
        parent.label(egui::RichText::new("VRAM Data").size(24.0));
        parent.separator();

        let _ = egui::ScrollArea::new([true, true]).show(parent, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);

            for y in 0..24 {
                ui.horizontal(|ui| {
                    for x in 0..16 {
                        let tile_num = (y * 16) + x;

                        let texture = Self::render_tile(ctx, memory_ref, tile_num);
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
            format!("tile_id_{}", tile_num),
            image,
            egui::TextureOptions::NEAREST,
        );
    }
}
