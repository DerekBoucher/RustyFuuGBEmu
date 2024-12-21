use egui::Context;

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

    pub fn render(&mut self, ctx: &Context) {
        let _ = egui::SidePanel::new(egui::panel::Side::Right, egui::Id::new("cpu_viewer"))
            .min_width(650.0)
            .resizable(true)
            .show_animated(ctx, self.show, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("close").clicked() {
                        self.show = false;
                    }
                });
            });
    }
}
