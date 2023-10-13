use crate::authors::{Author, AuthorId};
use eyre::Result;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

/// Id for the books in the database
pub type BookId = i32;

/// Insert a book with the given name into the database and return the newly created book id
pub async fn create_book(_pool: &Pool<Postgres>, _name: &str) -> Result<BookId> {
    todo!()
}

/// Retrieve a single book from the database and return it. We don't care about the authors of this book yet.
///
/// In the case that the provided id doesn't exist in the database, return a None
pub async fn get_book_by_id(_pool: &Pool<Postgres>, _book_id: BookId) -> Result<Option<Book>> {
    todo!()
}

/// Retrieve all of the books from the database and return them. We don't care about the authors yet, so this will just be a Vector of simple book objects
pub async fn get_all_books(_pool: &Pool<Postgres>) -> Result<Vec<Book>> {
    todo!()
}

/// Update the books name with the given id in the database.
pub async fn update_book(_pool: &Pool<Postgres>, _name: &str, _book_id: BookId) -> Result<()> {
    todo!()
}

/// Permanently delete the book with the given id from the database
pub async fn delete_book(_pool: &Pool<Postgres>, _book_id: BookId) -> Result<()> {
    todo!()
}

/// Create a book and it's author at the same and associate them together in the database. Return a tuple with the book and author ids.
///
/// Since this is a bulk operation make sure that if any command fails during it the database is left unchanged.
pub async fn create_book_and_author(
    _pool: &Pool<Postgres>,
    _book_name: &str,
    _author_name: &str,
) -> Result<(BookId, AuthorId)> {
    todo!()
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
