use crate::remote::controller::RemoteController;
use eframe::egui;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn create_ui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
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
        egui::FontData::from_static(include_bytes!("../../static/msyh.ttc")).into(),
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

pub struct ClientApp {
    pub runtime: tokio::runtime::Runtime,
    pub remote: RemoteController,

    pub ping: Arc<Mutex<Option<Duration>>>,
    pub ping_forwarding:Arc<Mutex<bool>>,

    pub info: Arc<Mutex<Result<Option<String>, String>>>,

    info_frame_color: Option<egui::Color32>,
}

impl ClientApp {
    pub fn default(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        setup_fonts(&cc.egui_ctx);
        Self {
            runtime,
            remote: RemoteController::new(),

            ping: Arc::new(Mutex::new(None)),
            ping_forwarding: Arc::new(Mutex::new(false)),

            info: Arc::new(Mutex::new(Ok(None))),

            info_frame_color: None,
        }
    }
}

impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar_panel").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Boat3 Client");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_connection_ui(ctx, frame, ui);
            ui.separator();
            ui.horizontal(|ui| {
                self.draw_gpio_ui(ctx, frame, ui);
                ui.separator();
                self.draw_camera_ui(ctx, frame, ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_info_bar_panel")
            .frame({
                let mut frame = egui::Frame::new();
                if let Some(color) = self.info_frame_color {
                    frame = frame.fill(color);
                }
                frame
            })
            .show(ctx, |ui| {
                self.info_frame_color = self.draw_info_ui(ctx, frame, ui);
            });
    }
}
