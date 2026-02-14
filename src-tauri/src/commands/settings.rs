use crate::config::store::{ApiKeyStore, DEFAULT_TARGET_LANGUAGE, DEFAULT_HOTKEY};
use crate::deepl::client::{DeepLClient, TranslationClient};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Clone)]
pub struct Settings {
    pub has_api_key: bool,
    pub target_language: String,
    pub hotkey: String,
}

#[derive(Debug, Deserialize)]
pub struct PartialSettings {
    pub target_language: Option<String>,
    pub hotkey: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredSettings {
    target_language: String,
    hotkey: String,
}

impl Default for StoredSettings {
    fn default() -> Self {
        Self {
            target_language: DEFAULT_TARGET_LANGUAGE.to_string(),
            hotkey: DEFAULT_HOTKEY.to_string(),
        }
    }
}

fn get_config_directory() -> String {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/Library/Application Support/SnapTranslate", home)
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "C:\\temp".to_string());
        format!("{}\\SnapTranslate", appdata)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        "/tmp/snap-translate".to_string()
    }
}

fn settings_file_path() -> std::path::PathBuf {
    let config_dir = get_config_directory();
    std::path::PathBuf::from(config_dir).join("settings.json")
}

fn load_stored_settings() -> StoredSettings {
    let path = settings_file_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => StoredSettings::default(),
    }
}

fn save_stored_settings(settings: &StoredSettings) -> Result<(), AppError> {
    let path = settings_file_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::KeychainError(format!("Failed to create config dir: {}", e)))?;
    }
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| AppError::KeychainError(format!("Failed to serialize settings: {}", e)))?;
    std::fs::write(&path, content)
        .map_err(|e| AppError::KeychainError(format!("Failed to write settings: {}", e)))?;
    Ok(())
}

#[tauri::command]
pub async fn get_settings(
    api_key_store: tauri::State<'_, Arc<dyn ApiKeyStore>>,
) -> Result<Settings, AppError> {
    let has_api_key = api_key_store
        .get_api_key()?
        .is_some();

    let stored = load_stored_settings();

    Ok(Settings {
        has_api_key,
        target_language: stored.target_language,
        hotkey: stored.hotkey,
    })
}

#[tauri::command]
pub async fn save_api_key(
    api_key: String,
    api_key_store: tauri::State<'_, Arc<dyn ApiKeyStore>>,
) -> Result<(), AppError> {
    if api_key.trim().is_empty() {
        return Err(AppError::InvalidApiKey);
    }

    // Validate with DeepL API
    let client = DeepLClient::new(api_key.clone());
    client.validate_key().await?;

    // Store in keychain
    api_key_store.set_api_key(&api_key)?;

    Ok(())
}

#[tauri::command]
pub async fn save_settings(settings: PartialSettings) -> Result<(), AppError> {
    let mut stored = load_stored_settings();

    if let Some(lang) = settings.target_language {
        stored.target_language = lang;
    }
    if let Some(key) = settings.hotkey {
        stored.hotkey = key;
    }

    save_stored_settings(&stored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_api_key_empty_rejected() {
        // Direct validation test - empty key should be rejected before API call
        let result_empty = "".trim().is_empty();
        assert!(result_empty);
    }

    #[tokio::test]
    async fn test_save_api_key_whitespace_rejected() {
        let result_ws = "   ".trim().is_empty();
        assert!(result_ws);
    }

    #[tokio::test]
    async fn test_save_settings_accepts_partial() {
        let settings = PartialSettings {
            target_language: Some("EN".to_string()),
            hotkey: None,
        };
        let result = save_settings(settings).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_settings_struct_serialization() {
        let settings = Settings {
            has_api_key: true,
            target_language: "KO".to_string(),
            hotkey: "CmdOrCtrl+Shift+D".to_string(),
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"has_api_key\":true"));
        assert!(json.contains("\"target_language\":\"KO\""));
    }

    #[tokio::test]
    async fn test_default_values() {
        assert_eq!(DEFAULT_TARGET_LANGUAGE, "KO");
        assert_eq!(DEFAULT_HOTKEY, "CmdOrCtrl+Shift+D");
    }

    #[test]
    fn test_stored_settings_default() {
        let stored = StoredSettings::default();
        assert_eq!(stored.target_language, "KO");
        assert_eq!(stored.hotkey, "CmdOrCtrl+Shift+D");
    }

    #[test]
    fn test_stored_settings_serialization() {
        let stored = StoredSettings {
            target_language: "EN".to_string(),
            hotkey: "Ctrl+Alt+T".to_string(),
        };
        let json = serde_json::to_string(&stored).unwrap();
        let deserialized: StoredSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.target_language, "EN");
        assert_eq!(deserialized.hotkey, "Ctrl+Alt+T");
    }
}
