pub struct EmbeddingService {
    api_key: String,
}

impl EmbeddingService {
    pub fn new() -> Self {
        let api_key = std::env::var("COHERE_API_KEY")
            .expect("COHERE_API_KEY must be set");
        Self { api_key }
    }

    pub async fn embed(&self, _text: &str) -> Vec<f32> {
        // TODO: call Cohere embed API
        vec![]
    }
}
