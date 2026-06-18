pub fn chunk_text(text: &str, max_chunk_size: usize) -> Vec<String> {
    let paragraphs: Vec<&str> = text.split('\n').filter(|p| !p.trim().is_empty()).collect();
    let mut chunks = Vec::new();
    let mut current = String::new();

    for p in paragraphs {
        if current.len() + p.len() > max_chunk_size && !current.is_empty() {
            chunks.push(current.trim().to_string());
            current = String::new();
        }

        if p.len() > max_chunk_size {
            if !current.is_empty() {
                chunks.push(current.trim().to_string());
                current = String::new();
            }
            for sentence in split_sentences(p) {
                if current.len() + sentence.len() > max_chunk_size && !current.is_empty() {
                    chunks.push(current.trim().to_string());
                    current = String::new();
                }
                current.push_str(&sentence);
                current.push(' ');
            }
        } else {
            current.push_str(p);
            current.push('\n');
        }
    }

    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }

    chunks
}

fn split_sentences(text: &str) -> Vec<&str> {
    let mut sentences = Vec::new();
    let mut start = 0;
    for (i, c) in text.char_indices() {
        if c == '.' || c == '!' || c == '?' {
            let s = text[start..=i].trim();
            if !s.is_empty() {
                sentences.push(s);
            }
            start = i + 1;
        }
    }
    if start < text.len() {
        let s = text[start..].trim();
        if !s.is_empty() {
            sentences.push(s);
        }
    }
    sentences
}
