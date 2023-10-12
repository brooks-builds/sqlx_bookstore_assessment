use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx::{Pool, Postgres};
use sqlx_bookstore_assessment::authors::{create_author, get_author_by_id};

#[sqlx::test]
async fn should_create_author(pool: Pool<Postgres>) -> Result<()> {
    let mut rng = thread_rng();
    let new_name = Alphanumeric.sample_string(&mut rng, 8);
    let created_author_id = create_author(&pool, &new_name).await?;

    let created_author = sqlx::query_as!(
        TestAuthor,
        "SELECT * FROM authors WHERE author_id = $1",
        created_author_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(new_name, created_author.name);

    Ok(())
}

#[sqlx::test]
async fn should_get_one_author(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let author = get_author_by_id(&pool, 1).await?;

    assert_eq!(author.author_id, 1);
    assert_eq!(author.name, "Aldous Huxley".to_owned());

    Ok(())
}

struct TestAuthor {
    author_id: i32,
    name: String,
}
