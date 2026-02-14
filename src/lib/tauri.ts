import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, TranslateResult } from "../types";

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function saveApiKey(apiKey: string): Promise<void> {
  return invoke("save_api_key", { apiKey });
}

export async function saveSettings(settings: Partial<AppSettings>): Promise<void> {
  return invoke("save_settings", { settings });
}

export async function translate(
  text: string,
  targetLang: string,
  sourceLang?: string,
): Promise<TranslateResult> {
  return invoke<TranslateResult>("translate", { text, targetLang, sourceLang });
}
