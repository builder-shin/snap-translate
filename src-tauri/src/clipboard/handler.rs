use crate::errors::AppError;

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardBackup {
    Text(String),
    Image(Vec<u8>),
    Empty,
}

pub trait ClipboardAccess: Send + Sync {
    fn read_text(&self) -> Result<String, AppError>;
    fn write_text(&self, text: &str) -> Result<(), AppError>;
    fn read_image(&self) -> Result<Vec<u8>, AppError>;
    fn write_image(&self, data: &[u8]) -> Result<(), AppError>;
    fn clear(&self) -> Result<(), AppError>;
}

/// Backup the current clipboard content
pub fn backup_clipboard(clipboard: &dyn ClipboardAccess) -> ClipboardBackup {
    // Try text first (most common)
    if let Ok(text) = clipboard.read_text() {
        if !text.is_empty() {
            return ClipboardBackup::Text(text);
        }
    }
    // Try image
    if let Ok(image) = clipboard.read_image() {
        if !image.is_empty() {
            return ClipboardBackup::Image(image);
        }
    }
    ClipboardBackup::Empty
}

/// Restore clipboard from backup
pub fn restore_clipboard(clipboard: &dyn ClipboardAccess, backup: &ClipboardBackup) -> Result<(), AppError> {
    match backup {
        ClipboardBackup::Text(text) => clipboard.write_text(text),
        ClipboardBackup::Image(data) => clipboard.write_image(data),
        ClipboardBackup::Empty => clipboard.clear(),
    }
}

/// Read clipboard text with retry-with-backoff
/// Compares against backup to detect if Cmd+C actually copied new text
pub async fn read_clipboard_with_retry(
    clipboard: &dyn ClipboardAccess,
    backup: &ClipboardBackup,
) -> Result<String, AppError> {
    let delays_ms = [50, 100, 200, 400, 800];

    for delay in delays_ms {
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;

        if let Ok(text) = clipboard.read_text() {
            if !text.is_empty() {
                let is_new = match backup {
                    ClipboardBackup::Text(old) => text != *old,
                    ClipboardBackup::Image(_) => true, // changed from image to text
                    ClipboardBackup::Empty => true,    // was empty, now has text
                };
                if is_new {
                    return Ok(text);
                }
            }
        }
    }

    Err(AppError::NothingSelected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockClipboard {
        text: Mutex<Option<String>>,
        image: Mutex<Option<Vec<u8>>>,
        read_count: Mutex<u32>,
        text_sequence: Mutex<Vec<String>>,
    }

    impl MockClipboard {
        fn new() -> Self {
            Self {
                text: Mutex::new(None),
                image: Mutex::new(None),
                read_count: Mutex::new(0),
                text_sequence: Mutex::new(vec![]),
            }
        }

        fn with_text(text: &str) -> Self {
            let mock = Self::new();
            *mock.text.lock().unwrap() = Some(text.to_string());
            mock
        }

        fn with_image(data: Vec<u8>) -> Self {
            let mock = Self::new();
            *mock.image.lock().unwrap() = Some(data);
            mock
        }

        /// Set a sequence of texts to return on successive read_text() calls
        fn with_text_sequence(texts: Vec<&str>) -> Self {
            let mock = Self::new();
            *mock.text_sequence.lock().unwrap() = texts.into_iter().map(String::from).collect();
            mock
        }
    }

    impl ClipboardAccess for MockClipboard {
        fn read_text(&self) -> Result<String, AppError> {
            let seq = self.text_sequence.lock().unwrap();
            if !seq.is_empty() {
                let mut count = self.read_count.lock().unwrap();
                let idx = (*count as usize).min(seq.len() - 1);
                *count += 1;
                let text = seq[idx].clone();
                if text.is_empty() {
                    return Err(AppError::ClipboardReadError);
                }
                return Ok(text);
            }
            drop(seq);

            self.text.lock().unwrap().clone().ok_or(AppError::ClipboardReadError)
        }

        fn write_text(&self, text: &str) -> Result<(), AppError> {
            *self.text.lock().unwrap() = Some(text.to_string());
            *self.image.lock().unwrap() = None;
            Ok(())
        }

        fn read_image(&self) -> Result<Vec<u8>, AppError> {
            self.image.lock().unwrap().clone().ok_or(AppError::ClipboardReadError)
        }

        fn write_image(&self, data: &[u8]) -> Result<(), AppError> {
            *self.image.lock().unwrap() = Some(data.to_vec());
            *self.text.lock().unwrap() = None;
            Ok(())
        }

        fn clear(&self) -> Result<(), AppError> {
            *self.text.lock().unwrap() = None;
            *self.image.lock().unwrap() = None;
            Ok(())
        }
    }

    #[test]
    fn test_text_read_write() {
        let clip = MockClipboard::new();
        clip.write_text("hello").unwrap();
        assert_eq!(clip.read_text().unwrap(), "hello");
    }

    #[test]
    fn test_image_read_write() {
        let clip = MockClipboard::new();
        let data = vec![0x89, 0x50, 0x4E, 0x47];
        clip.write_image(&data).unwrap();
        assert_eq!(clip.read_image().unwrap(), data);
    }

    #[test]
    fn test_clear() {
        let clip = MockClipboard::with_text("hello");
        clip.clear().unwrap();
        assert!(clip.read_text().is_err());
    }

    #[test]
    fn test_backup_text() {
        let clip = MockClipboard::with_text("original");
        let backup = backup_clipboard(&clip);
        assert_eq!(backup, ClipboardBackup::Text("original".to_string()));
    }

    #[test]
    fn test_backup_image() {
        let clip = MockClipboard::with_image(vec![0x89, 0x50]);
        let backup = backup_clipboard(&clip);
        assert_eq!(backup, ClipboardBackup::Image(vec![0x89, 0x50]));
    }

    #[test]
    fn test_backup_empty() {
        let clip = MockClipboard::new();
        let backup = backup_clipboard(&clip);
        assert_eq!(backup, ClipboardBackup::Empty);
    }

    #[test]
    fn test_restore_text() {
        let clip = MockClipboard::new();
        clip.write_text("temp").unwrap();
        let backup = ClipboardBackup::Text("original".to_string());
        restore_clipboard(&clip, &backup).unwrap();
        assert_eq!(clip.read_text().unwrap(), "original");
    }

    #[test]
    fn test_restore_image() {
        let clip = MockClipboard::new();
        clip.write_text("temp").unwrap();
        let backup = ClipboardBackup::Image(vec![0x89, 0x50]);
        restore_clipboard(&clip, &backup).unwrap();
        assert_eq!(clip.read_image().unwrap(), vec![0x89, 0x50]);
    }

    #[test]
    fn test_restore_empty() {
        let clip = MockClipboard::with_text("temp");
        let backup = ClipboardBackup::Empty;
        restore_clipboard(&clip, &backup).unwrap();
        assert!(clip.read_text().is_err());
    }

    #[tokio::test]
    async fn test_retry_first_attempt_success() {
        let clip = MockClipboard::with_text_sequence(vec!["new text"]);
        let backup = ClipboardBackup::Text("old".to_string());
        let result = read_clipboard_with_retry(&clip, &backup).await.unwrap();
        assert_eq!(result, "new text");
    }

    #[tokio::test]
    async fn test_retry_third_attempt_success() {
        let clip = MockClipboard::with_text_sequence(vec!["old", "old", "new text"]);
        let backup = ClipboardBackup::Text("old".to_string());
        let result = read_clipboard_with_retry(&clip, &backup).await.unwrap();
        assert_eq!(result, "new text");
    }

    #[tokio::test]
    async fn test_retry_all_fail() {
        let clip = MockClipboard::with_text_sequence(vec!["old", "old", "old", "old", "old"]);
        let backup = ClipboardBackup::Text("old".to_string());
        let result = read_clipboard_with_retry(&clip, &backup).await;
        assert!(matches!(result, Err(AppError::NothingSelected)));
    }

    #[tokio::test]
    async fn test_retry_empty_backup_any_text_succeeds() {
        let clip = MockClipboard::with_text_sequence(vec!["new text"]);
        let backup = ClipboardBackup::Empty;
        let result = read_clipboard_with_retry(&clip, &backup).await.unwrap();
        assert_eq!(result, "new text");
    }

    #[tokio::test]
    async fn test_retry_image_backup_text_succeeds() {
        let clip = MockClipboard::with_text_sequence(vec!["copied text"]);
        let backup = ClipboardBackup::Image(vec![0x89]);
        let result = read_clipboard_with_retry(&clip, &backup).await.unwrap();
        assert_eq!(result, "copied text");
    }
}
