export interface Document {
  id: string;
  filename: string;
  mime_type: string | null;
  raw_text: string | null;
  status: "pending" | "processing" | "done" | "error";
  created_at: string;
  updated_at: string;
}

export interface Expense {
  id: string;
  document_id: string | null;
  category_id: string | null;
  amount: number | null;
  currency: string;
  vendor: string | null;
  date: string | null;
  description: string | null;
  confidence: number | null;
  created_at: string;
}

export interface RAGResult {
  answer: string;
  sources: { chunk_id: string; content: string; score: number }[];
}
