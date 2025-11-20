#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use boat3_client::ui;

fn main() -> eframe::Result {
    ui::create_ui()
}