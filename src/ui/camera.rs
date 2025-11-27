use crate::ui::app::ClientApp;
use eframe::egui;

impl ClientApp {
    pub fn draw_camera_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        egui::CollapsingHeader::new("Camera")
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Todo");
                    if ui.button("Fetch Image").clicked() {
                        println!("Fetch Image Todo");
                    }
                });
            });
    }
}
