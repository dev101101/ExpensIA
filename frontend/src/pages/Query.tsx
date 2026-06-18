import { useState } from "react";
import { queryRAG } from "../api/client";

interface RAGResult {
  answer: string;
  sources: { content: string; score: number }[];
}

export default function Query() {
  const [question, setQuestion] = useState("");
  const [result, setResult] = useState<RAGResult | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    try {
      const data = await queryRAG(question);
      setResult(data);
    } catch {
      alert("Query failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <section>
      <h2>RAG Query</h2>
      <form onSubmit={handleSubmit}>
        <input
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
          placeholder="Ask about your expenses..."
        />
        <button type="submit" disabled={loading}>
          {loading ? "Searching..." : "Ask"}
        </button>
      </form>
      {result && (
        <div className="result">
          <p>{result.answer}</p>
          {result.sources?.map((s, i) => (
            <details key={i}>
              <summary>Source (score: {s.score.toFixed(3)})</summary>
              <p>{s.content}</p>
            </details>
          ))}
        </div>
      )}
    </section>
  );
}
