import { invoke } from "@tauri-apps/api/core";
import styles from "./LanguageSelect.module.css";
import { TARGET_LANGUAGES, type TargetLanguage } from "../types";

interface LanguageSelectProps {
  currentLanguage: string;
  onChange?: (language: string) => void;
}

export default function LanguageSelect({ currentLanguage, onChange }: LanguageSelectProps) {
  const handleChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newLang = e.target.value;
    try {
      await invoke("save_settings", { settings: { targetLanguage: newLang } });
      onChange?.(newLang);
    } catch (err) {
      console.error("Failed to save language:", err);
    }
  };

  return (
    <div className={styles.container}>
      <label className={styles.label}>번역 대상 언어</label>
      <select
        className={styles.select}
        value={currentLanguage}
        onChange={handleChange}
      >
        {(Object.entries(TARGET_LANGUAGES) as [TargetLanguage, string][]).map(
          ([code, name]) => (
            <option key={code} value={code}>
              {name} ({code})
            </option>
          )
        )}
      </select>
    </div>
  );
}
