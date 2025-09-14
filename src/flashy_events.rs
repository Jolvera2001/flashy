use crate::models::recurrence::Recurrence;
use crate::models::user::User;

pub enum FlashyEvents {
    UserLogIn(User),
    UserLogOut,

    // ui
    DialogClosed(Dialog),
    DialogOpened(Dialog),
    ClearFields(ClearFieldEvent),

    // error
    OperationFailed { operation: String, error: String },
}

pub enum ClearFieldEvent {
    LoginFields,
    RegisterFields,
    RecurrenceFields,
}

pub enum Dialog {
    Auth,
    Recurrence
}