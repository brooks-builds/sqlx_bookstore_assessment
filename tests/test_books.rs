use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx::{pool::PoolOptions, ConnectOptions, Pool, Postgres};
use sqlx_bookstore_assessment::{
    books::{
        create_book, create_book_and_author, delete_book, get_all_books, get_book_by_id,
        update_book,
    },
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

    let book = get_book_by_id(&pool, 1).await?.unwrap();

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

#[sqlx::test]
async fn should_update_book(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let mut book = sqlx::query_as!(TestBook, "SELECT * FROM books WHERE book_id = 1;")
        .fetch_one(&pool)
        .await?;
    book.name.push('!');

    update_book(&pool, &book.name, book.book_id).await?;

    let updated_book = get_book_by_id(&pool, book.book_id).await?.unwrap();

    assert_eq!(book.book_id, updated_book.book_id);
    assert_eq!(book.name, updated_book.name);

    Ok(())
}

#[sqlx::test]
async fn should_delete_book(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    delete_book(&pool, 1).await?;
    let deleted_book = get_book_by_id(&pool, 1).await?;

    assert!(deleted_book.is_none());

    let result = sqlx::query!("SELECT COUNT(*) FROM books;")
        .fetch_one(&pool)
        .await?;

    assert_eq!(result.count, Some(5));

    Ok(())
}

#[sqlx::test]
async fn should_create_book_and_author_together(pool: Pool<Postgres>) -> Result<()> {
    let mut rng = thread_rng();
    let book_name = Alphanumeric.sample_string(&mut rng, 8);
    let author_name = Alphanumeric.sample_string(&mut rng, 8);

    let (book_id, author_id) = create_book_and_author(&pool, &book_name, &author_name).await?;

    let db_book_and_author = sqlx::query_as!(
        TestBookAuthor,
        r#"
        SELECT 
            b.book_id,
            b.name AS book_name,
            a.author_id,
            a.name AS author_name
        FROM book_authors ba
        JOIN books b on b.book_id = ba.book_id
        JOIN authors a on a.author_id = ba.author_id
        WHERE ba.book_id = $1
        AND ba.author_id = $2
    "#,
        book_id,
        author_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(book_name, db_book_and_author.book_name);
    assert_eq!(author_name, db_book_and_author.author_name);
    assert_eq!(author_id, db_book_and_author.author_id);
    assert_eq!(book_id, db_book_and_author.book_id);

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

struct TestBookAuthor {
    book_id: i32,
    author_id: i32,
    book_name: String,
    author_name: String,
}
