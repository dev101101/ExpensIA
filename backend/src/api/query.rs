use axum::{Router, extract::State, routing::post};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/", post(query))
}

async fn query(State(_pool): State<PgPool>) -> &'static str {
    "query - not implemented"
}
