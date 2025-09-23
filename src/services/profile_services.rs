use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

pub async fn create_user(pool: &SqlitePool, name: &str, description: &str) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO profiles (id, date_created, date_updated, name, description) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&now)
    .bind(&now)
    .bind(&name)
    .bind(&description)
    .execute(pool).await?;

    Ok(id)
}

pub async fn delete_user(pool: &SqlitePool, id: &Uuid) -> Result<(), Error> {
    sqlx::query("DELETE FROM profiles WHERE id = ?")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}
