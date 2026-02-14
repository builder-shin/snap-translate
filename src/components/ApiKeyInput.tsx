import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import styles from "./ApiKeyInput.module.css";

interface ApiKeyInputProps {
  hasApiKey: boolean;
  onSaved?: () => void;
}

export default function ApiKeyInput({ hasApiKey, onSaved }: ApiKeyInputProps) {
  const [apiKey, setApiKey] = useState("");
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSave = async () => {
    if (!apiKey.trim()) {
      setError("API Key를 입력해주세요.");
      return;
    }

    setLoading(true);
    setError("");
    setSuccess("");

    try {
      await invoke("save_api_key", { apiKey: apiKey.trim() });
      setSuccess("API Key가 설정되었습니다.");
      setApiKey("");
      onSaved?.();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className={styles.container}>
      <label className={styles.label}>DeepL API Key</label>
      {hasApiKey && !apiKey && (
        <p className={styles.status}>API Key가 설정되어 있습니다.</p>
      )}
      <div className={styles.inputGroup}>
        <input
          type="password"
          className={styles.input}
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder={hasApiKey ? "새 API Key 입력..." : "API Key 입력..."}
          disabled={loading}
        />
        <button
          className={styles.button}
          onClick={handleSave}
          disabled={loading || !apiKey.trim()}
        >
          {loading ? "검증 중..." : "저장"}
        </button>
      </div>
      {error && <p className={styles.error}>{error}</p>}
      {success && <p className={styles.success}>{success}</p>}
    </div>
  );
}
