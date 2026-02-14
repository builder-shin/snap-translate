use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;

pub fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = get_log_directory();

    let file_appender = rolling::daily(&log_dir, "snap-translate.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    tracing::info!("Snap Translate logging initialized");
    guard
}

pub fn get_log_directory() -> String {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/Library/Logs/SnapTranslate", home)
    }

    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "C:\\temp".to_string());
        format!("{}\\SnapTranslate\\logs", appdata)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        "/tmp/snap-translate/logs".to_string()
    }
}
