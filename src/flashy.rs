use crate::flashy_events::FlashyEvents;
use crate::models;
use crate::models::user::User;
use eframe::{App, Frame};
use egui::Context;
use poll_promise::Promise;
use sqlx::SqlitePool;

pub struct Flashy {
    // connections/services/events
    db_pool: SqlitePool,
    pub current_operation: Option<Promise<FlashyEvents>>,

    // dialogs
    pub test_dialog_open: bool,

    // state
    pub current_user: Option<User>,
}

impl Flashy {
    pub fn new(cc: &eframe::CreationContext<'_>, db_pool: SqlitePool) -> Self {
        Self {
            db_pool,
            current_operation: None,
            test_dialog_open: false,
            current_user: None,
        }
    }
}

impl App for Flashy {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.handle_events(ctx);

        egui::containers::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            menu_bar(self, ui, ctx);
        });

        self.check_login_register_dialog(ctx);
    }
}

fn menu_bar(app: &mut Flashy, ui: &mut egui::Ui, ctx: &Context) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("User", |ui| {
            if let Some(user) = &app.current_user {
                ui.label(format!("Welcome {}", user.name));
                ui.separator();
                if ui.button("Overview").clicked() {
                    // something
                }
                if ui.button("Logout").clicked() {
                    app.current_user = None;
                };
            } else {
                ui.label("Not logged in");
                ui.separator();
                if ui.button("Login").clicked() {
                    app.test_dialog_open = true;
                };
            }
        });
    });
}
