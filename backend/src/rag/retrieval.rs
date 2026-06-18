pub struct Retriever;

impl Retriever {
    pub async fn retrieve(&self, _embedding: &[f32], _top_k: i64) -> Vec<String> {
        // TODO: pgvector similarity search
        vec![]
    }
}
