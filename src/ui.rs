use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eframe::egui;

use crate::remote::{controller::RemoteController, net::RemoteError};

pub fn create_ui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Boat3 Client",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ClientApp::default(cc)))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "msyh".to_owned(),
        egui::FontData::from_static(include_bytes!("../static/msyh.ttc")).into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "msyh".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("msyh".to_owned());

    ctx.set_fonts(fonts);
}

struct ClientApp {
    runtime: tokio::runtime::Runtime,
    remote: RemoteController,
    ping: Arc<Mutex<Option<Duration>>>,
    error: Arc<Mutex<Option<String>>>,
}

impl ClientApp {
    fn default(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        setup_fonts(&cc.egui_ctx);
        Self {
            runtime,
            remote: RemoteController::new(),
            ping: Arc::new(Mutex::new(None)),
            error: Arc::new(Mutex::new(None)),
        }
    }

    fn draw_connection_ui(
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
                                    *error = Some(err.unwrap());
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
    fn draw_gpio_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        ui.collapsing("GPIO", |ui| {
            ui.vertical(|ui| {
                ui.label("Hi");
            });
        });
    }
}

impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Boat3 Client");
            ui.separator();
            self.draw_connection_ui(ctx, frame, ui);
            ui.separator();
            self.draw_gpio_ui(ctx, frame, ui);
        });
    }
}
