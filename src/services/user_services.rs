use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO users (id, date_created, date_updated, name, email, password_hash) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&now)
    .bind(&now)
    .bind(&name)
    .bind(&email)
    .bind(&password_hash)
    .execute(pool).await?;

    Ok(id)
}

pub async fn delete_user(pool: &SqlitePool, id: &Uuid) -> Result<(), Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}
