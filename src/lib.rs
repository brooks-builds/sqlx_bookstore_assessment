pub mod authors;
pub mod books;

use dotenvy::{dotenv, var};
use eyre::Result;
use sqlx::{pool::PoolOptions, Pool, Postgres};

/// Complete this function so that it connects to a Postgres instance and returns the pool.
pub async fn connect(pool_options: PoolOptions<Postgres>) -> Result<Pool<Postgres>> {
    dotenv().ok();

    let database_uri = var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

    Ok(pool_options.connect(&database_uri).await?)
}
