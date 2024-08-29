use egui;

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

    pub fn render(&mut self, egui_ctx: &egui::Context) {
        let _ = egui::SidePanel::new(egui::panel::Side::Right, egui::Id::new("vram_viewer"))
            .resizable(false)
            .min_width(400.0)
            .show_animated(egui_ctx, self.show, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("close").clicked() {
                        self.show = false;
                    }
                });
            });
    }
}
