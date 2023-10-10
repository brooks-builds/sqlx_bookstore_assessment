use dotenvy::{dotenv, var};
use eyre::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// Complete this function so that it connects to a Postgres instance and returns the pool.
pub async fn connect() -> Result<Pool<Postgres>> {
    // dotenv().ok();

    // let database_uri = var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

    // Ok(PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&database_uri)
    //     .await?)
    todo!()
}
