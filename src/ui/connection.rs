use crate::ui::app::ClientApp;
use eframe::egui;
use std::sync::Arc;

impl ClientApp {
    pub fn draw_connection_ui(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        egui::CollapsingHeader::new("Connection")
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let address_label = ui.label("Address");
                        ui.text_edit_singleline(&mut self.remote.net.address)
                            .labelled_by(address_label.id);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Reset").clicked() {
                            println!("Reset all");
                            let _ = self.remote.gpio_reset();
                        }
                        if ui.button("Ping").clicked() {
                            let mut clone_net = self.remote.net.clone();
                            let clone_ping = Arc::clone(&self.ping);
                            let clone_error = Arc::clone(&self.error);
                            let ctx = ctx.clone();
                            self.runtime.spawn(async move {
                                let ping_result = clone_net.net_ping().await;
                                let mut ping = clone_ping.lock().unwrap();
                                let mut error = clone_error.lock().unwrap();
                                match ping_result {
                                    Ok(result) => {
                                        *ping = Some(result);
                                        *error = None;
                                    }
                                    Err(mut err) => {
                                        *ping = None;
                                        *error = Some(err.to_string());
                                    }
                                }
                                ctx.request_repaint();
                            });
                        }
                        let clone_ping = Arc::clone(&self.ping);
                        if let Some(p) = *clone_ping.lock().unwrap() {
                            ui.label(format!("Ping {}ms", p.as_millis()));
                        }
                    });
                });
            });
    }
}
