use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Recurrence {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub amount: String,
    pub circulating_date: DateTime<Utc>,
}
