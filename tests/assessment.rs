use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx_bookstore_assessment::{books::create_book, connect};

#[tokio::test]
async fn should_connect() -> Result<()> {
    let pool = connect().await?;

    let result = sqlx::query!("SELECT name FROM books WHERE name = 'Moby Dick';")
        .fetch_one(&pool)
        .await?;
    assert_eq!(result.name, "Moby Dick");

    Ok(())
}

#[tokio::test]
async fn should_create_a_book() -> Result<()> {
    let pool = connect().await?;
    let mut rng = thread_rng();
    let name = Alphanumeric.sample_string(&mut rng, 8);
    let new_book_id = create_book(&pool, &name).await?;
    let created_book = sqlx::query_as!(
        TestBook,
        "SELECT * FROM books WHERE book_id = $1",
        new_book_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(name, created_book.name);

    Ok(())
}

struct TestBook {
    book_id: i32,
    name: String,
}
