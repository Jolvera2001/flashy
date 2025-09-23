use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Profile {
    pub id: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub name: String,
    pub description: String,
}