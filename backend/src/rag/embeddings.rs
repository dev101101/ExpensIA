#[derive(serde::Serialize)]
struct CohereEmbedRequest {
    texts: Vec<String>,
    model: String,
    input_type: String,
    embedding_types: Vec<String>,
}

#[derive(serde::Deserialize)]
struct CohereEmbedResponse {
    embeddings: CohereEmbeddings,
}

#[derive(serde::Deserialize)]
struct CohereEmbeddings {
    #[serde(rename = "float")]
    float_: Vec<Vec<f32>>,
}

#[derive(Clone)]
pub struct EmbeddingService {
    api_key: String,
    client: reqwest::Client,
}

impl EmbeddingService {
    pub fn new() -> Self {
        let api_key = std::env::var("COHERE_API_KEY")
            .expect("COHERE_API_KEY must be set");
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        let mut result = self.embed_batch(&[text]).await?;
        result.pop().ok_or("empty embedding result".into())
    }

    pub async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        let body = CohereEmbedRequest {
            texts: texts.iter().map(|t| t.to_string()).collect(),
            model: "embed-english-v3.0".into(),
            input_type: "search_document".into(),
            embedding_types: vec!["float".into()],
        };

        let resp = self
            .client
            .post("https://api.cohere.ai/v2/embed")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("cohere request failed: {e}"))?;

        let data: CohereEmbedResponse = resp
            .json()
            .await
            .map_err(|e| format!("cohere parse failed: {e}"))?;

        Ok(data.embeddings.float_)
    }
}
