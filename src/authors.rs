use eyre::{bail, Result};
use sqlx::{Pool, Postgres};

use crate::books::BookId;

pub type AuthorId = i32;

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

pub async fn create_author_and_book(
    pool: &Pool<Postgres>,
    author_name: &str,
    book_name: &str,
) -> Result<(AuthorId, BookId)> {
    let mut transaction = pool.begin().await?;

    let author_id = match sqlx::query!(
        "INSERT INTO authors (name) VALUES ($1) RETURNING author_id",
        author_name
    )
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(result) => result.author_id,
        Err(error) => {
            eprintln!("Error inserting author: {error}");
            transaction.rollback().await?;
            bail!("There was a problem inserting the author");
        }
    };

    let book_id = match sqlx::query!(
        "INSERT INTO books (name) VALUES ($1) RETURNING book_id",
        book_name
    )
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(result) => result.book_id,
        Err(error) => {
            eprintln!("Error inserting book: {error}");
            transaction.rollback().await?;
            bail!("There was a problem inserting the book");
        }
    };

    if let Err(error) = sqlx::query!(
        "INSERT INTO book_authors (author_id, book_id) VALUES ($1, $2)",
        author_id,
        book_id
    )
    .execute(&mut *transaction)
    .await
    {
        eprintln!("Error inserting book_author: {error}");
        transaction.rollback().await?;
        bail!("There was a problem associating the author and book");
    }

    transaction.commit().await?;

    Ok((author_id, book_id))
}

pub struct Author {
    pub author_id: AuthorId,
    pub name: String,
}
