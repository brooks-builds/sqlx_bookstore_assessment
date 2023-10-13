use crate::books::{Book, BookAuthor, BookId};
use eyre::{bail, Result};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

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

pub type Authors = HashMap<AuthorId, AuthorWithBooks>;

pub async fn get_all_authors_with_books(pool: &Pool<Postgres>) -> Result<Authors> {
    let book_authors = sqlx::query_as!(
        BookAuthor,
        r#"
            SELECT
                a.author_id,
                a.name AS author_name,
                b.book_id,
                b.name AS book_name
            FROM book_authors ba
            JOIN authors a ON a.author_id = ba.author_id
            JOIN books b ON b.book_id = ba.book_id
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut authors = Authors::new();

    for book_author in book_authors {
        let author = authors
            .entry(book_author.author_id)
            .or_insert(AuthorWithBooks {
                author_id: book_author.author_id,
                name: book_author.author_name,
                books: vec![],
            });

        author.books.push(Book {
            book_id: book_author.book_id,
            name: book_author.book_name,
        });
    }

    Ok(authors)
}

pub struct Author {
    pub author_id: AuthorId,
    pub name: String,
}

pub struct AuthorWithBooks {
    pub author_id: AuthorId,
    pub name: String,
    pub books: Vec<Book>,
}
