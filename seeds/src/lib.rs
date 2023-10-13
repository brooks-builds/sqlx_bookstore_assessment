use eyre::Result;
use sqlx::{Pool, Postgres};

/// Fill out the run function so that it will seed the database with the data defined in the [README]("../../README.md")
pub async fn run(pool: Pool<Postgres>) -> Result<()> {
    todo!()
}
