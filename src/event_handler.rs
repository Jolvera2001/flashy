use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Dialog, FlashyEvents};

impl Flashy {
    pub fn handle_events(&mut self, ctx: &egui::Context) {
        if let Some(promise) = &self.current_operation {
            if let Some(result) = promise.ready() {
                match result {
                    FlashyEvents::UserLogIn(user) => {}
                    FlashyEvents::UserLogOut => {}
                    FlashyEvents::AddRecurrence => {}
                    FlashyEvents::DialogClosed(dialog) => match dialog {
                        Dialog::Auth => {
                            self.profile_form.clear();
                            println!("Dialog Auth closed")
                        }
                        Dialog::Recurrence => {
                            println!("Dialog Recurrence closed")
                        }
                    },
                    FlashyEvents::DialogOpened(dialog) => match dialog {
                        Dialog::Auth => {
                            println!("Dialog Auth opened!")
                        }
                        Dialog::Recurrence => {
                            println!("Dialog Recurrence opened!")
                        }
                    },
                    FlashyEvents::ClearFields(clear_field_event) => match clear_field_event {
                        ClearFieldEvent::ProfileFields => {
                            self.profile_form.clear();
                        }
                        ClearFieldEvent::RecurrenceFields => {}
                    },
                    FlashyEvents::OperationFailed { operation, error } => {
                        println!(
                            "{}",
                            format!("Operation: {} failed with error : {}", operation, error)
                        );
                    }
                }
                self.current_operation = None;
            } else {
                println!("Promise not ready yet")
            }
        }
    }
}
