<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# lib

## Purpose
공유 프론트엔드 유틸리티: Tauri `invoke()` 호출을 래핑한 타입-안전한 함수들과 전역 상수. Rust 백엔드 커맨드에 해당하는 1:1 래퍼를 제공하여 컴포넌트가 직접 `invoke(<string>)`을 부르는 대신 타입 체크된 헬퍼를 사용할 수 있게 한다.

## Key Files
| File | Description |
|------|-------------|
| `tauri.ts` | `getSettings`, `saveApiKey`, `saveSettings`, `translate` 래퍼 (snake_case 커맨드 → camelCase API) |
| `constants.ts` | `DEFAULT_TARGET_LANGUAGE="KO"`, `DEFAULT_HOTKEY`, `MAX_TEXT_LENGTH=5000`, `APP_NAME` |

## For AI Agents

### Working In This Directory
- 새 Rust 커맨드 추가 시 `tauri.ts`에 대응 래퍼 함수를 추가할 것 (함수명은 camelCase, invoke 문자열은 snake_case).
- 상수 값은 Rust `config::store`의 `DEFAULT_TARGET_LANGUAGE`/`DEFAULT_HOTKEY` 및 `translate_flow`의 `MAX_TEXT_LENGTH`와 일치해야 한다 — 한쪽만 바꾸면 동작이 어긋남.
- Tauri 매개변수 직렬화 규칙: JS 객체의 camelCase 키는 Tauri가 자동으로 Rust snake_case로 매핑 (`apiKey` → `api_key`). 하지만 response 쪽은 Rust의 serde 기본값(snake_case)이 그대로 나오므로 타입 정의에서 확인 필요.

### Testing Requirements
- `tests/lib/tauri.test.ts`가 각 래퍼의 invoke 인자/결과를 검증.
- 새 래퍼 추가 시 동일 패턴으로 테스트 추가.

### Common Patterns
- 모든 함수는 `async` + `Promise<T>` 반환, 내부적으로 `invoke<T>(...)` 사용.
- `Partial<AppSettings>`로 부분 업데이트 지원.

## Dependencies

### Internal
- `../types` - `AppSettings`, `TranslateResult`

### External
- @tauri-apps/api/core
