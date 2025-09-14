use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, FlashyEvents};

impl Flashy {
    pub fn handle_events(&mut self, ctx: &egui::Context) {
        if let Some(promise) = &self.current_operation {
            if let Some(result) = promise.ready() {
                match result {
                    FlashyEvents::UserLogIn(user) => {},
                    FlashyEvents::UserLogOut => {},
                    FlashyEvents::DialogClosed(dialog) => {
                        println!("{}", format!("Dialog {} was closed", dialog))
                    }
                    FlashyEvents::DialogOpened(dialog) => {
                        println!("{}", format!("Dialog {} was opened", dialog))
                    }
                    FlashyEvents::OperationFailed { operation, error } => {
                        println!("{}", format!("Operation: {} failed with error : {}", operation, error));
                    }
                    FlashyEvents::ClearFields(clear_field_event) => {
                        match clear_field_event {
                            ClearFieldEvent::LoginFields => { 
                                self.auth_form.login_name = String::new();
                                self.auth_form.login_password = String::new();
                            }
                            ClearFieldEvent::RegisterFields => {
                                self.auth_form.register_name = String::new();
                                self.auth_form.register_email = String::new();
                                self.auth_form.register_password = String::new();
                            }
                            ClearFieldEvent::RecurrenceFields => {}
                        }
                    }
                }
                self.current_operation = None;
            } else {
                println!("Promise not ready yet")
            }
        }
    }
}