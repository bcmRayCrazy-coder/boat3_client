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
                        let clone_ping_forwarding = Arc::clone(&self.ping_forwarding);
                        let mut ping_btn_enabled = false;
                        if let Ok(forwarding) = clone_ping_forwarding.try_lock() {
                            ping_btn_enabled = !*forwarding;
                        }
                        ui.add_enabled_ui(ping_btn_enabled, |ui| {
                            if ui.button("Ping").clicked() {
                                let mut clone_net = self.remote.net.clone();
                                let clone_ping = Arc::clone(&self.ping);
                                let clone_ping_forwarding = Arc::clone(&self.ping_forwarding);
                                let clone_info = Arc::clone(&self.info);
                                let ctx = ctx.clone();
                                self.runtime.spawn(async move {
                                    *clone_ping_forwarding.lock().unwrap() = true;
                                    let ping_result = clone_net.net_ping().await;
                                    let mut ping = clone_ping.lock().unwrap();
                                    let mut info = clone_info.lock().unwrap();
                                    match ping_result {
                                        Ok(result) => {
                                            *ping = Some(result);
                                            *info = Ok(None);
                                        }
                                        Err(err) => {
                                            *ping = None;
                                            *info = Err(err.to_string());
                                        }
                                    }
                                    *clone_ping_forwarding.lock().unwrap() = false;
                                    ctx.request_repaint();
                                });
                            }
                        });
                        let clone_ping = Arc::clone(&self.ping);
                        if let Ok(ping) = clone_ping.try_lock() {
                            if let Some(ping) = ping.clone() {
                                ui.label(format!("Ping {}ms", ping.as_millis()));
                            }
                        }
                    });
                });
            });
    }
}
