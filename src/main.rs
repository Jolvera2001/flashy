extern crate core;

use crate::flashy::Flashy;
use eframe::egui;
use sqlx::SqlitePool;
use std::str::FromStr;

mod flashy;
mod flashy_events;
mod models;
mod flashy_dialogs;
mod event_handler;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    let db_options =
        sqlx::sqlite::SqliteConnectOptions::from_str("sqlite:test.db")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options).await?;
    run_migrations(&pool).await?;

    eframe::run_native(
        "Flashy",
        options,
        Box::new(|cc| Ok(Box::new(Flashy::new(cc, pool)))),
    )?;

    Ok(())
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            date_created TEXT NOT NULL,
            date_updated TEXT NOT NULL,
            name TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS recurrences (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            date_created TEXT NOT NULL,
            date_updated TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            amount TEXT NOT NULL,
            circulating_date TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        )
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
