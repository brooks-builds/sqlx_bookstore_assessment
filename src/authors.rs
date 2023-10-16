use crate::books::{Book, BookId};
use eyre::Result;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

pub type AuthorId = i32;

/// Insert an author into the database and return the newly created author id
pub async fn create_author(pool: &Pool<Postgres>, name: &str) -> Result<AuthorId> {
    let result = sqlx::query!(
        "INSERT INTO authors (name) VALUES ($1) RETURNING author_id",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(result.author_id)
}

/// Retrieve a single author from the database and return it. We don't care about their books yet so this will just be an object that has the author id and the name.
///
/// Since it's possible that the id doesn't exist in the database, return a None if the author cannot be found.
pub async fn get_author_by_id(pool: &Pool<Postgres>, id: AuthorId) -> Result<Option<Author>> {
    Ok(
        sqlx::query_as!(Author, "SELECT * FROM authors WHERE author_id = $1", id)
            .fetch_optional(pool)
            .await?,
    )
}

/// Retrieve all of the authors from the database and return them. We don't care about their books yet so this will just be a Vector of objects
pub async fn get_all_authors(pool: &Pool<Postgres>) -> Result<Vec<Author>> {
    Ok(sqlx::query_as!(Author, "SELECT * FROM authors")
        .fetch_all(pool)
        .await?)
}

/// Update the author's name in the database.
pub async fn update_author(pool: &Pool<Postgres>, id: AuthorId, name: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE authors SET name = $1 WHERE author_id = $2",
        name,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Permanently delete the author from the database.
pub async fn delete_author(pool: &Pool<Postgres>, id: AuthorId) -> Result<()> {
    sqlx::query!("DELETE FROM authors WHERE author_id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Create an author, a book, and associate them together all at the same time in the database.
///
/// This is bulk operation, ensure that if any of the commands fail, then the database will not be changed.
///
/// Return a tuple with the author id and book ids that are created
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
            return Err(error.into());
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
            return Err(error.into());
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
        eprintln!("Error associating new author with book: {error}");
        transaction.rollback().await?;
        return Err(error.into());
    }

    transaction.commit().await?;

    Ok((author_id, book_id))
}

/// When returning all of the authors together implement a HashMap as provided here or any other Maps, for example a BTreeMap if you want to ensure the order of the authors in the Map.
///
/// The author should have the books associated them in a Vector
pub type Authors = HashMap<AuthorId, AuthorWithBooks>;

/// Retrieve all of the authors from the database with their books. Use a single query to the database to get all of the data you need and then return the authors using the Authors type.
pub async fn get_all_authors_with_books(pool: &Pool<Postgres>) -> Result<Authors> {
    let book_authors = sqlx::query!(
        r#"
        SELECT
            a.author_id,
            a.name AS author_name,
            b.book_id,
            b.name AS book_name
        FROM authors a
        JOIN book_authors ba ON ba.author_id = a.author_id
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

        let book = Book {
            book_id: book_author.book_id,
            name: book_author.book_name,
        };

        author.books.push(book);
    }

    Ok(authors)
}

/// Single Author with just it's id and name
///
/// We don't care about the Author's books yet
pub struct Author {
    pub author_id: AuthorId,
    pub name: String,
}

/// Single Author with their books in a Vector. the books are the simgle single Book type which just includes the book id and name.
pub struct AuthorWithBooks {
    pub author_id: AuthorId,
    pub name: String,
    pub books: Vec<Book>,
}
