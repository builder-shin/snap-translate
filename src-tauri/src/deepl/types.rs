use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TranslateRequest {
    pub text: Vec<String>,
    pub target_lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TranslateResponse {
    pub translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UsageResponse {
    pub character_count: u64,
    pub character_limit: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLanguage {
    KO,
    EN,
    JA,
    ZH,
    DE,
    FR,
    ES,
    PT,
    RU,
}

impl std::fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLanguage::KO => write!(f, "KO"),
            TargetLanguage::EN => write!(f, "EN"),
            TargetLanguage::JA => write!(f, "JA"),
            TargetLanguage::ZH => write!(f, "ZH"),
            TargetLanguage::DE => write!(f, "DE"),
            TargetLanguage::FR => write!(f, "FR"),
            TargetLanguage::ES => write!(f, "ES"),
            TargetLanguage::PT => write!(f, "PT-BR"),
            TargetLanguage::RU => write!(f, "RU"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_request_serialization() {
        let request = TranslateRequest {
            text: vec!["Hello".to_string()],
            target_lang: "KO".to_string(),
            source_lang: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""text":["Hello"]"#));
        assert!(json.contains(r#""target_lang":"KO"#));
        assert!(!json.contains("source_lang"));
    }

    #[test]
    fn test_translate_request_with_source_lang() {
        let request = TranslateRequest {
            text: vec!["Hello".to_string()],
            target_lang: "KO".to_string(),
            source_lang: Some("EN".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""source_lang":"EN"#));
    }

    #[test]
    fn test_translate_response_deserialization() {
        let json = r#"{"translations":[{"detected_source_language":"EN","text":"안녕하세요"}]}"#;
        let response: TranslateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.translations[0].text, "안녕하세요");
        assert_eq!(response.translations[0].detected_source_language, "EN");
    }

    #[test]
    fn test_usage_response_deserialization() {
        let json = r#"{"character_count":1234,"character_limit":500000}"#;
        let usage: UsageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(usage.character_count, 1234);
        assert_eq!(usage.character_limit, 500000);
    }

    #[test]
    fn test_target_language_display() {
        assert_eq!(TargetLanguage::KO.to_string(), "KO");
        assert_eq!(TargetLanguage::EN.to_string(), "EN");
        assert_eq!(TargetLanguage::PT.to_string(), "PT-BR");
    }
}
