use crate::errors::AppError;

/// Trait for secure API key storage
pub trait ApiKeyStore: Send + Sync {
    fn get_api_key(&self) -> Result<Option<String>, AppError>;
    fn set_api_key(&self, key: &str) -> Result<(), AppError>;
    fn delete_api_key(&self) -> Result<(), AppError>;
}

/// Keychain-based API key storage using the `keyring` crate
pub struct KeychainApiKeyStore {
    service: String,
    username: String,
}

impl KeychainApiKeyStore {
    pub fn new() -> Self {
        Self {
            service: "snap-translate".to_string(),
            username: "deepl-api-key".to_string(),
        }
    }
}

impl ApiKeyStore for KeychainApiKeyStore {
    fn get_api_key(&self) -> Result<Option<String>, AppError> {
        let entry = keyring::Entry::new(&self.service, &self.username)
            .map_err(|e| AppError::KeychainError(e.to_string()))?;

        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AppError::KeychainError(e.to_string())),
        }
    }

    fn set_api_key(&self, key: &str) -> Result<(), AppError> {
        let entry = keyring::Entry::new(&self.service, &self.username)
            .map_err(|e| AppError::KeychainError(e.to_string()))?;

        entry
            .set_password(key)
            .map_err(|e| AppError::KeychainError(e.to_string()))
    }

    fn delete_api_key(&self) -> Result<(), AppError> {
        let entry = keyring::Entry::new(&self.service, &self.username)
            .map_err(|e| AppError::KeychainError(e.to_string()))?;

        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted, that's fine
            Err(e) => Err(AppError::KeychainError(e.to_string())),
        }
    }
}

/// Default settings values
pub const DEFAULT_TARGET_LANGUAGE: &str = "KO";
pub const DEFAULT_HOTKEY: &str = "CmdOrCtrl+Shift+D";

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;

    /// Mock ApiKeyStore for testing (uses HashMap instead of Keychain)
    pub struct MockApiKeyStore {
        store: Mutex<HashMap<String, String>>,
    }

    impl MockApiKeyStore {
        pub fn new() -> Self {
            Self {
                store: Mutex::new(HashMap::new()),
            }
        }

        pub fn with_key(key: &str) -> Self {
            let mock = Self::new();
            mock.store.lock().unwrap().insert("api_key".to_string(), key.to_string());
            mock
        }
    }

    impl ApiKeyStore for MockApiKeyStore {
        fn get_api_key(&self) -> Result<Option<String>, AppError> {
            Ok(self.store.lock().unwrap().get("api_key").cloned())
        }

        fn set_api_key(&self, key: &str) -> Result<(), AppError> {
            self.store.lock().unwrap().insert("api_key".to_string(), key.to_string());
            Ok(())
        }

        fn delete_api_key(&self) -> Result<(), AppError> {
            self.store.lock().unwrap().remove("api_key");
            Ok(())
        }
    }

    #[test]
    fn test_set_and_get_api_key() {
        let store = MockApiKeyStore::new();
        store.set_api_key("test-api-key:fx").unwrap();
        assert_eq!(store.get_api_key().unwrap(), Some("test-api-key:fx".to_string()));
    }

    #[test]
    fn test_get_api_key_when_empty() {
        let store = MockApiKeyStore::new();
        assert_eq!(store.get_api_key().unwrap(), None);
    }

    #[test]
    fn test_get_api_key_with_preset() {
        let store = MockApiKeyStore::with_key("preset-key:fx");
        assert_eq!(store.get_api_key().unwrap(), Some("preset-key:fx".to_string()));
    }

    #[test]
    fn test_delete_api_key() {
        let store = MockApiKeyStore::with_key("test-key");
        store.delete_api_key().unwrap();
        assert_eq!(store.get_api_key().unwrap(), None);
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let store = MockApiKeyStore::new();
        // Should not error when deleting a key that doesn't exist
        store.delete_api_key().unwrap();
    }

    #[test]
    fn test_overwrite_key() {
        let store = MockApiKeyStore::with_key("old-key");
        store.set_api_key("new-key").unwrap();
        assert_eq!(store.get_api_key().unwrap(), Some("new-key".to_string()));
    }

    #[test]
    fn test_default_values() {
        assert_eq!(DEFAULT_TARGET_LANGUAGE, "KO");
        assert_eq!(DEFAULT_HOTKEY, "CmdOrCtrl+Shift+D");
    }
}
