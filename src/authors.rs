use eyre::Result;
use sqlx::{Pool, Postgres};

type AuthorId = i32;

pub async fn create_author(pool: &Pool<Postgres>, name: &str) -> Result<AuthorId> {
    let result = sqlx::query!(
        "INSERT INTO authors (name) VALUES ($1) RETURNING author_id",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(result.author_id)
}
