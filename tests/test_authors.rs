use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx::{Pool, Postgres};
use sqlx_bookstore_assessment::authors::{
    create_author, delete_author, get_all_authors, get_author_by_id, update_author,
};

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

    let author = get_author_by_id(&pool, 1).await?.unwrap();

    assert_eq!(author.author_id, 1);
    assert_eq!(author.name, "Aldous Huxley".to_owned());

    Ok(())
}

#[sqlx::test]
async fn should_get_all_authors(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let authors = get_all_authors(&pool).await?;
    let test_authors = create_test_authors();

    assert_eq!(authors.len(), test_authors.len());

    for (index, author) in authors.into_iter().enumerate() {
        assert_eq!(author.author_id, test_authors[index].author_id);
        assert_eq!(author.name, test_authors[index].name);
    }

    Ok(())
}

#[sqlx::test]
async fn should_update_author(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let mut author = get_author_by_id(&pool, 1).await?.unwrap();
    author.name.push('!');

    update_author(&pool, 1, &author.name).await?;
    let updated_author = get_author_by_id(&pool, author.author_id).await?.unwrap();

    assert_eq!(author.author_id, updated_author.author_id);
    assert_eq!(author.name, updated_author.name);

    Ok(())
}

#[sqlx::test]
async fn should_delete_author(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    delete_author(&pool, 1).await?;
    let deleted_author = get_author_by_id(&pool, 1).await?;

    assert!(deleted_author.is_none());

    Ok(())
}

struct TestAuthor {
    pub author_id: i32,
    pub name: String,
}

fn create_test_authors() -> Vec<TestAuthor> {
    vec![
        TestAuthor {
            author_id: 1,
            name: "Aldous Huxley".to_owned(),
        },
        TestAuthor {
            author_id: 2,
            name: "Herman Melville".to_owned(),
        },
        TestAuthor {
            author_id: 3,
            name: "Washington Irving".to_owned(),
        },
        TestAuthor {
            author_id: 4,
            name: "Edgar Allan Poe".to_owned(),
        },
        TestAuthor {
            author_id: 5,
            name: "Alistair Thompson".to_owned(),
        },
        TestAuthor {
            author_id: 6,
            name: "Emily Sinclair".to_owned(),
        },
    ]
}
