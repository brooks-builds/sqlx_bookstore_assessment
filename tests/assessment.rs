use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx::{pool::PoolOptions, ConnectOptions, Pool, Postgres};
use sqlx_bookstore_assessment::{
    books::{create_book, get_all_books, get_one_book},
    connect,
};

#[sqlx::test]
async fn should_connect(
    pool_options: PoolOptions<Postgres>,
    _connect_options: impl ConnectOptions,
) -> Result<()> {
    let pool = connect(pool_options).await?;

    let result = sqlx::query!("SELECT 'hello' AS hi;")
        .fetch_one(&pool)
        .await?;
    assert_eq!(result.hi.unwrap(), "hello");

    Ok(())
}

#[sqlx::test]
async fn should_create_a_book(pool: Pool<Postgres>) -> Result<()> {
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

#[sqlx::test]
async fn should_get_one_book(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let book = get_one_book(&pool, 1).await?;

    assert_eq!(book.book_id, 1);
    assert_eq!(book.name, "Brave New World");

    Ok(())
}

#[sqlx::test]
async fn should_get_all_books(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;
    let books = get_all_books(&pool).await?;
    let test_books = create_test_books();

    assert_eq!(books.len(), 6);

    for (index, book) in books.into_iter().enumerate() {
        assert_eq!(book.book_id, test_books[index].book_id);
        assert_eq!(book.name, test_books[index].name);
    }

    Ok(())
}

fn create_test_books() -> Vec<TestBook> {
    vec![
        TestBook {
            book_id: 1,
            name: "Brave New World".to_owned(),
        },
        TestBook {
            book_id: 2,
            name: "Moby Dick".to_owned(),
        },
        TestBook {
            book_id: 3,
            name: "Omoo".to_owned(),
        },
        TestBook {
            book_id: 4,
            name: "Rip Van Winkle".to_owned(),
        },
        TestBook {
            book_id: 5,
            name: "The Raven and Other Poems".to_owned(),
        },
        TestBook {
            book_id: 6,
            name: "Mastering the Art of Programming: A Comprehensive Guide for Beginners"
                .to_owned(),
        },
    ]
}

struct TestBook {
    book_id: i32,
    name: String,
}
