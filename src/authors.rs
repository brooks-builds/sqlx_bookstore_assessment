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

pub async fn get_author_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Author>> {
    let author = sqlx::query_as!(Author, "SELECT * FROM authors WHERE author_id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(author)
}

pub async fn get_all_authors(pool: &Pool<Postgres>) -> Result<Vec<Author>> {
    let authors = sqlx::query_as!(Author, "SELECT * FROM authors")
        .fetch_all(pool)
        .await?;

    Ok(authors)
}

pub async fn update_author(pool: &Pool<Postgres>, id: i32, name: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE authors SET name = $1 WHERE author_id = $2",
        name,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_author(pool: &Pool<Postgres>, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM authors WHERE author_id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub struct Author {
    pub author_id: AuthorId,
    pub name: String,
}
