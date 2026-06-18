use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::rag::retrieval::Retriever;
use crate::api::ApiError;

use super::AppState;

#[derive(Deserialize)]
struct QueryRequest {
    question: String,
}

#[derive(Serialize)]
struct Source {
    content: String,
    score: f64,
}

#[derive(Serialize)]
struct QueryResponse {
    answer: String,
    sources: Vec<Source>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(query))
}

async fn query(
    State(state): State<AppState>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, ApiError> {
    let query_embedding = state
        .embedder
        .embed(&req.question)
        .await
        .map_err(|e| ApiError::Processing(e))?;

    let chunks = Retriever::retrieve(&state.pool, &query_embedding, 5)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let context = chunks
        .iter()
        .map(|c| c.content.as_str())
        .collect::<Vec<_>>()
        .join("\n---\n");

    let answer = state
        .generator
        .generate(&req.question, &context)
        .await;

    let sources = chunks
        .into_iter()
        .map(|c| Source {
            content: c.content,
            score: c.score,
        })
        .collect();

    Ok(Json(QueryResponse { answer, sources }))
}
