use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Commands, Dialog, StateEvent};
use egui::ScrollArea;
use egui_extras::DatePickerButton;

impl Flashy {
    pub fn check_auth_dialog(&mut self, ctx: &egui::Context) {
        if !self.profile_form_dialog {
            return;
        }

        let mut keep_open = true;
        egui::Window::new("Login/Register")
            .open(&mut keep_open)
            .resizable(true)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Create Profile");
                        });
                        ui.add_space(10.0);

                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.profile_form.name);
                        ui.add_space(5.0);

                        ui.label("Description:");
                        ui.text_edit_singleline(&mut self.profile_form.description);
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Add Profile").clicked() {
                                let name = self.profile_form.name.clone();
                                let description = self.profile_form.description.clone();

                                if let Err(e) = self
                                    .command_channel
                                    .send(Commands::AddProfile { name, description })
                                {
                                    eprintln!("Failed to send command: {}", e);
                                };
                                self.profile_form_dialog = false;
                            }

                            if ui.button("Clear").clicked() {
                                if let Err(e) = self
                                    .event_channel_sender
                                    .send(StateEvent::ClearFields(ClearFieldEvent::ProfileFields))
                                {
                                    eprintln!("Failed to send command: {}", e);
                                };
                            }
                        });
                    });
                    columns[1].group(|ui| {
                        ui.vertical_centered(|ui| {
                            if self.profiles.is_none() {
                                ui.heading("Fetching Profiles...");
                                if let Err(e) = self.command_channel.send(Commands::GetProfiles) {
                                    eprintln!("Failed to send command: {}", e);
                                };
                            } else {
                                ScrollArea::vertical().show(ui, |ui| {
                                    if let Some(profiles) = &self.profiles {
                                        for profile in profiles.iter() {
                                            let is_selected =
                                                self.selected_profile.as_ref().map(|p| p.id)
                                                    == Some(profile.id);
                                            if ui
                                                .selectable_label(is_selected, &profile.name)
                                                .clicked()
                                            {
                                                self.selected_profile = Some(profile.clone());
                                            }
                                        }
                                    }
                                });

                                ui.add_space(10.0);

                                if ui.button("Use Selected Profile").clicked() {
                                    if let Some(profile) = &self.selected_profile {
                                        if let Err(e) = self
                                            .event_channel_sender
                                            .send(StateEvent::ProfileSelected(profile.clone()))
                                        {
                                            eprintln!("Failed to send command: {}", e);
                                        }
                                    }
                                    self.profile_form_dialog = false;
                                }
                            };
                        });
                    });
                });
            });

        if !keep_open {
            self.profile_form_dialog = false;
        }

        if !self.profile_form_dialog {
            if let Err(e) = self
                .event_channel_sender
                .send(StateEvent::DialogClosed(Dialog::Auth))
            {
                eprintln!("Failed to send event: {}", e);
            };
            self.profile_form_dialog = false;
        }
    }

    pub fn check_recurrence_dialog(&mut self, ctx: &egui::Context) {
        if !self.recurrence_dialog || self.current_profile.is_none() {
            return;
        }

        let mut keep_open = true;
        egui::Window::new("New Recurrence")
            .open(&mut keep_open)
            .resizable(true)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.columns(1, |columns| {
                    columns[0].group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Add Recurrence");
                        });
                        ui.add_space(10.0);

                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.recurrence_form.name);
                        ui.add_space(5.0);

                        ui.label("Description:");
                        ui.text_edit_multiline(&mut self.recurrence_form.description);
                        ui.add_space(5.0);

                        ui.label("Amount:");
                        ui.add(
                            egui::DragValue::new(&mut self.recurrence_form.amount)
                                .speed(0.01)
                                .prefix("$")
                                .fixed_decimals(2),
                        );

                        ui.horizontal(|ui| {
                            ui.label("Circulating Date:");
                            ui.add(DatePickerButton::new(
                                &mut self.recurrence_form.circulating_date,
                            ));
                            ui.add_space(10.0);
                            ui.checkbox(&mut self.recurrence_form.is_income, "Is Income?");
                        });
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Add").clicked() {
                                let profile_id = self.current_profile.as_ref().unwrap().id.clone();
                                let name = self.recurrence_form.name.clone();
                                let description = self.recurrence_form.description.clone();
                                let amount = self.recurrence_form.amount;
                                let is_income = self.recurrence_form.is_income;

                                if let Err(e) = self.command_channel.send(Commands::AddRecurrence {
                                    profile_id,
                                    name,
                                    description,
                                    amount,
                                    is_income,
                                    circulating_date: self
                                        .recurrence_form
                                        .get_recurrence_date_time(),
                                }) {
                                    eprintln!("Failed to send command: {}", e);
                                }

                                self.recurrence_dialog = false;
                            };

                            if ui.button("Clear").clicked() {
                                if let Err(e) = self.event_channel_sender.send(
                                    StateEvent::ClearFields(ClearFieldEvent::RecurrenceFields),
                                ) {
                                    eprintln!("Failed to send command: {}", e);
                                }
                            };
                        });
                    });
                });
            });

        if !keep_open {
            self.profile_form_dialog = false;
        }

        if !self.recurrence_dialog {
            if let Err(e) = self
                .event_channel_sender
                .send(StateEvent::DialogClosed(Dialog::Auth))
            {
                eprintln!("Failed to send command: {}", e);
            }
            self.recurrence_dialog = false;
        }
    }
}
