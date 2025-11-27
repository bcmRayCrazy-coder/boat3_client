use std::sync::Arc;

use egui::{Color32, RichText};

use crate::ui::app::ClientApp;

impl ClientApp {
    pub fn draw_info_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) -> Option<Color32> {
        let mut color = None;
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            let mut text = RichText::new("Done");
            if let Ok(err) = Arc::clone(&self.error).try_lock() {
                if let Some(err) = err.clone() {
                    text = RichText::new(err);
                    color = Some(Color32::from_rgba_unmultiplied(255, 0, 0, 64));
                }
            }
            ui.label(text.small());
        });
        color
    }
}
