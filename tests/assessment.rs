use eyre::Result;
use sqlx::{Pool, Postgres};
use sqlx_bookstore_assessment::connect;

#[tokio::test]
async fn should_connect() -> Result<()> {
    let pool = connect().await?;

    let result = sqlx::query!("SELECT name FROM books WHERE name = 'Moby Dick';")
        .fetch_one(&pool)
        .await?;
    assert_eq!(result.name, "Moby Dick");

    Ok(())
}
