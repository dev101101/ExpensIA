use sqlx::PgPool;
use uuid::Uuid;

pub struct DocumentRepo;

impl DocumentRepo {
    pub async fn insert(
        pool: &PgPool,
        filename: &str,
        mime_type: Option<&str>,
    ) -> Result<Uuid, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            INSERT INTO documents (filename, mime_type, status)
            VALUES ($1, $2, 'pending')
            RETURNING id
            "#,
        )
        .bind(filename)
        .bind(mime_type)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: &str,
        raw_text: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE documents SET status = $1, raw_text = COALESCE($2, raw_text), updated_at = now()
            WHERE id = $3
            "#,
        )
        .bind(status)
        .bind(raw_text)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

pub struct ChunkRepo;

impl ChunkRepo {
    pub async fn insert_with_embedding(
        pool: &PgPool,
        document_id: Uuid,
        chunk_index: i32,
        content: &str,
        embedding: &[f32],
    ) -> Result<(), sqlx::Error> {
        let embedding_str: String = format!(
            "[{}]",
            embedding
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        sqlx::query(
            r#"
            INSERT INTO document_chunks (document_id, chunk_index, content, embedding)
            VALUES ($1, $2, $3, $4::vector)
            "#,
        )
        .bind(document_id)
        .bind(chunk_index)
        .bind(content)
        .bind(&embedding_str)
        .execute(pool)
        .await?;
        Ok(())
    }
}
