use axum::{Router, extract::State, routing::get};
use sqlx::PgPool;

mod documents;
mod expenses;
mod query;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/health", get(health))
        .nest("/documents", documents::router())
        .nest("/expenses", expenses::router())
        .nest("/query", query::router())
}

async fn health(State(pool): State<PgPool>) -> &'static str {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("db health check failed");
    "ok"
}
