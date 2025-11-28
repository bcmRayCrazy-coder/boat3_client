use std::sync::{Arc, Mutex};

use crate::{
    remote::protocol::gpio::{GPIOMode, GPIOValue, GPIOValueLevel, RemoteGPIO},
    ui::app::ClientApp,
};
use eframe::egui;

#[derive(Debug, Clone)]
pub struct ClientAppGPIO {
    pub list: Arc<Mutex<Vec<RemoteGPIO>>>,
    pub add_mode: GPIOMode,
    pub add_pin: u8,
    pub add_forwarding: Arc<Mutex<bool>>,
    pub set_pin: u8,
    pub set_mode: GPIOMode,
    pub set_value: GPIOValue,
    pub set_forwarding: Arc<Mutex<bool>>,
}

impl ClientAppGPIO {
    pub fn new() -> Self {
        Self {
            list: Arc::new(Mutex::new(Vec::new())),
            add_mode: GPIOMode::INPUT,
            add_pin: 0,
            add_forwarding: Arc::new(Mutex::new(false)),
            set_pin: 0,
            set_mode: GPIOMode::UNKNOWN,
            set_value: GPIOValue::NONE,
            set_forwarding: Arc::new(Mutex::new(false)),
        }
    }
}

impl ClientApp {
    pub fn draw_gpio_ui(
        &mut self,
        ctx: &egui::Context,
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
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Add Pin");
                                    ui.add(egui::Slider::new(&mut self.gpio.add_pin, 0..=255));
                                });
                                egui::ComboBox::from_label("Mode")
                                    .selected_text(format!("{}", self.gpio.add_mode))
                                    .show_ui(ui, |ui| {
                                        let mode_presets = [
                                            GPIOMode::INPUT,
                                            GPIOMode::OUTPUT,
                                            GPIOMode::ANALOG,
                                            GPIOMode::PWM,
                                        ];
                                        for mode in &mode_presets {
                                            ui.selectable_value(
                                                &mut self.gpio.add_mode,
                                                mode.clone(),
                                                format!("{}", *mode),
                                            );
                                        }
                                    });

                                ui.horizontal(|ui| {
                                    let clone_add_forwarding =
                                        Arc::clone(&self.gpio.add_forwarding);
                                    if let Ok(add_forwarding) = clone_add_forwarding.try_lock() {
                                        ui.add_enabled_ui(!(*add_forwarding), |ui| {
                                            if ui.button("+").clicked() {
                                                let mut clone_remote = self.remote.clone();
                                                let clone_info = Arc::clone(&self.info);
                                                let clone_forwarding =
                                                    Arc::clone(&self.gpio.add_forwarding);
                                                let clone_list = Arc::clone(&self.gpio.list);
                                                let clone_gpio = self.gpio.clone();
                                                let ctx = ctx.clone();
                                                self.runtime.spawn(async move {
                                                    *clone_forwarding.lock().unwrap() = true;
                                                    let config_result = clone_remote
                                                        .gpio_config(&RemoteGPIO::new(
                                                            clone_gpio.add_pin,
                                                            clone_gpio.add_mode,
                                                        ))
                                                        .await;

                                                    match config_result {
                                                        Ok(_) => {
                                                            let mut list =
                                                                clone_list.lock().unwrap();
                                                            list.push(RemoteGPIO::new(
                                                                clone_gpio.add_pin,
                                                                clone_gpio.add_mode,
                                                            ));
                                                            if let Ok(mut info) = clone_info.lock()
                                                            {
                                                                *info = Ok(Some(format!(
                                                                    "Added pin {}",
                                                                    clone_gpio.add_pin
                                                                )))
                                                            }
                                                        }
                                                        Err(err) => {
                                                            if let Ok(mut info) = clone_info.lock()
                                                            {
                                                                *info = Err(format!(
                                                                    "Unable to add pin: {}",
                                                                    err.to_string()
                                                                ))
                                                            }
                                                        }
                                                    }
                                                    *clone_forwarding.lock().unwrap() = false;
                                                    ctx.request_repaint();
                                                });
                                            }
                                        });

                                        match self.gpio.add_mode {
                                            GPIOMode::UNKNOWN => {}
                                            GPIOMode::OUTPUT => {}
                                            GPIOMode::INPUT => {}
                                            GPIOMode::ANALOG => todo!(),
                                            GPIOMode::PWM => todo!(),
                                        }
                                    }
                                    if ui.button("x All").clicked() {
                                        // TODO: Delete All
                                    }
                                    if cfg!(target_os = "windows") {
                                        if ui.link("Pinout").clicked() {
                                            std::process::Command::new("cmd.exe")
                                                .arg("/C")
                                                .arg("start")
                                                .arg("https://pinout.xyz")
                                                .spawn()
                                                .expect("Unable to open link");
                                        }
                                    }
                                });
                            })
                        });
                        ui.separator();
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Write Pin");
                                    ui.add(egui::Slider::new(&mut self.gpio.set_pin, 0..=255));
                                });
                                let clone_list = Arc::clone(&self.gpio.list);
                                let mut pin_tip: Option<String> =
                                    Some("Pin not added yet".to_owned());
                                if let Ok(list) = clone_list.try_lock() {
                                    for pin in list.iter() {
                                        if pin.pin == self.gpio.set_pin {
                                            self.gpio.set_mode = pin.mode.clone();
                                            match pin.mode {
                                                GPIOMode::OUTPUT => {
                                                    pin_tip = None;
                                                    egui::ComboBox::from_label("Value")
                                                        .selected_text(
                                                            self.gpio.set_value.to_string(),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            ui.selectable_value(
                                                                &mut self.gpio.set_value,
                                                                GPIOValue::LEVEL(
                                                                    GPIOValueLevel::LOW,
                                                                ),
                                                                "LOW",
                                                            );
                                                            ui.selectable_value(
                                                                &mut self.gpio.set_value,
                                                                GPIOValue::LEVEL(
                                                                    GPIOValueLevel::HIGH,
                                                                ),
                                                                "HIGH",
                                                            );
                                                        });
                                                }
                                                _ => pin_tip = Some("Unwriteable Pin".to_owned()),
                                            }
                                            break;
                                        }
                                    }
                                }
                                if let Some(tip) = pin_tip {
                                    ui.label(tip);
                                } else {
                                    let clone_set_forwarding =
                                        Arc::clone(&self.gpio.set_forwarding);
                                    if let Ok(set_forwarding) = clone_set_forwarding.try_lock() {
                                        ui.add_enabled_ui(!*set_forwarding, |ui| {
                                            if ui.button("Write").clicked() {
                                                let clone_info = Arc::clone(&self.info);
                                                let clone_forwarding =
                                                    Arc::clone(&self.gpio.set_forwarding);
                                                let clone_list = Arc::clone(&self.gpio.list);
                                                let mut clone_remote = self.remote.clone();
                                                let clone_ctx = ctx.clone();
                                                let target_gpio = RemoteGPIO {
                                                    pin: self.gpio.set_pin,
                                                    mode: self.gpio.set_mode,
                                                    value: self.gpio.set_value,
                                                };
                                                self.runtime.spawn(async move {
                                                    *clone_forwarding.lock().unwrap() = true;
                                                    let result =
                                                        clone_remote.gpio_set(&target_gpio).await;
                                                    *clone_forwarding.lock().unwrap() = false;
                                                    if let Ok(mut info) = clone_info.try_lock() {
                                                        if let Err(err) = result {
                                                            *info = Err(err.to_string());
                                                        } else {
                                                            *info = Ok(Some(format!(
                                                                "Set pin {} to {}",
                                                                target_gpio.pin, target_gpio.value
                                                            )));
                                                            if let Ok(mut list) = clone_list.lock()
                                                            {
                                                                *list = list
                                                                    .iter()
                                                                    .map(|pin| {
                                                                        if pin.pin
                                                                            == target_gpio.pin
                                                                        {
                                                                            let mut new_pin =
                                                                                pin.clone();
                                                                            new_pin.value =
                                                                                target_gpio.value;
                                                                            return new_pin;
                                                                        }
                                                                        pin.clone()
                                                                    })
                                                                    .collect();
                                                            }
                                                        }
                                                    }
                                                    clone_ctx.request_repaint();
                                                });
                                            }
                                        });
                                    }
                                }
                            });
                        })
                    });

                    egui::ScrollArea::vertical()
                        .min_scrolled_height(150.0)
                        .show(ui, |ui| {
                            egui::Grid::new("gpio_input_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("Pin");
                                    ui.label("Type");
                                    ui.label("Value");
                                    ui.end_row();

                                    if let Ok(list) = Arc::clone(&self.gpio.list).try_lock() {
                                        for pin in &*list {
                                            ui.label(pin.pin.to_string());
                                            ui.label(format!("{}", pin.mode));

                                            ui.label(pin.value.to_string());
                                            ui.horizontal(|ui| {
                                                if ui.button("x").clicked() {
                                                    let clone_list = Arc::clone(&self.gpio.list);
                                                    let clone_info = Arc::clone(&self.info);
                                                    let clone_pin = pin.clone();
                                                    let _clone_remote = self.remote.clone();
                                                    self.runtime.spawn(async move {
                                                        // TODO: Delet at remote
                                                        if let Ok(mut list) = clone_list.try_lock()
                                                        {
                                                            list.retain(|current_pin| {
                                                                current_pin.pin != clone_pin.pin
                                                            });
                                                        }
                                                        if let Ok(mut info) = clone_info.try_lock()
                                                        {
                                                            *info = Ok(Some(format!(
                                                                "Delete Pin {}",
                                                                clone_pin.pin
                                                            )));
                                                        }
                                                    });
                                                }
                                            });
                                            ui.end_row();
                                        }
                                    }
                                });
                        });
                });
            });
    }
}
