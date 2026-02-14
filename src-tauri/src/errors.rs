use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("API Key가 유효하지 않습니다")]
    InvalidApiKey,

    #[error("번역 할당량이 초과되었습니다")]
    QuotaExceeded,

    #[error("요청이 너무 많습니다. 잠시 후 다시 시도해주세요")]
    RateLimited,

    #[error("네트워크 연결을 확인해주세요: {0}")]
    NetworkError(String),

    #[error("DeepL API 오류: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("선택된 텍스트가 없습니다")]
    NothingSelected,

    #[error("클립보드를 읽을 수 없습니다")]
    ClipboardReadError,

    #[error("클립보드에 쓸 수 없습니다")]
    ClipboardWriteError,

    #[error("텍스트가 너무 깁니다 (최대 {max}자, 현재 {actual}자)")]
    TextTooLong { max: usize, actual: usize },

    #[error("번역할 텍스트가 비어 있습니다")]
    EmptyText,

    #[error("키 입력 시뮬레이션 실패: {0}")]
    KeySimulationError(String),

    #[error("API Key가 설정되지 않았습니다")]
    ApiKeyNotSet,

    #[error("Keychain 접근 오류: {0}")]
    KeychainError(String),

    #[error("Accessibility 권한이 필요합니다")]
    AccessibilityNotGranted,

    #[error("번역이 이미 진행 중입니다")]
    TranslationInProgress,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
