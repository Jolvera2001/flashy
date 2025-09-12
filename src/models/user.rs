use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    date_created: DateTime<Utc>,
    date_updated: DateTime<Utc>,
    name: String,
}