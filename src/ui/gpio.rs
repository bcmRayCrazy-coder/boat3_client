use eframe::egui;
use crate::ui::app::ClientApp;

impl ClientApp {
    pub fn draw_gpio_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        ui.collapsing("GPIO", |ui| {
            ui.vertical(|ui| {
                if ui.button("Reset").clicked() {
                    println!("Reset GPIO");
                    let _ = self.remote.gpio_reset();
                }
            });
        });
    }
}
