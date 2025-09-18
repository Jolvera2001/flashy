use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<String, Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query!(
        "INSERT INTO users (id, date_created, date_updated, name, email, password_hash) VALUES (?, ?, ?, ?, ?, ?)",
        id,
        now,
        now,
        name,
        email,
        password_hash
    ).execute(pool)
    .await?;

    Ok(id)
}

// pub async fn delete_user(pool: &SqlitePool, id: &Uuid) -> Result<bool, Error> {}
