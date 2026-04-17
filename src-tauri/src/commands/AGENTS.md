<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# commands

## Purpose
`#[tauri::command]`로 프론트엔드에 노출되는 엔드포인트 집합. 번역 요청과 설정(CRUD) 기능을 제공한다. 모든 커맨드는 `Result<T, AppError>`를 반환하며 `AppError`는 한국어 메시지로 직렬화된다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `translate`, `settings` 모듈 재export |
| `translate.rs` | `translate(text, target_lang, source_lang?)` - Keychain에서 키 조회 → `DeepLClient` → `translate_with_retry` |
| `settings.rs` | `get_settings`, `save_api_key`(검증 포함), `save_settings`(partial), 파일 기반 `StoredSettings` 영속 |

## For AI Agents

### Working In This Directory
- 새 커맨드 추가 시: (1) 이 디렉토리에 파일 생성 → (2) `mod.rs`에 `pub mod` → (3) **반드시** `src-tauri/src/lib.rs`의 `tauri::generate_handler!` 매크로 목록에 `commands::<file>::<fn>` 추가. 누락 시 프론트에서 `Command not found` 런타임 에러.
- 커맨드 매개변수는 camelCase로 받을 수 있도록 Tauri가 자동 매핑하지만, Rust 측 함수 파라미터는 snake_case로 선언 (`api_key: String` ← `{apiKey: "..."}`).
- `tauri::State<'_, T>` 주입은 `.manage()`로 등록된 타입만 가능. 새 전역 상태 추가 시 `lib.rs`의 `.manage()` 호출도 같이 수정.

### Testing Requirements
- `save_api_key`는 빈 문자열/공백 거부 테스트 유지.
- `save_settings`는 `PartialSettings`로 부분 업데이트가 허용되어야 함.
- 실제 Keychain/DeepL 호출이 있는 함수는 mock ApiKeyStore나 `DeepLClient` 인스턴스 없이 검증 가능한 pure 로직(문자열 검사, serde 직렬화)만 단위 테스트.

### Common Patterns
- `api_key_store: tauri::State<'_, Arc<dyn ApiKeyStore>>` 주입.
- `save_api_key`는 저장 전에 `DeepLClient::validate_key()`로 DeepL API `/v2/usage` 왕복 검증 — 네트워크 의존성이 있다는 점에 주의.
- `StoredSettings`는 `get_config_directory()` 하위의 `settings.json`에 `serde_json::to_string_pretty`로 저장; 읽기 실패 시 `StoredSettings::default()` 폴백 (KO + CmdOrCtrl+Shift+D).
- `Settings` 응답 구조체는 serde 기본(snake_case)으로 직렬화 — 프론트 `AppSettings` camelCase와의 매핑 확인 필요.

## Dependencies

### Internal
- `crate::config::store` - `ApiKeyStore`, `DEFAULT_TARGET_LANGUAGE`, `DEFAULT_HOTKEY`
- `crate::deepl::client` - `DeepLClient`, `translate_with_retry`, `TranslationClient`
- `crate::errors::AppError`

### External
- tauri (command macro, State), serde, serde_json
