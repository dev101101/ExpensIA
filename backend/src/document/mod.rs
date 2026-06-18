// Document processing pipeline:
// 1. Receive uploaded file
// 2. OCR via OCR.space API
// 3. Text chunking
// 4. Embedding via Cohere API

pub mod ocr;
pub mod chunking;
