use reqwest::multipart;

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OcrSpaceResponse {
    parsed_results: Vec<ParsedResult>,
    is_errored_on_processing: bool,
    error_message: Option<String>,
    ocr_exit_code: i32,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ParsedResult {
    parsed_text: String,
}

#[derive(Clone)]
pub struct OcrService {
    api_key: String,
    client: reqwest::Client,
}

impl OcrService {
    pub fn new() -> Self {
        let api_key = std::env::var("OCR_SPACE_API_KEY")
            .expect("OCR_SPACE_API_KEY must be set");
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn extract_text(
        &self,
        image_data: &[u8],
        filename: &str,
    ) -> Result<String, String> {
        let file_part = multipart::Part::bytes(image_data.to_vec())
            .file_name(filename.to_string());

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("apikey", self.api_key.clone())
            .text("language", "eng")
            .text("isOverlayRequired", "false");

        let resp = self
            .client
            .post("https://api.ocr.space/parse/image")
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("ocr.space request failed: {e}"))?;

        let body: OcrSpaceResponse = resp
            .json()
            .await
            .map_err(|e| format!("ocr.space parse failed: {e}"))?;

        if body.is_errored_on_processing {
            return Err(body.error_message.unwrap_or("unknown ocr error".into()));
        }

        let text = body
            .parsed_results
            .into_iter()
            .map(|r| r.parsed_text)
            .collect::<Vec<_>>()
            .join("\n");

        Ok(text)
    }
}
