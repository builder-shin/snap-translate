import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import ApiKeyInput from "../components/ApiKeyInput";
import LanguageSelect from "../components/LanguageSelect";
import HotkeyInput from "../components/HotkeyInput";
import type { AppSettings } from "../types";
import styles from "./Settings.module.css";

export default function Settings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);

  const loadSettings = async () => {
    try {
      const s = await invoke<AppSettings>("get_settings");
      setSettings(s);
    } catch (err) {
      console.error("Failed to load settings:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSettings();
  }, []);

  if (loading) {
    return <div className={styles.container}><p>로딩 중...</p></div>;
  }

  if (!settings) {
    return <div className={styles.container}><p>설정을 불러올 수 없습니다.</p></div>;
  }

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>Snap Translate 설정</h1>

      <ApiKeyInput
        hasApiKey={settings.hasApiKey}
        onSaved={loadSettings}
      />

      <LanguageSelect
        currentLanguage={settings.targetLanguage}
        onChange={(lang) =>
          setSettings((s) => s ? { ...s, targetLanguage: lang } : s)
        }
      />

      <HotkeyInput currentHotkey={settings.hotkey} />

      <div className={styles.footer}>
        <p className={styles.version}>Snap Translate v0.1.0</p>
      </div>
    </div>
  );
}
