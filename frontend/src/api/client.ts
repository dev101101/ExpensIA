const BASE = import.meta.env.VITE_API_URL ?? "/api";

export async function getHealth(): Promise<string> {
  const res = await fetch(`${BASE}/health`);
  return res.text();
}

export async function uploadDocument(file: File): Promise<void> {
  const form = new FormData();
  form.append("file", file);
  await fetch(`${BASE}/documents/upload`, { method: "POST", body: form });
}

export async function queryRAG(question: string) {
  const res = await fetch(`${BASE}/query`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ question }),
  });
  return res.json();
}
