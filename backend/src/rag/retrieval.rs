use sqlx::PgPool;

pub struct RetrievedChunk {
    pub content: String,
    pub score: f64,
    pub document_id: uuid::Uuid,
    pub chunk_index: i32,
}

pub struct Retriever;

impl Retriever {
    pub async fn retrieve(
        pool: &PgPool,
        embedding: &[f32],
        top_k: i64,
    ) -> Result<Vec<RetrievedChunk>, sqlx::Error> {
        let embedding_str: String = format!(
            "[{}]",
            embedding
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        sqlx::query_as::<_, (String, f64, uuid::Uuid, i32)>(
            r#"
            SELECT content, 1 - (embedding <=> $1::vector) AS score,
                   document_id, chunk_index
            FROM document_chunks
            WHERE embedding IS NOT NULL
            ORDER BY embedding <=> $1::vector
            LIMIT $2
            "#,
        )
        .bind(&embedding_str)
        .bind(top_k)
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|(content, score, document_id, chunk_index)| RetrievedChunk {
                    content,
                    score,
                    document_id,
                    chunk_index,
                })
                .collect()
        })
    }
}
