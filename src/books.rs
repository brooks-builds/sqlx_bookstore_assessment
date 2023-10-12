use eyre::{bail, Result};
use sqlx::{Pool, Postgres};

use crate::authors::AuthorId;

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

pub async fn get_book_by_id(pool: &Pool<Postgres>, book_id: BookId) -> Result<Option<Book>> {
    Ok(
        sqlx::query_as!(Book, "SELECT * FROM books WHERE book_id = $1", book_id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn get_all_books(pool: &Pool<Postgres>) -> Result<Vec<Book>> {
    Ok(sqlx::query_as!(Book, "SELECT * FROM books")
        .fetch_all(pool)
        .await?)
}

pub async fn update_book(pool: &Pool<Postgres>, name: &str, book_id: BookId) -> Result<()> {
    sqlx::query!(
        "UPDATE books SET name = $1 WHERE book_id = $2",
        name,
        book_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_book(pool: &Pool<Postgres>, book_id: BookId) -> Result<()> {
    sqlx::query!("DELETE FROM books WHERE book_id = $1;", book_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn create_book_and_author(
    pool: &Pool<Postgres>,
    book_name: &str,
    author_name: &str,
) -> Result<(BookId, AuthorId)> {
    let mut transaction = pool.begin().await?;

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
            bail!("Error inserting book");
        }
    };

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
            bail!("Error inserting author");
        }
    };

    if let Err(error) = sqlx::query!(
        "INSERT INTO book_authors (book_id, author_id) VALUES ($1, $2)",
        book_id,
        author_id
    )
    .execute(&mut *transaction)
    .await
    {
        eprintln!("Error associating book with author: {error}");
        transaction.rollback().await?;
        bail!("Error associating book with author");
    }

    transaction.commit().await?;

    Ok((book_id, author_id))
}

pub struct Book {
    pub book_id: BookId,
    pub name: String,
}
