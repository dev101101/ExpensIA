use axum::extract::State;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;

use crate::document::ocr::OcrService;
use crate::rag::embeddings::EmbeddingService;
use crate::rag::generation::Generator;

mod documents;
mod error;
mod expenses;
mod query;

pub use error::ApiError;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ocr: OcrService,
    pub embedder: EmbeddingService,
    pub generator: Generator,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            ocr: OcrService::new(),
            embedder: EmbeddingService::new(),
            generator: Generator::new(),
            pool,
        }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/documents", documents::router())
        .nest("/expenses", expenses::router())
        .nest("/query", query::router())
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Result<&'static str, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok("ok")
}
