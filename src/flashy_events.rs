use crate::models::recurrence::Recurrence;
use crate::models::user::User;

pub enum FlashyEvents {
    UserLogIn(User),
    UserLogOut,

    // ui
    DialogClosed(String),
    DialogOpened(String),

    // error
    OperationFailed { operation: String, error: String },
}