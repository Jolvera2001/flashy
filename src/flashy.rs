use crate::channel_handlers::handle_commands;
use crate::flashy_events::{Commands, StateEvent};
use crate::models::profile::Profile;
use crate::models::profile_dto::ProfileDto;
use crate::models::recurrence::Recurrence;
use crate::models::recurrence_dto::RecurrenceDto;
use eframe::{App, Frame};
use egui::{Context, Ui};
use egui_extras::{Column, TableBuilder};
use sqlx::SqlitePool;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct Flashy {
    // connections/services/events/channels
    pub db_pool: SqlitePool,
    pub command_channel: Sender<Commands>,
    pub event_channel_sender: Sender<StateEvent>,
    pub event_channel_receiver: Receiver<StateEvent>,

    // dialogs/forms
    pub profile_form_dialog: bool,
    pub recurrence_dialog: bool,
    pub profile_form: ProfileDto,
    pub selected_profile: Option<Profile>,
    pub recurrence_form: RecurrenceDto,

    // state
    pub profiles: Option<Vec<Profile>>,
    pub current_profile: Option<Profile>,
    pub recurrences: Option<Vec<Recurrence>>,
    pub chosen_recurrence: Option<Recurrence>,
}

impl Flashy {
    pub fn new(_cc: &eframe::CreationContext<'_>, db_pool: SqlitePool) -> Self {
        let (command_tx, mut command_rx) = broadcast::channel::<Commands>(30);
        let (mut event_tx, event_rx) = broadcast::channel::<StateEvent>(30);
        let internal_ref = event_tx.clone();

        let db_clone = db_pool.clone();
        tokio::spawn(async move {
            handle_commands(db_clone, &mut command_rx, &mut event_tx).await;
        });

        Self {
            db_pool,
            command_channel: command_tx,
            event_channel_sender: internal_ref,
            event_channel_receiver: event_rx,
            profile_form_dialog: false,
            recurrence_dialog: false,
            profile_form: ProfileDto::default(),
            selected_profile: None,
            recurrence_form: RecurrenceDto::default(),
            profiles: None,
            current_profile: None,
            recurrences: None,
            chosen_recurrence: None,
        }
    }

    pub fn menu_bar(&mut self, ui: &mut Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("User", |ui| {
                if self.current_profile.is_some() {
                    if ui.button("Overview").clicked() {
                        // TODO: Maybe do a different page?
                    }
                    if ui.button("Logout").clicked() {
                        if let Err(e) = self.event_channel_sender.send(
                            StateEvent::ProfileDeselected
                        ) {
                            eprintln!("Failed to send command: {}", e);
                        }
                    };
                } else {
                    if ui.button("Add/Select").clicked() {
                        self.profile_form_dialog = true;
                    };
                }
            });
        });
    }

    pub fn bottom_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(profile) = &self.current_profile {
                ui.label(format!("Current Profile: {}", profile.name));
                ui.separator();
            } else {
                ui.label("Not logged in");
            }
        });
    }

    pub fn main_content(&mut self, ui: &mut Ui) {
        let has_profile = self.current_profile.is_some();

        ui.horizontal(|ui| {
            if ui
                .add_enabled(has_profile, egui::Button::new("Add Recurrence"))
                .clicked()
            {
                self.recurrence_dialog = true;
            };
            ui.separator();
        });
        ui.separator();

        if let Some(recurrences) = &self.recurrences {
            if recurrences.is_empty() {
                ui.horizontal_centered(|ui| {
                    ui.vertical_centered(|ui| ui.heading("No Recurrences on this profile!"));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::auto().resizable(true))
                            .column(Column::auto().resizable(true))
                            .column(Column::auto().resizable(true))
                            .header(25.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Amount");
                                });
                                header.col(|ui| {
                                    ui.heading("Circulating Date");
                                });
                            })
                            .body(|mut body| {
                                if let Some(recurrences) = &self.recurrences {
                                    for recurrence in recurrences {
                                        body.row(20.0, |mut row| {
                                            let is_selected = self
                                                .chosen_recurrence
                                                .as_ref()
                                                .map_or(false, |c| c.id == recurrence.id);

                                            row.col(|ui| {
                                                if ui
                                                    .selectable_label(is_selected, &recurrence.name)
                                                    .clicked()
                                                {
                                                    self.chosen_recurrence =
                                                        Some(recurrence.clone());
                                                }
                                            });
                                            row.col(|ui| {
                                                ui.label(&recurrence.amount.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(
                                                    &recurrence
                                                        .circulating_date
                                                        .date_naive()
                                                        .to_string(),
                                                );
                                            });
                                        });
                                    }
                                }
                            });
                    });

                    ui.separator();

                    if let Some(recurrence) = &self.chosen_recurrence {
                        ui.vertical(|ui| {
                            ui.heading(&recurrence.name);
                            ui.separator();
                            ui.label(format!("Amount: {}", recurrence.amount));
                            ui.label(format!("Circulating Date: {}", recurrence.circulating_date));
                            ui.label(format!("Is Income? {}", recurrence.is_income))
                        });
                    }
                });

            }
        } else {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| ui.heading("No Recurrences"));
            });
        }
    }
}

impl App for Flashy {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.handle_events(ctx);

        if let Some(profile) = &self.current_profile
            && self.recurrences.is_none()
        {
            if let Err(e) = self.command_channel.send(Commands::GetRecurrences {
                profile_id: profile.id,
            }) {
                eprintln!("Error sending GetRecurrences command: {}", e)
            }
        }

        egui::containers::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_content(ui);
        });

        egui::containers::TopBottomPanel::bottom("Info Bar").show(ctx, |ui| {
            self.bottom_bar(ui);
        });

        self.check_auth_dialog(ctx);
        self.check_recurrence_dialog(ctx);
    }
}
