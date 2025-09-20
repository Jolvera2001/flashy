use crate::models::recurrence::Recurrence;
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

pub async fn get_recurrences(pool: &SqlitePool, user_id: &Uuid) -> Result<Vec<Recurrence>, Error> {
    let recurrences =
        sqlx::query_as::<_, Recurrence>("SELECT * FROM recurrences WHERE user_id = ?")
            .bind(&user_id)
            .fetch_all(pool)
            .await?;

    Ok(recurrences)
}

pub async fn get_recurrence_single(pool: &SqlitePool, id: &Uuid) -> Result<Recurrence, Error> {
    let recurrence = sqlx::query_as::<_, Recurrence>("SELECT * FROM recurrences WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(recurrence)
}

pub async fn update_recurrence(
    pool: &SqlitePool,
    name: String,
    description: String,
    amount: Decimal,
    circulating_date: DateTime<Utc>,
) -> Result<(), Error> {
    sqlx::query("UPDATE recurrences SET name = ?, description = ?, amount = ?, circulating_date = ? WHERE id = ?", ).bind(&name)
    .bind(&description)
    .bind(&amount.to_string())
    .bind(&circulating_date)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_recurrence(pool: &SqlitePool, id: &Uuid) -> Result<(), Error> {
    sqlx::query("DELETE FROM recurrences WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
