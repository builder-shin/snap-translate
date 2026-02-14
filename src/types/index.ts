export interface AppSettings {
  hasApiKey: boolean;
  targetLanguage: string;
  hotkey: string;
}

export interface TranslateResult {
  text: string;
  detectedSource: string;
}

export type TargetLanguage =
  | "KO"
  | "EN"
  | "JA"
  | "ZH"
  | "DE"
  | "FR"
  | "ES"
  | "PT"
  | "RU";

export const TARGET_LANGUAGES: Record<TargetLanguage, string> = {
  KO: "한국어",
  EN: "영어",
  JA: "일본어",
  ZH: "중국어 (간체)",
  DE: "독일어",
  FR: "프랑스어",
  ES: "스페인어",
  PT: "포르투갈어",
  RU: "러시아어",
};
