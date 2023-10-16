pub mod authors;
pub mod books;

use eyre::Result;
use sqlx::{pool::PoolOptions, Pool, Postgres};

/// Complete this function so that it connects to a Postgres instance and returns the pool.
///
/// For testing purposes the PgPoolOptions have already been created for you. All you need to do is configure the connection if you want/need and connect to the database.
pub async fn connect(pool_options: PoolOptions<Postgres>) -> Result<Pool<Postgres>> {
    dotenvy::dotenv().ok();

    let database_uri =
        std::env::var("DATABASE_URL").expect("ERROR - missing environment variable DATABASE_URL");
    Ok(pool_options
        .max_connections(5)
        .connect(&database_uri)
        .await?)
}
