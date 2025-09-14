use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Dialog, FlashyEvents};
use poll_promise::Promise;

impl Flashy {
    pub fn check_login_register_dialog(&mut self, ctx: &egui::Context) {
        if !self.test_dialog_open {
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
            self.test_dialog_open = false;
        }
    }
}
