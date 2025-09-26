use crate::flashy_events::{Commands, FlashyEvents};
use eframe::{App, Frame};
use egui::{Context, Ui};
use poll_promise::Promise;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::models::profile_dto::ProfileDto;
use crate::models::profile::Profile;
use crate::models::recurrence::Recurrence;
use crate::models::recurrence_dto::RecurrenceDto;

pub struct Flashy {
    // connections/services/events/channels
    pub db_pool: SqlitePool,
    pub current_operation: Option<Promise<FlashyEvents>>,
    pub command_channel: Sender<Commands>,
    pub event_channel: Receiver<FlashyEvents>,

    // dialogs/forms
    pub auth_form_dialog: bool,
    pub recurrence_dialog: bool,
    pub profile_form: ProfileDto,
    pub recurrence_form: RecurrenceDto,

    // state
    pub current_profile: Option<Profile>,
    pub recurrences: Option<Vec<Recurrence>>,
    pub chosen_recurrence: Option<Recurrence>,
}

impl Flashy {
    pub fn new(cc: &eframe::CreationContext<'_>, db_pool: SqlitePool) -> Self {
        let (command_tx, mut command_rx) = mpsc::channel::<Commands>(32);
        let (mut event_tx, event_rx) = mpsc::channel::<FlashyEvents>(32);

        tokio::spawn(async move {
            Self::handle_commands(&mut command_rx, &mut event_tx).await;
        });

        Self {
            db_pool,
            current_operation: None,
            command_channel: command_tx,
            event_channel: event_rx,
            auth_form_dialog: false,
            recurrence_dialog: false,
            profile_form: ProfileDto::default(),
            recurrence_form: RecurrenceDto::default(),
            current_profile: None,
            recurrences: None,
            chosen_recurrence: None,
        }
    }

    pub fn menu_bar(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("User", |ui| {
                if let Some(user) = &self.current_profile {
                    ui.label(format!("Welcome {}", user.name));
                    ui.separator();
                    if ui.button("Overview").clicked() {
                        // something
                    }
                    if ui.button("Logout").clicked() {
                        self.current_profile = None;
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

        self.check_auth_dialog(ctx);
        self.check_recurrence_dialog(ctx);
    }
}
