use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Recurrence {
    id: Uuid,
    date_created: DateTime<Utc>,
    date_updated: DateTime<Utc>,
    name: String,
    description: String,
    amount: Decimal, // store as TEXT in sqlite
    circulating_date: DateTime<Utc>,
}
