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
pub async fn get_author_by_id(_pool: &Pool<Postgres>, _id: AuthorId) -> Result<Option<Author>> {
    todo!()
}

/// Retrieve all of the authors from the database and return them. We don't care about their books yet so this will just be a Vector of objects
pub async fn get_all_authors(_pool: &Pool<Postgres>) -> Result<Vec<Author>> {
    todo!()
}

/// Update the author's name in the database.
pub async fn update_author(_pool: &Pool<Postgres>, _id: AuthorId, _name: &str) -> Result<()> {
    todo!()
}

/// Permanently delete the author from the database.
pub async fn delete_author(_pool: &Pool<Postgres>, _id: AuthorId) -> Result<()> {
    todo!()
}

/// Create an author, a book, and associate them together all at the same time in the database.
///
/// This is bulk operation, ensure that if any of the commands fail, then the database will not be changed.
///
/// Return a tuple with the author id and book ids that are created
pub async fn create_author_and_book(
    _pool: &Pool<Postgres>,
    _author_name: &str,
    _book_name: &str,
) -> Result<(AuthorId, BookId)> {
    todo!()
}

/// When returning all of the authors together implement a HashMap as provided here or any other Maps, for example a BTreeMap if you want to ensure the order of the authors in the Map.
///
/// The author should have the books associated them in a Vector
pub type Authors = HashMap<AuthorId, AuthorWithBooks>;

/// Retrieve all of the authors from the database with their books. Use a single query to the database to get all of the data you need and then return the authors using the Authors type.
pub async fn get_all_authors_with_books(_pool: &Pool<Postgres>) -> Result<Authors> {
    todo!()
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
