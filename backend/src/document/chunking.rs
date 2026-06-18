pub fn chunk_text(text: &str, max_chunk_size: usize) -> Vec<String> {
    // TODO: smart chunking by paragraphs/sentences
    text.chars()
        .collect::<Vec<_>>()
        .chunks(max_chunk_size)
        .map(|c| c.iter().collect())
        .collect()
}
