use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod repository;

pub async fn create_pool() -> PgPool {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://expensia:expensia_dev@localhost:5432/expensia".into()
        });

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("can't connect to database")
}
