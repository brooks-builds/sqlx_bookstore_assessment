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

pub async fn get_one_book(pool: &Pool<Postgres>, book_id: BookId) -> Result<Book> {
    Ok(
        sqlx::query_as!(Book, "SELECT * FROM books WHERE book_id = $1", book_id)
            .fetch_one(pool)
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

pub struct Book {
    pub book_id: BookId,
    pub name: String,
}
