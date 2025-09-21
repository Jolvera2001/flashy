use egui_extras::DatePickerButton;
use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Dialog, FlashyEvents};
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
                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Login");
                        });
                        ui.add_space(10.0);

                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.auth_form.login_name);
                        ui.add_space(5.0);

                        ui.label("Password:");
                        ui.text_edit_singleline(&mut self.auth_form.login_password);
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Login").clicked() {
                                // login operation
                            }

                            if ui.button("Clear").clicked() {
                                self.current_operation = Some(Promise::spawn_async(async move {
                                    FlashyEvents::ClearFields(ClearFieldEvent::LoginFields)
                                }));
                            }
                        });
                    });
                    columns[1].group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Register");
                        });
                        ui.add_space(10.0);

                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.auth_form.register_name);
                        ui.add_space(5.0);

                        ui.label("Email:");
                        ui.text_edit_singleline(&mut self.auth_form.register_email);
                        ui.add_space(5.0);

                        ui.label("Password:");
                        ui.text_edit_singleline(&mut self.auth_form.register_password);
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Register").clicked() {
                                // register operation
                            }

                            if ui.button("Clear").clicked() {
                                self.current_operation = Some(Promise::spawn_async(async move {
                                    FlashyEvents::ClearFields(ClearFieldEvent::RegisterFields)
                                }));
                            }
                        });
                    });
                });
            });

        if !open {
            self.current_operation = Some(Promise::spawn_async(async move {
                FlashyEvents::DialogClosed(Dialog::Auth)
            }));
            self.auth_form_dialog = false;
        }
    }

    pub fn check_recurrence_dialog(&mut self, ctx: &egui::Context) {
        if !self.recurrence_dialog || self.current_user.is_none() {
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
                                .fixed_decimals(2)
                        );

                        ui.label("Circulating Date:");
                        ui.add(
                            DatePickerButton::new(&mut self.recurrence_form.circulating_date)
                        );
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Add").clicked() {}
                            if ui.button("Clear").clicked() {}
                        });
                    });
                });
            });

        if !open {
            self.current_operation = Some(Promise::spawn_async(async move {
                FlashyEvents::DialogClosed(Dialog::Recurrence)
            }));
            self.recurrence_dialog = false;
        }
    }
}
