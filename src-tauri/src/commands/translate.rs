use crate::config::store::ApiKeyStore;
use crate::deepl::client::{DeepLClient, translate_with_retry};
use crate::errors::AppError;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize, Clone)]
pub struct TranslateResult {
    pub text: String,
    pub detected_source: String,
}

#[tauri::command]
pub async fn translate(
    text: String,
    target_lang: String,
    source_lang: Option<String>,
    api_key_store: tauri::State<'_, Arc<dyn ApiKeyStore>>,
) -> Result<TranslateResult, AppError> {
    let api_key = api_key_store
        .get_api_key()?
        .ok_or(AppError::ApiKeyNotSet)?;

    let client = DeepLClient::new(api_key);
    let response = translate_with_retry(
        &client,
        &text,
        &target_lang,
        source_lang.as_deref(),
    )
    .await?;

    let translation = response
        .translations
        .into_iter()
        .next()
        .ok_or(AppError::EmptyText)?;

    Ok(TranslateResult {
        text: translation.text,
        detected_source: translation.detected_source_language,
    })
}
