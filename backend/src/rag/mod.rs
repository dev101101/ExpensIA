// RAG pipeline:
// 1. Embed query via Cohere API
// 2. pgvector similarity search
// 3. Build context from top-k chunks
// 4. Generate answer via llama.cpp

pub mod embeddings;
pub mod retrieval;
pub mod generation;
