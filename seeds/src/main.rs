use seeds::run;
use sqlx::postgres::PgPoolOptions;

/// Connect to the database and run the seeds function in the `lib.rs` file
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_uri = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&database_uri).await.unwrap();
    run(pool).await.unwrap();
}
