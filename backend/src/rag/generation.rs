pub struct Generator;

impl Generator {
    pub async fn generate(&self, _query: &str, _context: &str) -> String {
        // TODO: call local llama.cpp server
        String::new()
    }
}
