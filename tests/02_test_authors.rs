use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sqlx::{Pool, Postgres};
use sqlx_bookstore_assessment::authors::{
    create_author, create_author_and_book, delete_author, get_all_authors,
    get_all_authors_with_books, get_author_by_id, update_author,
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
    assert_eq!(created_author.author_id, created_author_id);

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

#[sqlx::test]
async fn should_create_author_and_book(pool: Pool<Postgres>) -> Result<()> {
    let mut rng = thread_rng();
    let author_name = Alphanumeric.sample_string(&mut rng, 8);
    let book_name = Alphanumeric.sample_string(&mut rng, 8);

    let (author_id, book_id) = create_author_and_book(&pool, &author_name, &book_name).await?;

    let test_author_book = sqlx::query_as!(
        TestAuthorBook,
        r#"
        SELECT
            a.author_id,
            a.name AS author_name,
            b.book_id,
            b.name AS book_name
        FROM book_authors ba
        JOIN authors a ON a.author_id = ba.author_id
        JOIN books b ON b.book_id = ba.book_id
    "#
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(test_author_book.author_id, author_id);
    assert_eq!(test_author_book.book_id, book_id);
    assert_eq!(test_author_book.author_name, author_name);
    assert_eq!(test_author_book.book_name, book_name);

    Ok(())
}

#[sqlx::test]
async fn should_get_all_authors_and_their_books(pool: Pool<Postgres>) -> Result<()> {
    seeds::run(pool.clone()).await?;

    let authors = get_all_authors_with_books(&pool).await?;
    let test_authors = create_test_authors();

    assert_eq!(authors.len(), test_authors.len());

    for test_author in test_authors {
        let author = authors.get(&test_author.author_id).unwrap();

        assert_eq!(test_author.author_id, author.author_id);
        assert_eq!(test_author.name, author.name);
        assert_eq!(test_author.books.len(), author.books.len());

        for test_book in test_author.books {
            let book = author
                .books
                .iter()
                .find(|book| book.book_id == test_book.book_id)
                .unwrap();

            assert_eq!(test_book.book_id, book.book_id);
            assert_eq!(test_book.name, book.name);
        }
    }

    Ok(())
}

struct TestAuthor {
    pub author_id: i32,
    pub name: String,
}

fn create_test_authors() -> Vec<TestAuthorWithBooks> {
    vec![
        TestAuthorWithBooks {
            author_id: 1,
            name: "Aldous Huxley".to_owned(),
            books: vec![TestBook {
                book_id: 1,
                name: "Brave New World".to_owned(),
            }],
        },
        TestAuthorWithBooks {
            author_id: 2,
            name: "Herman Melville".to_owned(),
            books: vec![
                TestBook {
                    book_id: 2,
                    name: "Moby Dick".to_owned(),
                },
                TestBook {
                    book_id: 3,
                    name: "Omoo".to_owned(),
                },
            ],
        },
        TestAuthorWithBooks {
            author_id: 3,
            name: "Washington Irving".to_owned(),
            books: vec![TestBook {
                book_id: 4,
                name: "Rip Van Winkle".to_owned(),
            }],
        },
        TestAuthorWithBooks {
            author_id: 4,
            name: "Edgar Allan Poe".to_owned(),
            books: vec![TestBook {
                book_id: 5,
                name: "The Raven and Other Poems".to_owned(),
            }],
        },
        TestAuthorWithBooks {
            author_id: 5,
            name: "Alistair Thompson".to_owned(),
            books: vec![TestBook {
                book_id: 6,
                name: "Mastering the Art of Programming: A Comprehensive Guide for Beginners"
                    .to_owned(),
            }],
        },
        TestAuthorWithBooks {
            author_id: 6,
            name: "Emily Sinclair".to_owned(),
            books: vec![TestBook {
                book_id: 6,
                name: "Mastering the Art of Programming: A Comprehensive Guide for Beginners"
                    .to_owned(),
            }],
        },
    ]
}

struct TestAuthorBook {
    author_id: i32,
    book_id: i32,
    author_name: String,
    book_name: String,
}

struct TestAuthorWithBooks {
    author_id: i32,
    name: String,
    books: Vec<TestBook>,
}

struct TestBook {
    book_id: i32,
    name: String,
}
