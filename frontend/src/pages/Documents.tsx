import { useState } from "react";
import { uploadDocument } from "../api/client";

export default function Documents() {
  const [uploading, setUploading] = useState(false);

  async function handleUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    setUploading(true);
    try {
      await uploadDocument(file);
      alert("Uploaded successfully");
    } catch {
      alert("Upload failed");
    } finally {
      setUploading(false);
    }
  }

  return (
    <section>
      <h2>Documents</h2>
      <label className="upload-btn">
        {uploading ? "Uploading..." : "Upload Invoice / Receipt"}
        <input type="file" hidden onChange={handleUpload} accept="image/*,.pdf" />
      </label>
    </section>
  );
}
