use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Commands, Dialog, StateEvent};
use tokio::sync::broadcast::{Receiver, Sender};

impl Flashy {
    pub async fn handle_events(&mut self, ctx: &egui::Context) {
        while let Ok(state_event) = &self.event_channel_receiver.recv().await {
            match state_event {
                StateEvent::UserLogIn(profile) => {}
                StateEvent::UserLogOut => {}
                StateEvent::AddRecurrence => {}
                StateEvent::DialogClosed(dialog) => match dialog {
                    Dialog::Auth => {
                        self.profile_form.clear();
                        println!("Dialog Auth closed")
                    }
                    Dialog::Recurrence => {
                        self.recurrence_form.clear();
                        println!("Dialog Recurrence closed")
                    }
                },
                StateEvent::DialogOpened(dialog) => match dialog {
                    Dialog::Auth => {
                        println!("Dialog Auth opened!")
                    }
                    Dialog::Recurrence => {
                        println!("Dialog Recurrence opened!")
                    }
                },
                StateEvent::ClearFields(clear_field_event) => match clear_field_event {
                    ClearFieldEvent::ProfileFields => {
                        self.profile_form.clear();
                    }
                    ClearFieldEvent::RecurrenceFields => {}
                },
                StateEvent::OperationFailed { operation, error } => {
                    println!(
                        "{}",
                        format!("Operation: {} failed with error : {}", operation, error)
                    );
                }
            }
            self.current_operation = None;
        }
    }

    pub async fn handle_commands(
        command_receiver: &mut Receiver<Commands>,
        event_sender: &mut Sender<StateEvent>,
    ) {
        while let Ok(command) = command_receiver.recv().await {
            // TODO: Move business logic here instead of within UI
            // Ui can call channel sender in order to handle commands
            match command {
                Commands::AddProfile { .. } => {}
                Commands::AddRecurrence { .. } => {}
            }
        }
    }
}
