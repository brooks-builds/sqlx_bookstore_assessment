use eyre::Result;
use sqlx::{Pool, Postgres};

pub type BookId = i32;

pub async fn create_book(pool: &Pool<Postgres>, name: &str) -> Result<BookId> {
    let result = sqlx::query!(
        "INSERT INTO books (name) VALUES ($1) RETURNING book_id;",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(result.book_id)
}
