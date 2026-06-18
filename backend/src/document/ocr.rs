// OCR.space API integration
pub struct OcrService {
    api_key: String,
}

impl OcrService {
    pub fn new() -> Self {
        let api_key = std::env::var("OCR_SPACE_API_KEY")
            .expect("OCR_SPACE_API_KEY must be set");
        Self { api_key }
    }

    pub async fn extract_text(&self, _image_data: &[u8]) -> String {
        // TODO: call OCR.space API
        String::new()
    }
}
