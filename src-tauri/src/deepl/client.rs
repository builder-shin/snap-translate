use async_trait::async_trait;
use crate::errors::AppError;
use super::types::{TranslateRequest, TranslateResponse, UsageResponse};
use std::time::Duration;

const MAX_TEXT_LENGTH: usize = 5000;
const MAX_RETRIES: u32 = 4;

#[async_trait]
pub trait TranslationClient: Send + Sync {
    async fn translate(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
    ) -> Result<TranslateResponse, AppError>;

    async fn validate_key(&self) -> Result<UsageResponse, AppError>;
}

pub struct DeepLClient {
    api_key: String,
    http_client: reqwest::Client,
    base_url: String,
}

impl DeepLClient {
    pub fn new(api_key: String) -> Self {
        let base_url = Self::detect_base_url(&api_key).to_string();
        Self {
            api_key,
            http_client: reqwest::Client::new(),
            base_url,
        }
    }

    pub fn detect_base_url(api_key: &str) -> &'static str {
        if api_key.ends_with(":fx") {
            "https://api-free.deepl.com"
        } else {
            "https://api.deepl.com"
        }
    }
}

#[async_trait]
impl TranslationClient for DeepLClient {
    async fn translate(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
    ) -> Result<TranslateResponse, AppError> {
        if text.is_empty() {
            return Err(AppError::EmptyText);
        }

        if text.chars().count() > MAX_TEXT_LENGTH {
            return Err(AppError::TextTooLong {
                max: MAX_TEXT_LENGTH,
                actual: text.chars().count(),
            });
        }

        let request = TranslateRequest {
            text: vec![text.to_string()],
            target_lang: target_lang.to_string(),
            source_lang: source_lang.map(|s| s.to_string()),
        };

        let url = format!("{}/v2/translate", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            200 => {
                let result = response
                    .json::<TranslateResponse>()
                    .await
                    .map_err(|e| AppError::NetworkError(e.to_string()))?;
                Ok(result)
            }
            403 => Err(AppError::InvalidApiKey),
            429 => Err(AppError::RateLimited),
            456 => Err(AppError::QuotaExceeded),
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(AppError::ApiError {
                    status,
                    message: body,
                })
            }
        }
    }

    async fn validate_key(&self) -> Result<UsageResponse, AppError> {
        let url = format!("{}/v2/usage", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            200 => {
                let usage = response
                    .json::<UsageResponse>()
                    .await
                    .map_err(|e| AppError::NetworkError(e.to_string()))?;
                Ok(usage)
            }
            403 => Err(AppError::InvalidApiKey),
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(AppError::ApiError {
                    status,
                    message: body,
                })
            }
        }
    }
}

/// Translates with exponential backoff retry on rate limiting (429)
pub async fn translate_with_retry(
    client: &dyn TranslationClient,
    text: &str,
    target_lang: &str,
    source_lang: Option<&str>,
) -> Result<TranslateResponse, AppError> {
    let mut last_error = AppError::RateLimited;

    for attempt in 0..=MAX_RETRIES {
        match client.translate(text, target_lang, source_lang).await {
            Ok(response) => return Ok(response),
            Err(AppError::RateLimited) => {
                last_error = AppError::RateLimited;
                if attempt < MAX_RETRIES {
                    let delay = Duration::from_secs(1 << attempt); // 1s, 2s, 4s, 8s
                    tokio::time::sleep(delay).await;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Err(last_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::collections::VecDeque;

    #[test]
    fn test_free_api_key_detection() {
        assert_eq!(
            DeepLClient::detect_base_url("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx:fx"),
            "https://api-free.deepl.com"
        );
    }

    #[test]
    fn test_pro_api_key_detection() {
        assert_eq!(
            DeepLClient::detect_base_url("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"),
            "https://api.deepl.com"
        );
    }

    // --- Mock client for trait-based testing ---

    struct MockTranslationClient {
        responses: Mutex<VecDeque<Result<TranslateResponse, AppError>>>,
    }

    impl MockTranslationClient {
        fn new(responses: Vec<Result<TranslateResponse, AppError>>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
            }
        }

        fn ok_response(text: &str, source: &str) -> Result<TranslateResponse, AppError> {
            Ok(TranslateResponse {
                translations: vec![super::super::types::Translation {
                    text: text.to_string(),
                    detected_source_language: source.to_string(),
                }],
            })
        }
    }

    #[async_trait]
    impl TranslationClient for MockTranslationClient {
        async fn translate(
            &self,
            text: &str,
            _target_lang: &str,
            _source_lang: Option<&str>,
        ) -> Result<TranslateResponse, AppError> {
            if text.is_empty() {
                return Err(AppError::EmptyText);
            }
            if text.chars().count() > 5000 {
                return Err(AppError::TextTooLong {
                    max: 5000,
                    actual: text.chars().count(),
                });
            }
            self.responses.lock().unwrap().pop_front().unwrap()
        }

        async fn validate_key(&self) -> Result<UsageResponse, AppError> {
            // Pop a response and convert - for validate_key tests we store
            // the UsageResponse as an Err variant won't work, so we handle
            // this separately
            let resp = self.responses.lock().unwrap().pop_front().unwrap();
            match resp {
                Ok(_) => Ok(UsageResponse {
                    character_count: 1234,
                    character_limit: 500000,
                }),
                Err(e) => Err(e),
            }
        }
    }

    #[tokio::test]
    async fn test_translate_success() {
        let client = MockTranslationClient::new(vec![
            MockTranslationClient::ok_response("안녕하세요", "EN"),
        ]);
        let result = client.translate("Hello", "KO", None).await.unwrap();
        assert_eq!(result.translations[0].text, "안녕하세요");
        assert_eq!(result.translations[0].detected_source_language, "EN");
    }

    #[tokio::test]
    async fn test_translate_empty_text() {
        let client = MockTranslationClient::new(vec![]);
        let result = client.translate("", "KO", None).await;
        assert!(matches!(result, Err(AppError::EmptyText)));
    }

    #[tokio::test]
    async fn test_translate_text_too_long() {
        let client = MockTranslationClient::new(vec![]);
        let long_text = "a".repeat(5001);
        let result = client.translate(&long_text, "KO", None).await;
        assert!(matches!(
            result,
            Err(AppError::TextTooLong {
                max: 5000,
                actual: 5001
            })
        ));
    }

    #[tokio::test]
    async fn test_translate_invalid_api_key() {
        let client = MockTranslationClient::new(vec![Err(AppError::InvalidApiKey)]);
        let result = client.translate("Hello", "KO", None).await;
        assert!(matches!(result, Err(AppError::InvalidApiKey)));
    }

    #[tokio::test]
    async fn test_translate_network_error() {
        let client = MockTranslationClient::new(vec![Err(AppError::NetworkError(
            "connection refused".to_string(),
        ))]);
        let result = client.translate("Hello", "KO", None).await;
        assert!(matches!(result, Err(AppError::NetworkError(_))));
    }

    #[tokio::test]
    async fn test_translate_quota_exceeded() {
        let client = MockTranslationClient::new(vec![Err(AppError::QuotaExceeded)]);
        let result = client.translate("Hello", "KO", None).await;
        assert!(matches!(result, Err(AppError::QuotaExceeded)));
    }

    #[tokio::test]
    async fn test_validate_key_success() {
        let client = MockTranslationClient::new(vec![
            MockTranslationClient::ok_response("", ""), // triggers Ok path in validate_key
        ]);
        let result = client.validate_key().await.unwrap();
        assert_eq!(result.character_count, 1234);
        assert_eq!(result.character_limit, 500000);
    }

    #[tokio::test]
    async fn test_validate_key_invalid() {
        let client = MockTranslationClient::new(vec![Err(AppError::InvalidApiKey)]);
        let result = client.validate_key().await;
        assert!(matches!(result, Err(AppError::InvalidApiKey)));
    }

    #[tokio::test]
    async fn test_translate_with_retry_immediate_success() {
        let client = MockTranslationClient::new(vec![
            MockTranslationClient::ok_response("안녕하세요", "EN"),
        ]);
        let result = translate_with_retry(&client, "Hello", "KO", None)
            .await
            .unwrap();
        assert_eq!(result.translations[0].text, "안녕하세요");
    }

    #[tokio::test]
    async fn test_translate_with_retry_non_retryable_error() {
        let client = MockTranslationClient::new(vec![Err(AppError::InvalidApiKey)]);
        let result = translate_with_retry(&client, "Hello", "KO", None).await;
        assert!(matches!(result, Err(AppError::InvalidApiKey)));
    }

    #[tokio::test]
    async fn test_translate_with_retry_empty_text_no_retry() {
        let client = MockTranslationClient::new(vec![]);
        let result = translate_with_retry(&client, "", "KO", None).await;
        assert!(matches!(result, Err(AppError::EmptyText)));
    }
}
