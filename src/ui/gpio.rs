use crate::ui::app::ClientApp;
use eframe::egui;

impl ClientApp {
    pub fn draw_gpio_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        egui::CollapsingHeader::new("GPIO")
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Reset").clicked() {
                        println!("Reset GPIO");
                        let _ = self.remote.gpio_reset();
                    }
                    // ui.
                });
            });
    }
}
