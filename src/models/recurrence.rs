use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Recurrence {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub amount: String,
    pub is_income: bool,
    pub circulating_date: DateTime<Utc>,
}
