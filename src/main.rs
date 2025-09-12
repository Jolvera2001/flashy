extern crate core;

use eframe::egui;
use crate::flashy::Flashy;

mod flashy;
mod flashy_events;
mod models;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Flashy",
        options,
        Box::new(|cc| Ok(Box::new(Flashy::new()))),
    )
}
