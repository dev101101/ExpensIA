use axum::{Router, extract::State, routing::get};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/", get(list))
}

async fn list(State(_pool): State<PgPool>) -> &'static str {
    "list expenses - not implemented"
}
