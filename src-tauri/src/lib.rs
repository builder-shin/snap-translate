pub mod commands;
pub mod config;
pub mod deepl;
pub mod clipboard;
pub mod key_simulator;
pub mod hotkey;
pub mod accessibility;
pub mod logging;
pub mod errors;
pub mod translate_flow;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WebviewWindowBuilder,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_notification::NotificationExt;

use crate::accessibility::checker::AccessibilityChecker;
use crate::config::store::{ApiKeyStore, KeychainApiKeyStore};
use crate::errors::AppError;
use crate::hotkey::manager::create_default_shortcut;

// ---------------------------------------------------------------------------
// App State
// ---------------------------------------------------------------------------

/// Shared application state managed by Tauri.
pub struct AppState {
    pub is_translating: Arc<AtomicBool>,
}

// ---------------------------------------------------------------------------
// Settings Window
// ---------------------------------------------------------------------------

/// Create the settings window, or focus it if it already exists.
fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("settings") {
        let _ = win.set_focus();
        return;
    }

    let builder = WebviewWindowBuilder::new(app, "settings", tauri::WebviewUrl::App("/".into()))
        .title("Snap Translate 설정")
        .inner_size(480.0, 520.0)
        .resizable(false)
        .center();

    match builder.build() {
        Ok(_) => tracing::info!("Settings window opened"),
        Err(e) => tracing::error!("Failed to open settings window: {}", e),
    }
}

// ---------------------------------------------------------------------------
// Log Directory
// ---------------------------------------------------------------------------

/// Open the log directory in the system file manager.
fn open_log_directory() {
    let log_dir = crate::logging::get_log_directory();
    tracing::info!("Opening log directory: {}", log_dir);

    // Ensure the directory exists before opening
    let _ = std::fs::create_dir_all(&log_dir);

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&log_dir).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(&log_dir).spawn();
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(&log_dir).spawn();
    }
}

// ---------------------------------------------------------------------------
// Notifications
// ---------------------------------------------------------------------------

/// Send a desktop notification with the given title and body.
fn send_notification(app: &tauri::AppHandle, title: &str, body: &str) {
    if let Err(e) = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show()
    {
        tracing::error!("Failed to send notification: {}", e);
    }
}

/// Handle a FlowResult and send appropriate notification.
fn notify_flow_result(app: &tauri::AppHandle, result: translate_flow::FlowResult) {
    match result {
        translate_flow::FlowResult::Success { translated_text, .. } => {
            let preview: String = translated_text.chars().take(100).collect();
            send_notification(
                app,
                "번역 완료",
                &format!("번역 완료! 클립보드에 복사되었습니다.\n{}", preview),
            );
        }
        translate_flow::FlowResult::ApiKeyNotSet => {
            show_settings_window(app);
        }
        translate_flow::FlowResult::Error(e) => {
            match e {
                AppError::NetworkError(_) => {
                    send_notification(app, "번역 실패", "번역 실패: 네트워크 연결을 확인해주세요.");
                }
                AppError::InvalidApiKey => {
                    send_notification(app, "번역 실패", "번역 실패: API Key가 유효하지 않습니다.");
                }
                AppError::QuotaExceeded => {
                    send_notification(app, "번역 실패", "번역 실패: 번역 할당량이 초과되었습니다.");
                }
                AppError::RateLimited => {
                    send_notification(app, "번역 실패", "번역 실패: 요청이 너무 많습니다.");
                }
                AppError::NothingSelected => {
                    send_notification(app, "Snap Translate", "선택된 텍스트가 없습니다.");
                }
                AppError::TextTooLong { .. } => {
                    send_notification(app, "Snap Translate", "텍스트가 너무 깁니다. 최대 5,000자까지 번역할 수 있습니다.");
                }
                AppError::TranslationInProgress => {
                    tracing::debug!("Translation already in progress");
                }
                AppError::AccessibilityNotGranted => {
                    send_notification(app, "Snap Translate", "Accessibility 권한이 필요합니다. 시스템 설정에서 허용해주세요.");
                }
                other => {
                    send_notification(app, "번역 실패", &format!("오류가 발생했습니다: {}", other));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Hotkey Handler
// ---------------------------------------------------------------------------

/// Handle the translation hotkey press.
/// Delegates to `execute_translate_flow` from the tested `translate_flow` module.
fn handle_hotkey(app: &tauri::AppHandle) {
    let app_handle = app.clone();

    tauri::async_runtime::spawn(async move {
        use crate::accessibility::checker::PlatformAccessibilityChecker;
        use crate::deepl::client::DeepLClient;
        use crate::key_simulator::simulator::EnigoKeySimulator;
        use crate::translate_flow::execute_translate_flow;

        let state = app_handle.state::<AppState>();
        let is_translating = Arc::clone(&state.is_translating);

        // Get API key to create DeepL client
        let key_store = app_handle.state::<Arc<dyn ApiKeyStore>>();
        let api_key = match key_store.get_api_key() {
            Ok(Some(key)) => key,
            Ok(None) => {
                notify_flow_result(&app_handle, translate_flow::FlowResult::ApiKeyNotSet);
                return;
            }
            Err(e) => {
                notify_flow_result(&app_handle, translate_flow::FlowResult::Error(e));
                return;
            }
        };

        let accessibility = PlatformAccessibilityChecker::new();
        let clipboard = TauriClipboard::new(app_handle.clone());
        let key_sim = EnigoKeySimulator::new();
        let client = DeepLClient::new(api_key);
        let target_lang = crate::config::store::DEFAULT_TARGET_LANGUAGE;

        let result = execute_translate_flow(
            &is_translating,
            &accessibility,
            &**key_store,
            &clipboard,
            &key_sim,
            &client,
            target_lang,
        ).await;

        notify_flow_result(&app_handle, result);
    });
}

// ---------------------------------------------------------------------------
// Tauri Clipboard Adapter
// ---------------------------------------------------------------------------

/// Adapter that implements ClipboardAccess using tauri_plugin_clipboard_manager.
struct TauriClipboard {
    app: tauri::AppHandle,
}

impl TauriClipboard {
    fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}

impl crate::clipboard::handler::ClipboardAccess for TauriClipboard {
    fn read_text(&self) -> Result<String, AppError> {
        use tauri_plugin_clipboard_manager::ClipboardExt;
        self.app
            .clipboard()
            .read_text()
            .map_err(|_| AppError::ClipboardReadError)
    }

    fn write_text(&self, text: &str) -> Result<(), AppError> {
        use tauri_plugin_clipboard_manager::ClipboardExt;
        self.app
            .clipboard()
            .write_text(text)
            .map_err(|_| AppError::ClipboardWriteError)
    }

    fn read_image(&self) -> Result<Vec<u8>, AppError> {
        use tauri_plugin_clipboard_manager::ClipboardExt;
        let image = self
            .app
            .clipboard()
            .read_image()
            .map_err(|_| AppError::ClipboardReadError)?;
        Ok(image.rgba().to_vec())
    }

    fn write_image(&self, _data: &[u8]) -> Result<(), AppError> {
        // Image write-back is not critical for the translation flow;
        // raw bytes cannot be directly written back through the plugin
        // without knowing dimensions. For clipboard restore we only need text.
        Err(AppError::ClipboardWriteError)
    }

    fn clear(&self) -> Result<(), AppError> {
        use tauri_plugin_clipboard_manager::ClipboardExt;
        self.app
            .clipboard()
            .write_text("")
            .map_err(|_| AppError::ClipboardWriteError)
    }
}

// ---------------------------------------------------------------------------
// App Entry Point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging first — hold the guard so the worker thread stays alive
    let _log_guard = logging::init_logging();

    let api_key_store: Arc<dyn ApiKeyStore> = Arc::new(KeychainApiKeyStore::new());

    let builder = tauri::Builder::default();

    builder
        .manage(api_key_store)
        .manage(AppState {
            is_translating: Arc::new(AtomicBool::new(false)),
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::translate::translate,
            commands::settings::get_settings,
            commands::settings::save_api_key,
            commands::settings::save_settings,
        ])
        .setup(|app| {
            let handle = app.handle();

            // --- System Tray ---
            setup_system_tray(handle)?;

            // --- Global Shortcut ---
            setup_global_shortcut(handle)?;

            // --- Check Accessibility (prompt on first run) ---
            let checker = accessibility::checker::PlatformAccessibilityChecker::new();
            if !checker.is_trusted() {
                tracing::info!("Accessibility not granted, prompting user...");
                checker.check_and_prompt();
            }

            tracing::info!("Snap Translate setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ---------------------------------------------------------------------------
// System Tray Setup
// ---------------------------------------------------------------------------

fn setup_system_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let settings_item = MenuItem::with_id(app, "settings", "설정", true, None::<&str>)?;
    let open_log_item = MenuItem::with_id(app, "open_log", "로그 열기", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&settings_item, &open_log_item, &quit_item])?;

    let tray_icon = app.default_window_icon().cloned().unwrap_or_else(|| {
        tauri::image::Image::from_path("icons/tray-icon.png")
            .expect("Failed to load tray icon")
    });

    let _tray = TrayIconBuilder::new()
        .icon(tray_icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                show_settings_window(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                show_settings_window(app);
            }
            "open_log" => {
                open_log_directory();
            }
            "quit" => {
                tracing::info!("Quit requested via tray menu");
                app.exit(0);
            }
            _ => {
                tracing::warn!("Unknown tray menu event: {:?}", event.id);
            }
        })
        .build(app)?;

    tracing::info!("System tray initialized");
    Ok(())
}

// ---------------------------------------------------------------------------
// Global Shortcut Setup
// ---------------------------------------------------------------------------

fn setup_global_shortcut(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = create_default_shortcut();

    app.global_shortcut().on_shortcut(shortcut, move |app, _shortcut, event| {
        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            handle_hotkey(app);
        }
    })?;

    tracing::info!("Global shortcut registered: Cmd/Ctrl+Shift+D");
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_app_state_default() {
        let state = AppState {
            is_translating: Arc::new(AtomicBool::new(false)),
        };
        assert!(!state.is_translating.load(Ordering::SeqCst));
    }

    #[test]
    fn test_get_log_directory_not_empty() {
        let dir = crate::logging::get_log_directory();
        assert!(!dir.is_empty());
    }
}
