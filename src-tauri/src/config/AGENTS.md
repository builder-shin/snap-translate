<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# config

## Purpose
API 키 저장을 위한 `ApiKeyStore` trait과 OS 키체인 기반 실구현(`KeychainApiKeyStore`), 그리고 기본 설정값(`DEFAULT_TARGET_LANGUAGE="KO"`, `DEFAULT_HOTKEY="CmdOrCtrl+Shift+D"`) 상수를 제공한다. API 키는 반드시 이 모듈을 통해서만 접근해야 한다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `store` 모듈 재export |
| `store.rs` | `ApiKeyStore` trait, `KeychainApiKeyStore` (service="snap-translate", username="deepl-api-key"), 기본 상수, 테스트용 `MockApiKeyStore` |

## For AI Agents

### Working In This Directory
- API 키 저장 위치를 절대 파일/env/JSON으로 옮기지 말 것 — `keyring` 크레이트 (`apple-native` + `windows-native`)로만.
- `KeychainApiKeyStore::delete_api_key`는 `NoEntry` 에러를 성공으로 매핑 (멱등성 유지) — 이 동작을 깨지 말 것.
- `DEFAULT_TARGET_LANGUAGE`/`DEFAULT_HOTKEY`를 변경하면 `src/lib/constants.ts`의 프론트 상수, `deepl::types::TargetLanguage`, `commands::settings::StoredSettings::default`와의 일관성을 검토해야 한다.

### Testing Requirements
- `MockApiKeyStore`(HashMap 기반)로 set/get/delete/overwrite/없는 키 삭제/기본값 확인 테스트 유지.
- 실제 Keychain 테스트는 CI에서 불가능하므로 mock만 사용.

### Common Patterns
- `Result<Option<String>, AppError>` 반환으로 "키 없음"과 "에러"를 명확히 구분.
- trait 객체로만 사용(`Arc<dyn ApiKeyStore>`) — `lib.rs`에서 `.manage(Arc::new(KeychainApiKeyStore::new()))`로 등록.

## Dependencies

### Internal
- `crate::errors::AppError::KeychainError`

### External
- keyring 3 (apple-native, windows-native)
