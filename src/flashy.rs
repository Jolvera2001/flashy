use eframe::{App, Frame};
use egui::Context;
use poll_promise::Promise;
use sqlx::SqlitePool;
use crate::flashy_events::FlashyEvents;

pub struct Flashy {
    db_pool: SqlitePool,
    pub current_operation: Option<Promise<FlashyEvents>>
}

impl Flashy {
    pub fn new(cc: &eframe::CreationContext<'_>, db_pool: SqlitePool) -> Self {
        Self {
            db_pool,
            current_operation: None
        }
    }
}

impl App for Flashy {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::containers::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            ui.heading("Flashy");
        });
    }
}