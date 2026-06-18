use axum::extract::{Multipart, State};
use axum::routing::post;
use axum::Router;
use serde_json::json;

use crate::api::ApiError;
use crate::db::repository::{ChunkRepo, DocumentRepo};
use crate::document::chunking;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}

async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?
        .ok_or(ApiError::BadRequest("no file provided".into()))?;

    let filename = field
        .file_name()
        .unwrap_or("upload")
        .to_string();
    let mime_type = field
        .content_type()
        .map(|s| s.to_string());
    let data = field
        .bytes()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    if data.is_empty() {
        return Err(ApiError::BadRequest("empty file".into()));
    }

    let doc_id = DocumentRepo::insert(&state.pool, &filename, mime_type.as_deref())
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let raw_text = state
        .ocr
        .extract_text(&data, &filename)
        .await
        .map_err(|e| {
            let _ = DocumentRepo::update_status(&state.pool, doc_id, "error", None);
            ApiError::Processing(e)
        })?;

    DocumentRepo::update_status(&state.pool, doc_id, "processing", Some(&raw_text))
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let chunks = chunking::chunk_text(&raw_text, 512);

    let texts: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();
    let embeddings = state
        .embedder
        .embed_batch(&texts)
        .await
        .map_err(|e| {
            let _ = DocumentRepo::update_status(&state.pool, doc_id, "error", None);
            ApiError::Processing(e)
        })?;

    for (i, (chunk, emb)) in chunks.iter().zip(embeddings.iter()).enumerate() {
        ChunkRepo::insert_with_embedding(&state.pool, doc_id, i as i32, chunk, emb)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;
    }

    DocumentRepo::update_status(&state.pool, doc_id, "done", None)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(axum::Json(json!({
        "id": doc_id,
        "filename": filename,
        "chunks": chunks.len(),
        "status": "done"
    })))
}
