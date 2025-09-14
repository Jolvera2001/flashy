use crate::models::recurrence::Recurrence;
use crate::models::user::User;

pub enum FlashyEvents {
    UserLogIn(User),
    UserLogOut,

    // ui
    DialogClosed(String),
    DialogOpened(String),
    ClearFields(ClearFieldEvent),

    // error
    OperationFailed { operation: String, error: String },
}

pub enum ClearFieldEvent {
    LoginFields,
    RegisterFields,
    RecurrenceFields,
}