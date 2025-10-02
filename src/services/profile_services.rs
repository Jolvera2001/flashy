use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;
use crate::models::profile::Profile;

pub async fn create_profile(pool: &SqlitePool, name: &str, description: &str) -> Result<Profile, Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO profiles (id, date_created, date_updated, name, description) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&now)
    .bind(&now)
    .bind(name)
    .bind(description)
    .execute(pool).await?;

    let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await?;

    Ok(profile)
}

pub async fn delete_profile(pool: &SqlitePool, id: &Uuid) -> Result<(), Error> {
    sqlx::query("DELETE FROM profiles WHERE id = ?")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}
