use seeds;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Error loading .env");

    let database_uri =
        std::env::var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_uri)
        .await
        .expect("Error connecting to database");

    seeds::run(pool).await.expect("Error running seeds");
}
