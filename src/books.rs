use crate::authors::{create_author, Author, AuthorId};
use eyre::{bail, Result};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

/// Id for the books in the database
pub type BookId = i32;

/// Insert a book with the given name into the database and return the newly created book id
pub async fn create_book(pool: &Pool<Postgres>, name: &str) -> Result<BookId> {
    let result = sqlx::query!(
        "INSERT INTO books (name) VALUES ($1) RETURNING book_id",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(result.book_id)
}

/// Retrieve a single book from the database and return it. We don't care about the authors of this book yet.
///
/// In the case that the provided id doesn't exist in the database, return a None
pub async fn get_book_by_id(pool: &Pool<Postgres>, book_id: BookId) -> Result<Option<Book>> {
    Ok(
        sqlx::query_as!(Book, "SELECT * FROM books WHERE book_id = $1", book_id)
            .fetch_optional(pool)
            .await?,
    )
}

/// Retrieve all of the books from the database and return them. We don't care about the authors yet, so this will just be a Vector of simple book objects
pub async fn get_all_books(pool: &Pool<Postgres>) -> Result<Vec<Book>> {
    Ok(sqlx::query_as!(Book, "SELECT * FROM books")
        .fetch_all(pool)
        .await?)
}

/// Update the books name with the given id in the database.
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

/// Permanently delete the book with the given id from the database
pub async fn delete_book(pool: &Pool<Postgres>, book_id: BookId) -> Result<()> {
    sqlx::query!("DELETE FROM books WHERE book_id = $1", book_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Create a book and it's author at the same and associate them together in the database. Return a tuple with the book and author ids.
///
/// Since this is a bulk operation make sure that if any command fails during it the database is left unchanged.
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
        Ok(record) => record.book_id,
        Err(error) => {
            transaction.rollback().await?;
            return Err(error.into());
        }
    };

    let author_id = match sqlx::query!(
        "INSERT INTO authors (name) VALUES ($1) RETURNING author_id",
        author_name
    )
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(record) => record.author_id,
        Err(error) => {
            transaction.rollback().await?;
            return Err(error.into());
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
        transaction.rollback().await?;
        return Err(error.into());
    }

    transaction.commit().await?;

    Ok((book_id, author_id))
}

/// The books that we're returing with the authors is a HashMap. Feel free to change this to any other kind of Map like a BTreeMap
pub type Books = HashMap<BookId, BookWithAuthors>;

/// Retrieve all books in the database with their authors and return using the Books type above.
///
/// Use a single operation to the database to get all of the data you need
pub async fn get_all_books_with_authors(_pool: &Pool<Postgres>) -> Result<Books> {
    todo!()
}

/// This struct models just the books table. Use this struct when we don't care about the authors of the books
pub struct Book {
    pub book_id: BookId,
    pub name: String,
}

/// This struct models the relationship between a book and it's authors. Use this struct when returning both books and their authors together
pub struct BookWithAuthors {
    pub book_id: BookId,
    pub name: String,
    pub authors: Vec<Author>,
}
