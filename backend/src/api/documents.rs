use axum::{Router, extract::State, routing::post};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/upload", post(upload))
}

async fn upload(State(_pool): State<PgPool>) -> &'static str {
    "upload - not implemented"
}
