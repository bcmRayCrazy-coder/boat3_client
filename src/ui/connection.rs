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
        ui.collapsing("Connection", |ui| {
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
                            match clone_net.net_ping().await {
                                Ok(result) => {
                                    let mut ping = clone_ping.lock().unwrap();
                                    *ping = Some(result);
                                }
                                Err(mut err) => {
                                    let mut error = clone_error.lock().unwrap();
                                    *error = Some(err.to_string());
                                }
                            }
                            ctx.request_repaint();
                        });
                    }
                    if let Ok(ping) = Arc::clone(&self.ping).try_lock() {
                        if let Some(ping) = *ping {
                            ui.label(format!("Ping {}ms", ping.as_millis()));
                        }
                    }
                });
                if let Ok(error) = Arc::clone(&self.error).try_lock() {
                    match error.clone() {
                        Some(err) => {
                            ui.label(err);
                        }
                        None => {}
                    }
                }
            });
        });
    }
}
