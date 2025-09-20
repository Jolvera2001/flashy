use crate::flashy_events::FlashyEvents;
use crate::models::user::User;
use eframe::{App, Frame};
use egui::{Context, Ui};
use poll_promise::Promise;
use sqlx::SqlitePool;
use crate::models::login_register_dto::LoginRegisterDto;
use crate::models::recurrence::Recurrence;

pub struct Flashy {
    // connections/services/events
    db_pool: SqlitePool,
    pub current_operation: Option<Promise<FlashyEvents>>,

    // dialogs/forms
    pub auth_form_dialog: bool,
    pub auth_form: LoginRegisterDto,

    // state
    pub current_user: Option<User>,
    pub recurrences: Option<Vec<Recurrence>>,
    pub chosen_recurrence: Option<Recurrence>,
}

impl Flashy {
    pub fn new(cc: &eframe::CreationContext<'_>, db_pool: SqlitePool) -> Self {
        Self {
            db_pool,
            current_operation: None,
            auth_form_dialog: false,
            auth_form: LoginRegisterDto::default(),
            current_user: None,
            recurrences: None,
            chosen_recurrence: None,
        }
    }

    pub fn menu_bar(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("User", |ui| {
                if let Some(user) = &self.current_user {
                    ui.label(format!("Welcome {}", user.name));
                    ui.separator();
                    if ui.button("Overview").clicked() {
                        // something
                    }
                    if ui.button("Logout").clicked() {
                        self.current_user = None;
                    };
                } else {
                    ui.label("Not logged in");
                    ui.separator();
                    if ui.button("Login").clicked() {
                        self.auth_form_dialog = true;
                    };
                }
            });
        });
    }
}

impl App for Flashy {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.handle_events(ctx);

        egui::containers::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            self.menu_bar(ui, ctx);
        });

        self.check_login_register_dialog(ctx);
    }
}
