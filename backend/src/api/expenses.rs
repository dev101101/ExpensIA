use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::api::ApiError;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list))
}

async fn list(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query_as::<_, (serde_json::Value,)>(
        r#"
        SELECT json_build_object(
            'id', e.id,
            'amount', e.amount,
            'currency', e.currency,
            'vendor', e.vendor,
            'date', e.date,
            'description', e.description,
            'confidence', e.confidence,
            'category_name', c.name,
            'created_at', e.created_at
        )
        FROM expenses e
        LEFT JOIN categories c ON c.id = e.category_id
        ORDER BY e.created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let expenses: Vec<serde_json::Value> = rows.into_iter().map(|(v,)| v).collect();
    Ok(Json(json!({ "expenses": expenses })))
}
