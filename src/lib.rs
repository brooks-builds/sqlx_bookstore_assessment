pub mod authors;
pub mod books;

use eyre::Result;
use sqlx::{pool::PoolOptions, Pool, Postgres};

/// Complete this function so that it connects to a Postgres instance and returns the pool.
///
/// For testing purposes the PgPoolOptions have already been created for you. All you need to do is configure the connection if you want/need and connect to the database.
pub async fn connect(_pool_options: PoolOptions<Postgres>) -> Result<Pool<Postgres>> {
    todo!()
}
