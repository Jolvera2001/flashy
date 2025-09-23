use std::fmt::Error;
use crate::models::user::User;
use chrono::NaiveDate;

pub enum FlashyEvents {
    UserLogIn(User),
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

pub enum ClearFieldEvent {
    ProfileFields,
    RecurrenceFields,
}

pub enum Dialog {
    Auth,
    Recurrence,
}
