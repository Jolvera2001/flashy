use crate::models::profile::Profile;
use crate::models::recurrence::Recurrence;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub enum Commands {
    GetProfiles,
    AddProfile {
        name: String,
        description: String,
    },
    GetRecurrences {
        profile_id: Uuid,
    },
    AddRecurrence {
        profile_id: Uuid,
        name: String,
        description: String,
        amount: f64,
        is_income: bool,
        circulating_date: DateTime<Utc>,
    },
}

#[derive(Clone)]
pub enum StateEvent {
    ProfilesFetched(Vec<Profile>),
    ProfileCreated(Profile),
    ProfileSelected(Profile),
    ProfileDeselected,
    AddRecurrence(Recurrence),
    GetRecurrences(Vec<Recurrence>),
    DeselectCurrentRecurrence,

    // ui
    DialogClosed(Dialog),
    DialogOpened(Dialog),
    ClearFields(ClearFieldEvent),

    // error
    OperationFailed { operation: String, error: String },
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
