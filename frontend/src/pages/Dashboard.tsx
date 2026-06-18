import { useEffect, useState } from "react";
import { getHealth } from "../api/client";

export default function Dashboard() {
  const [status, setStatus] = useState("loading...");

  useEffect(() => {
    getHealth().then(setStatus).catch(() => setStatus("offline"));
  }, []);

  return (
    <section>
      <h2>Dashboard</h2>
      <p>API Status: <strong>{status}</strong></p>
    </section>
  );
}
