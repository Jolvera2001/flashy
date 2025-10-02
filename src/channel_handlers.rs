use crate::flashy::Flashy;
use crate::flashy_events::{ClearFieldEvent, Commands, Dialog, StateEvent};
use crate::services::profile_services::{create_profile, get_profiles};
use crate::services::recurrence_services::{create_recurrence, get_recurrences};
use sqlx::SqlitePool;
use tokio::sync::broadcast::{Receiver, Sender};

impl Flashy {
    pub fn handle_events(&mut self, ctx: &egui::Context) {
        while let Ok(state_event) = &self.event_channel_receiver.try_recv() {
            match state_event {
                StateEvent::ProfilesFetched(profiles) => {
                    self.profiles = Option::from(profiles.clone());
                }
                StateEvent::ProfileCreated(profile) => {
                    self.current_profile = Option::from(profile.clone());
                }
                StateEvent::ProfileSelected(profile) => {
                    self.current_profile = Option::from(profile.clone());
                }
                StateEvent::ProfileDeselected => {
                    self.current_profile = None;
                }
                StateEvent::GetRecurrences(recurrences) => {
                    self.recurrences = Option::from(recurrences.clone());
                }
                StateEvent::AddRecurrence(x) => {}
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
}

pub async fn handle_commands(
    db_pool: SqlitePool,
    command_receiver: &mut Receiver<Commands>,
    event_sender: &mut Sender<StateEvent>,
) {
    while let Ok(command) = command_receiver.recv().await {
        match command {
            Commands::GetProfiles => match get_profiles(&db_pool).await {
                Ok(profiles) => {
                    let event = StateEvent::ProfilesFetched(profiles);
                    let _ = event_sender.send(event);
                }
                Err(e) => {
                    let event = StateEvent::OperationFailed {
                        operation: "Getting profiles failed!".to_string(),
                        error: e.to_string(),
                    };
                    let _ = event_sender.send(event);
                }
            },
            Commands::GetRecurrences { profile_id } => {
                match get_recurrences(&db_pool, &profile_id).await {
                    Ok(recurrences) => {
                        let event = StateEvent::GetRecurrences(recurrences);
                        let _ = event_sender.send(event);
                    }
                    Err(e) => {
                        let event = StateEvent::OperationFailed {
                            operation: "Get Recurrences failed!".to_string(),
                            error: e.to_string(),
                        };
                        let _ = event_sender.send(event);
                    }
                }
            }
            Commands::AddProfile { name, description } => {
                match create_profile(&db_pool, &name, &description).await {
                    Ok(profile) => {
                        let event = StateEvent::ProfileCreated(profile);
                        let _ = event_sender.send(event);
                    }
                    Err(e) => {
                        let event = StateEvent::OperationFailed {
                            operation: "Create profile failed".to_string(),
                            error: e.to_string(),
                        };
                        let _ = event_sender.send(event);
                    }
                };
            }
            Commands::AddRecurrence {
                profile_id,
                name,
                description,
                amount,
                circulating_date,
            } => {
                match create_recurrence(
                    &db_pool,
                    &profile_id,
                    &name,
                    &description,
                    &amount,
                    &circulating_date,
                )
                .await
                {
                    Ok(recurrence) => {
                        let event = StateEvent::AddRecurrence(recurrence);
                        let _ = event_sender.send(event);
                    }
                    Err(e) => {
                        let event = StateEvent::OperationFailed {
                            operation: "Error creating recurrence".to_string(),
                            error: e.to_string(),
                        };
                        let _ = event_sender.send(event);
                    }
                }
            }
        }
    }
}
