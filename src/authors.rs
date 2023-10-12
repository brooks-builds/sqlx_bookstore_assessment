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

pub async fn get_author_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Author> {
    let author = sqlx::query_as!(Author, "SELECT * FROM authors WHERE author_id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(author)
}

pub struct Author {
    pub author_id: AuthorId,
    pub name: String,
}
