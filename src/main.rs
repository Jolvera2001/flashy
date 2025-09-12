extern crate core;

use crate::flashy::Flashy;
use eframe::egui;
use sqlx::SqlitePool;
use std::env;
use std::str::FromStr;

mod flashy;
mod flashy_events;
mod models;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };
    let db_options = sqlx::sqlite::SqliteConnectOptions::from_str("test.db")
        .expect("Issue Creating Database")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options).await.unwrap();

    // run migrations here

    eframe::run_native(
        "Flashy",
        options,
        Box::new(|cc| Ok(Box::new(Flashy::new(cc, pool)))),
    )
}
