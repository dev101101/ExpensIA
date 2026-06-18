use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LlamaRequest {
    prompt: String,
    n_predict: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct LlamaResponse {
    content: String,
}

#[derive(Clone)]
pub struct Generator {
    client: Client,
    url: String,
}

impl Generator {
    pub fn new() -> Self {
        let url = std::env::var("LLAMA_CPP_URL")
            .unwrap_or_else(|_| "http://localhost:8080".into());
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn generate(&self, query: &str, context: &str) -> String {
        let prompt = format!(
            r#"You are an expense analysis assistant. Use the following document excerpts to answer the question.

Context:
{context}

Question: {query}

Answer based only on the provided context. If the context doesn't contain enough information, say so."#
        );

        let body = LlamaRequest {
            prompt,
            n_predict: Some(512),
            temperature: Some(0.1),
        };

        match self
            .client
            .post(format!("{}/completion", self.url))
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<LlamaResponse>().await {
                Ok(data) => data.content,
                Err(_) => "LLM unavailable".into(),
            },
            Err(_) => "LLM unavailable".into(),
        }
    }
}
