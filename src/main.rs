#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use boat3_client::ui;

#[tokio::main]
async fn main() -> eframe::Result {
    ui::app::create_ui()
}
