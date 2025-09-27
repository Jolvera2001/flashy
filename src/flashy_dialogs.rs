use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Commands, Dialog, StateEvent};
use egui_extras::DatePickerButton;
use poll_promise::Promise;

impl Flashy {
    pub fn check_auth_dialog(&mut self, ctx: &egui::Context) {
        if !self.auth_form_dialog {
            return;
        }

        let mut open = true;

        egui::Window::new("Login/Register")
            .open(&mut open)
            .resizable(true)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.columns(1, |columns| {
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
                                // login operation
                            }

                            if ui.button("Clear").clicked() {
                                self.current_operation = Some(Promise::spawn_async(async move {
                                    StateEvent::ClearFields(ClearFieldEvent::ProfileFields)
                                }));
                            }
                        });
                    });
                });
            });

        if !open {
            self.current_operation = Some(Promise::spawn_async(async move {
                StateEvent::DialogClosed(Dialog::Auth)
            }));
            self.auth_form_dialog = false;
        }
    }

    pub fn check_recurrence_dialog(&mut self, ctx: &egui::Context) {
        if !self.recurrence_dialog || self.current_profile.is_none() {
            return;
        }

        let mut open = true;

        egui::Window::new("New Recurrence")
            .open(&mut open)
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

                        ui.label("Circulating Date:");
                        ui.add(DatePickerButton::new(
                            &mut self.recurrence_form.circulating_date,
                        ));
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Add").clicked() {
                                let profile_id = self.current_profile.as_ref().unwrap().id.clone();
                                let name = self.recurrence_form.name.clone();
                                let description = self.recurrence_form.description.clone();
                                let amount = self.recurrence_form.amount;

                                if let Err(e) = self.command_channel.try_send(Commands::AddRecurrence {
                                    profile_id,
                                    name,
                                    description,
                                    amount,
                                    circulating_date: self
                                        .recurrence_form
                                        .get_recurrence_date_time(),
                                }) {
                                    eprintln!("Failed to send command: {}", e);
                                }
                            };

                            if ui.button("Clear").clicked() {
                                
                            };
                        });
                    });
                });
            });

        if !open {
            self.current_operation = Some(Promise::spawn_async(async move {
                StateEvent::DialogClosed(Dialog::Recurrence)
            }));
            self.recurrence_dialog = false;
        }
    }
}
