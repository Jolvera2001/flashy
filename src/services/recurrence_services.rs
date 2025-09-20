use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

pub async fn create_recurrence(
    pool: &SqlitePool,
    user_id: Uuid,
    name: String,
    description: String,
    amount: Decimal,
    circulating_date: DateTime<Utc>,
) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query("INSERT INTO recurrences (id, user_id, date_created, date_updated, name, description, amount, circulating_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&user_id)
        .bind(&now)
        .bind(&now)
        .bind(&name)
        .bind(&description)
        .bind(&amount.to_string())
        .bind(&circulating_date)
        .execute(pool).await?;

    Ok(id)
}

pub async fn get_recurrences(user_id: &Uuid) {}

pub async fn get_recurrence_single(id: &Uuid) {}

pub async fn update_recurrence(
    user_id: Uuid,
    name: String,
    description: String,
    amount: Decimal,
    circulating_date: DateTime<Utc>,
) {
}

pub async fn delete_recurrence(id: &Uuid) {}
