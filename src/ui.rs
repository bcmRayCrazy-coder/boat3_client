use eframe::egui;
use crate::remote;

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
    connection_address: String,
    connection_state: remote::net::ConnectionState,
    connection_ping: Option<u32>,
}

impl ClientApp {
    fn default(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            connection_address: "0.0.0.0:10230".to_owned(),
            connection_state: remote::net::ConnectionState::Disconnected,
            connection_ping: None,
        }
    }

    fn draw_connection_ui(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        ui.collapsing("Connection", |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let address_label = ui.label("Address");
                    ui.text_edit_singleline(&mut self.connection_address)
                        .labelled_by(address_label.id);
                });
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.connection_state == remote::net::ConnectionState::Connected, |ui| {
                        if ui.button("Reconnect").clicked() {
                            println!("Click Reconnect");
                        }
                    });
                    ui.add_enabled_ui(self.connection_state != remote::net::ConnectionState::Connecting, |ui| {
                        if ui
                            .button(match self.connection_state {
                                remote::net::ConnectionState::Connected => "Disconnect",
                                remote::net::ConnectionState::Disconnected => "Connect",
                                remote::net::ConnectionState::Connecting => "Connecting...",
                            })
                            .clicked()
                        {
                            // TODO
                            match self.connection_state {
                                remote::net::ConnectionState::Connected => {
                                    println!("Click Disconnect");
                                    self.connection_state = remote::net::ConnectionState::Disconnected
                                }
                                remote::net::ConnectionState::Connecting => {}
                                remote::net::ConnectionState::Disconnected => {
                                    println!("Click Connect");
                                    self.connection_state = remote::net::ConnectionState::Connected
                                }
                            }
                        }
                    });
                    let ping_text = match self.connection_ping {
                        Some(n) => format!("Ping {} ms", n.to_string()),
                        None => match self.connection_state {
                            remote::net::ConnectionState::Connected => "Ping NaN".to_owned(),
                            remote::net::ConnectionState::Connecting => "Connecting".to_owned(),
                            remote::net::ConnectionState::Disconnected => "Disconnected".to_owned(),
                        },
                    };
                    ui.label(ping_text);
                });
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
