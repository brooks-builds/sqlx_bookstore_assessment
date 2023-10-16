use eyre::Result;
use sqlx::{Pool, Postgres};

/// Fill out the run function so that it will seed the database with the data defined in the [README]("../../README.md")
pub async fn run(pool: Pool<Postgres>) -> Result<()> {
    let mut transaction = pool.begin().await?;
    let mut is_success = true;

    if let Err(error) = sqlx::query_file!("./queries/books.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding books: {error}");
        is_success = false;
    }

    if let Err(error) = sqlx::query_file!("./queries/authors.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding authors: {error}");
        is_success = false;
    }

    if let Err(error) = sqlx::query_file!("./queries/book_authors.sql")
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Error seeding book_authors: {error}");
        is_success = false;
    }

    if is_success {
        transaction.commit().await?;
    } else {
        transaction.rollback().await?;
    }

    Ok(())
}
