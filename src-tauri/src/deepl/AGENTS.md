<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# deepl

## Purpose
DeepL API HTTP 클라이언트. `TranslationClient` trait + `DeepLClient` 실구현으로 번역(`/v2/translate`)과 API 키 검증(`/v2/usage`)을 제공한다. Rate limit(429)에 대해 지수 백오프 재시도(`translate_with_retry`)를 수행하며, API 키 접미사 `:fx` 존재 여부로 free/pro 엔드포인트를 자동 선택한다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `types`, `client` 재export |
| `client.rs` | `TranslationClient` trait, `DeepLClient`(reqwest), `translate_with_retry`(최대 4회 재시도, 1/2/4/8초 백오프), `detect_base_url` |
| `types.rs` | `TranslateRequest`, `TranslateResponse`, `Translation`, `UsageResponse`, `TargetLanguage` enum + `Display` impl |

## For AI Agents

### Working In This Directory
- HTTP 상태 코드 매핑 (`client.rs`의 `match response.status().as_u16()`):
  - `200` → OK
  - `403` → `AppError::InvalidApiKey`
  - `429` → `AppError::RateLimited` (재시도 대상)
  - `456` → `AppError::QuotaExceeded`
  - 기타 → `AppError::ApiError { status, message }`
  - 이 매핑을 변경하면 `lib.rs::notify_flow_result`의 사용자 메시지 분기도 확인.
- `MAX_TEXT_LENGTH = 5000`은 DeepL 문서/요금제 제한에 맞춰진 값. `translate_flow.rs` 및 `src/lib/constants.ts`와 반드시 동기화.
- `translate_with_retry`는 `RateLimited`만 재시도, 다른 에러는 즉시 반환. 재시도 로직을 확장하면 기존 테스트(`test_translate_with_retry_non_retryable_error`)도 갱신.
- `TargetLanguage::PT`의 `Display`는 `"PT-BR"`을 반환 — DeepL이 PT를 PT-BR/PT-PT로 구분하기 때문. 프론트 코드 `"PT"`와 매핑 차이에 주의.

### Testing Requirements
- `MockTranslationClient`(`VecDeque<Result<..., AppError>>` 기반)로 성공/빈 텍스트/길이 초과/invalid key/network error/quota/retry 경로를 모두 검증.
- serde 직렬화/역직렬화 round-trip 테스트 유지 (`TranslateRequest`의 `source_lang: None` skip, `UsageResponse` 디코딩).
- 실 네트워크 호출 테스트는 두지 않는다.

### Common Patterns
- `async_trait`로 비동기 trait 객체 지원.
- `Option<&str>` 파라미터는 serde `skip_serializing_if`로 JSON에서 생략.
- `reqwest::Client`는 `DeepLClient::new` 시마다 새로 생성 (현재 코드). 빈번한 호출이 있으면 재사용 최적화 여지 있음.

## Dependencies

### Internal
- `crate::errors::AppError`

### External
- reqwest (json), serde (Serialize/Deserialize), async-trait, tokio (sleep)
