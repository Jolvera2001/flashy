use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::profile::Profile;

#[derive(Clone)]
pub enum Commands {
    AddProfile {
        name: String,
        description: String,
    },
    AddRecurrence {
        name: String,
        description: String,
        amount: f64,
        circulating_date: DateTime<Utc>,
    },
}

#[derive(Clone)]
pub enum StateEvent {
    UserLogIn(Profile),
    UserLogOut,
    AddRecurrence,

    // ui
    DialogClosed(Dialog),
    DialogOpened(Dialog),
    ClearFields(ClearFieldEvent),

    // error
    OperationFailed {
        operation: String,
        error: core::fmt::Error,
    },
}

#[derive(Clone)]
pub enum ClearFieldEvent {
    ProfileFields,
    RecurrenceFields,
}

#[derive(Clone)]
pub enum Dialog {
    Auth,
    Recurrence,
}
