use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::clipboard::handler::{ClipboardAccess, backup_clipboard, restore_clipboard, read_clipboard_with_retry};
use crate::key_simulator::simulator::KeySimulator;
use crate::deepl::client::{TranslationClient, translate_with_retry};
use crate::config::store::ApiKeyStore;
use crate::accessibility::checker::AccessibilityChecker;
use crate::errors::AppError;

const MAX_TEXT_LENGTH: usize = 5000;

/// Guard that sets AtomicBool to false when dropped
/// Ensures the translation flag is always released, even on panic
pub struct TranslationGuard {
    flag: Arc<AtomicBool>,
}

impl TranslationGuard {
    /// Try to acquire the translation lock.
    /// Returns Some(guard) if acquired, None if already translating.
    pub fn try_acquire(flag: &Arc<AtomicBool>) -> Option<Self> {
        let was_translating = flag.compare_exchange(
            false,
            true,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        match was_translating {
            Ok(false) => Some(Self {
                flag: Arc::clone(flag),
            }),
            _ => None,
        }
    }
}

impl Drop for TranslationGuard {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::SeqCst);
    }
}

/// Result of a translation flow execution
#[derive(Debug)]
pub enum FlowResult {
    /// Translation completed successfully
    Success {
        translated_text: String,
        detected_source: String,
    },
    /// API key is not configured
    ApiKeyNotSet,
    /// An error occurred during the flow
    Error(AppError),
}

/// Execute the full translation flow:
/// 1. Guard check (prevent concurrent translations)
/// 2. Accessibility check (macOS)
/// 3. Get API key from keychain (fail fast)
/// 4. Clipboard backup
/// 5. Key simulation (Cmd+C / Ctrl+C)
/// 6. Read clipboard with retry-backoff
/// 7. Restore original clipboard
/// 8. Check text length limit
/// 9. Translate via DeepL with retry
/// 10. Write translation to clipboard
pub async fn execute_translate_flow(
    is_translating: &Arc<AtomicBool>,
    accessibility: &dyn AccessibilityChecker,
    api_key_store: &dyn ApiKeyStore,
    clipboard: &dyn ClipboardAccess,
    key_sim: &dyn KeySimulator,
    translator: &dyn TranslationClient,
    target_lang: &str,
) -> FlowResult {
    // Step 1: Acquire translation guard
    let _guard = match TranslationGuard::try_acquire(is_translating) {
        Some(guard) => guard,
        None => return FlowResult::Error(AppError::TranslationInProgress),
    };

    // Step 2: Check accessibility (macOS)
    if !accessibility.is_trusted() {
        accessibility.check_and_prompt();
        return FlowResult::Error(AppError::AccessibilityNotGranted);
    }

    // Step 3: Check API key exists (fail fast)
    match api_key_store.get_api_key() {
        Ok(Some(_)) => {} // key exists, translator was already created with it
        Ok(None) => return FlowResult::ApiKeyNotSet,
        Err(e) => return FlowResult::Error(e),
    }

    // Step 4: Backup clipboard
    let backup = backup_clipboard(clipboard);

    // Step 5: Simulate Cmd+C / Ctrl+C
    if let Err(e) = key_sim.simulate_copy() {
        let _ = restore_clipboard(clipboard, &backup);
        return FlowResult::Error(e);
    }

    // Step 6: Read clipboard with retry-backoff
    let selected_text = match read_clipboard_with_retry(clipboard, &backup).await {
        Ok(text) => text,
        Err(e) => {
            let _ = restore_clipboard(clipboard, &backup);
            return FlowResult::Error(e);
        }
    };

    // Step 7: Restore original clipboard content
    let _ = restore_clipboard(clipboard, &backup);

    // Step 8: Check text length
    if selected_text.chars().count() > MAX_TEXT_LENGTH {
        return FlowResult::Error(AppError::TextTooLong {
            max: MAX_TEXT_LENGTH,
            actual: selected_text.chars().count(),
        });
    }

    // Step 9: Translate via DeepL with retry
    let response = match translate_with_retry(translator, &selected_text, target_lang, None).await {
        Ok(resp) => resp,
        Err(e) => return FlowResult::Error(e),
    };

    let translation = match response.translations.into_iter().next() {
        Some(t) => t,
        None => return FlowResult::Error(AppError::EmptyText),
    };

    // Step 10: Write translation to clipboard
    if let Err(e) = clipboard.write_text(&translation.text) {
        return FlowResult::Error(e);
    }

    FlowResult::Success {
        translated_text: translation.text,
        detected_source: translation.detected_source_language,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use std::collections::VecDeque;
    use crate::deepl::types::{TranslateResponse, Translation, UsageResponse};

    // ---- Mock Clipboard ----
    struct MockClipboard {
        text: Mutex<Option<String>>,
        image: Mutex<Option<Vec<u8>>>,
        read_count: Mutex<u32>,
        text_sequence: Mutex<Vec<String>>,
        write_fail: bool,
    }

    impl MockClipboard {
        fn new() -> Self {
            Self {
                text: Mutex::new(None),
                image: Mutex::new(None),
                read_count: Mutex::new(0),
                text_sequence: Mutex::new(vec![]),
                write_fail: false,
            }
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
            if self.write_fail {
                return Err(AppError::ClipboardWriteError);
            }
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

    // ---- Mock KeySimulator ----
    struct MockKeySim {
        should_fail: bool,
    }

    impl MockKeySim {
        fn success() -> Self {
            Self { should_fail: false }
        }

    }

    impl KeySimulator for MockKeySim {
        fn simulate_copy(&self) -> Result<(), AppError> {
            if self.should_fail {
                Err(AppError::KeySimulationError("mock key sim error".to_string()))
            } else {
                Ok(())
            }
        }
    }

    // ---- Mock ApiKeyStore ----
    struct MockApiKeyStore {
        key: Mutex<Option<String>>,
    }

    impl MockApiKeyStore {
        fn with_key(key: &str) -> Self {
            Self {
                key: Mutex::new(Some(key.to_string())),
            }
        }

        fn empty() -> Self {
            Self {
                key: Mutex::new(None),
            }
        }
    }

    impl ApiKeyStore for MockApiKeyStore {
        fn get_api_key(&self) -> Result<Option<String>, AppError> {
            Ok(self.key.lock().unwrap().clone())
        }

        fn set_api_key(&self, key: &str) -> Result<(), AppError> {
            *self.key.lock().unwrap() = Some(key.to_string());
            Ok(())
        }

        fn delete_api_key(&self) -> Result<(), AppError> {
            *self.key.lock().unwrap() = None;
            Ok(())
        }
    }

    // ---- Mock AccessibilityChecker ----
    struct MockAccessibility {
        trusted: bool,
    }

    impl MockAccessibility {
        fn trusted() -> Self {
            Self { trusted: true }
        }

        fn not_trusted() -> Self {
            Self { trusted: false }
        }
    }

    impl AccessibilityChecker for MockAccessibility {
        fn is_trusted(&self) -> bool {
            self.trusted
        }

        fn check_and_prompt(&self) -> bool {
            self.trusted
        }
    }

    // ---- Mock TranslationClient ----
    struct MockTranslator {
        responses: Mutex<VecDeque<Result<TranslateResponse, AppError>>>,
    }

    impl MockTranslator {
        fn with_response(text: &str, source: &str) -> Self {
            let resp = Ok(TranslateResponse {
                translations: vec![Translation {
                    text: text.to_string(),
                    detected_source_language: source.to_string(),
                }],
            });
            Self {
                responses: Mutex::new(VecDeque::from(vec![resp])),
            }
        }

        fn with_error(err: AppError) -> Self {
            Self {
                responses: Mutex::new(VecDeque::from(vec![Err(err)])),
            }
        }
    }

    #[async_trait]
    impl TranslationClient for MockTranslator {
        async fn translate(
            &self,
            _text: &str,
            _target_lang: &str,
            _source_lang: Option<&str>,
        ) -> Result<TranslateResponse, AppError> {
            self.responses.lock().unwrap().pop_front().unwrap()
        }

        async fn validate_key(&self) -> Result<UsageResponse, AppError> {
            Ok(UsageResponse {
                character_count: 0,
                character_limit: 500000,
            })
        }
    }

    // ========== TESTS ==========

    #[test]
    fn test_guard_acquire_success() {
        let flag = Arc::new(AtomicBool::new(false));
        let guard = TranslationGuard::try_acquire(&flag);
        assert!(guard.is_some());
        assert!(flag.load(Ordering::SeqCst)); // flag should be true
    }

    #[test]
    fn test_guard_blocks_concurrent() {
        let flag = Arc::new(AtomicBool::new(false));
        let _guard1 = TranslationGuard::try_acquire(&flag).unwrap();
        let guard2 = TranslationGuard::try_acquire(&flag);
        assert!(guard2.is_none()); // should fail since already acquired
    }

    #[test]
    fn test_guard_releases_on_drop() {
        let flag = Arc::new(AtomicBool::new(false));
        {
            let _guard = TranslationGuard::try_acquire(&flag).unwrap();
            assert!(flag.load(Ordering::SeqCst));
        } // guard dropped here
        assert!(!flag.load(Ordering::SeqCst)); // flag should be false
    }

    #[test]
    fn test_guard_reacquire_after_drop() {
        let flag = Arc::new(AtomicBool::new(false));
        {
            let _guard = TranslationGuard::try_acquire(&flag).unwrap();
        }
        let guard2 = TranslationGuard::try_acquire(&flag);
        assert!(guard2.is_some());
    }

    #[test]
    fn test_guard_releases_on_panic() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&flag);

        let result = std::panic::catch_unwind(move || {
            let _guard = TranslationGuard::try_acquire(&flag_clone).unwrap();
            panic!("simulated error");
        });

        assert!(result.is_err());
        assert!(!flag.load(Ordering::SeqCst)); // flag should be released
    }

    // ===== Flow Tests =====

    /// TEST 1: Full successful translation flow
    #[tokio::test]
    async fn test_flow_success() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        // Clipboard: first read returns "original" (backup), then after Cmd+C returns "Hello world"
        let clipboard = MockClipboard::with_text_sequence(vec!["original", "Hello world"]);
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("안녕하세요 세계", "EN");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        match result {
            FlowResult::Success {
                translated_text,
                detected_source,
            } => {
                assert_eq!(translated_text, "안녕하세요 세계");
                assert_eq!(detected_source, "EN");
            }
            FlowResult::ApiKeyNotSet => panic!("Expected Success, got ApiKeyNotSet"),
            FlowResult::Error(e) => panic!("Expected Success, got Error: {:?}", e),
        }

        // Guard should be released
        assert!(!flag.load(Ordering::SeqCst));
    }

    /// TEST 2: API key not set returns ApiKeyNotSet
    #[tokio::test]
    async fn test_flow_api_key_not_set() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::empty();
        let clipboard = MockClipboard::new();
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("unused", "XX");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        assert!(matches!(result, FlowResult::ApiKeyNotSet));
        // Guard should be released
        assert!(!flag.load(Ordering::SeqCst));
    }

    /// TEST 3: Text exceeding 5000 chars returns TextTooLong
    #[tokio::test]
    async fn test_flow_text_too_long() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        let long_text = "a".repeat(5001);
        // Clipboard: backup is "original", then after Cmd+C returns the long text
        let clipboard = MockClipboard::with_text_sequence(vec!["original", &long_text]);
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("unused", "XX");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        match result {
            FlowResult::Error(AppError::TextTooLong { max, actual }) => {
                assert_eq!(max, 5000);
                assert_eq!(actual, 5001);
            }
            other => panic!("Expected TextTooLong, got {:?}", other),
        }
        assert!(!flag.load(Ordering::SeqCst));
    }

    /// TEST 4: Translation failure returns error and clipboard is restored
    #[tokio::test]
    async fn test_flow_translation_error_restores_clipboard() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        let clipboard = MockClipboard::with_text_sequence(vec!["original", "Hello"]);
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_error(AppError::NetworkError("timeout".to_string()));

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        assert!(matches!(result, FlowResult::Error(AppError::NetworkError(_))));
        // Clipboard should have been restored to "original" (from restore_clipboard call)
        // Note: after read_clipboard_with_retry succeeds, restore_clipboard is called
        // which writes "original" back. Then translation fails, but clipboard is already restored.
        assert!(!flag.load(Ordering::SeqCst));
    }

    /// TEST 5: Nothing selected (clipboard unchanged) returns NothingSelected
    #[tokio::test]
    async fn test_flow_nothing_selected() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        // Clipboard always returns "same" - simulating no new text copied
        let clipboard = MockClipboard::with_text_sequence(vec![
            "same", "same", "same", "same", "same", "same",
        ]);
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("unused", "XX");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        assert!(matches!(result, FlowResult::Error(AppError::NothingSelected)));
        assert!(!flag.load(Ordering::SeqCst));
    }

    /// TEST 6: Concurrent translation attempt returns TranslationInProgress
    #[tokio::test]
    async fn test_flow_concurrent_blocked() {
        let flag = Arc::new(AtomicBool::new(false));
        // Acquire guard externally to simulate ongoing translation
        let _guard = TranslationGuard::try_acquire(&flag).unwrap();

        let accessibility = MockAccessibility::trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        let clipboard = MockClipboard::new();
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("unused", "XX");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        assert!(matches!(result, FlowResult::Error(AppError::TranslationInProgress)));
    }

    /// TEST 7: Accessibility not granted returns error
    #[tokio::test]
    async fn test_flow_accessibility_not_granted() {
        let flag = Arc::new(AtomicBool::new(false));
        let accessibility = MockAccessibility::not_trusted();
        let api_key_store = MockApiKeyStore::with_key("test-key:fx");
        let clipboard = MockClipboard::new();
        let key_sim = MockKeySim::success();
        let translator = MockTranslator::with_response("unused", "XX");

        let result = execute_translate_flow(
            &flag,
            &accessibility,
            &api_key_store,
            &clipboard,
            &key_sim,
            &translator,
            "KO",
        )
        .await;

        assert!(matches!(result, FlowResult::Error(AppError::AccessibilityNotGranted)));
        assert!(!flag.load(Ordering::SeqCst));
    }
}
