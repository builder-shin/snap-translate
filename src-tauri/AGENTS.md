<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# src-tauri

## Purpose
Tauri 2.x Rust 백엔드 크레이트. 시스템 트레이, 글로벌 단축키, 클립보드 조작, Enigo 기반 키 입력 시뮬레이션, DeepL HTTP 클라이언트, OS 키체인 API 키 저장, macOS accessibility 권한 확인, 일별 롤링 로그를 담당한다. 프론트엔드는 `invoke_handler!`로 등록된 Tauri 커맨드를 통해 이 크레이트와 통신한다.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Rust 의존성 (tauri 2, reqwest, enigo, keyring, tracing, tokio 등) 및 타겟 플랫폼 분기 |
| `Cargo.lock` | Cargo lock file |
| `build.rs` | `tauri_build::build()` 호출 |
| `tauri.conf.json` | 앱 번들/윈도우/트레이 설정, CSP, 번역 앱 identifier |
| `capabilities/default.json` | Tauri permission manifest (트레이/알림/클립보드/단축키) |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | Rust 소스 트리 (lib.rs, 모듈들) (see `src/AGENTS.md`) |
| `capabilities/` | Tauri 권한 정책 JSON (see `capabilities/AGENTS.md`) |
| `icons/` | 앱/트레이 아이콘 바이너리 (32/128/128@2x PNG, ICO, ICNS, tray-icon) |
| `gen/` | Tauri 빌드 시 생성되는 schema (직접 편집 금지) |

## For AI Agents

### Working In This Directory
- 빌드/테스트는 `cd src-tauri && cargo build` / `cargo test`. 전체 앱 dev는 루트에서 `pnpm tauri dev`.
- 새 커맨드 추가 3단계: (1) `src/commands/<feature>.rs`에 `#[tauri::command]` 함수 작성 → (2) `src/commands/mod.rs`에 `pub mod` 선언 → (3) `src/lib.rs`의 `tauri::generate_handler!` 배열에 추가. 한 단계라도 빠지면 런타임 `Command not found` 에러.
- 플랫폼 분기 빌드: `Cargo.toml`의 `[target.'cfg(target_os = "macos")'.dependencies]`에 `macos-accessibility-client`가 정의되어 있음. macOS 전용 코드는 `#[cfg(target_os = "macos")]` 가드 필수.
- 시크릿은 **반드시** `keyring` 크레이트로만 저장 (`config::store::KeychainApiKeyStore`). 파일/환경변수/설정 JSON에 저장 금지.

### Testing Requirements
- 모든 Rust 모듈은 `#[cfg(test)] mod tests { ... }`와 mock 구현을 포함해야 한다 (`translate_flow.rs`의 7개 시나리오 테스트가 표준 참조).
- trait 기반 DI로 실제 네트워크/OS 호출 없이 단위 테스트 가능. Mock 추가 시 기존 mock 구조체 스타일(`Mutex<Option<T>>`, 시퀀스 응답)을 따를 것.
- 통합 테스트 디렉토리 `tests/`는 현재 없음 — 모든 테스트는 인라인.

### Common Patterns
- **Trait 추상화**: `TranslationClient`, `ClipboardAccess`, `KeySimulator`, `ApiKeyStore`, `AccessibilityChecker`. 외부 I/O는 모두 trait 뒤에 숨겨 테스트를 가능하게 한다.
- **Error 모델**: 모든 공개 함수는 `Result<T, AppError>` 반환. `AppError`는 `thiserror::Error` + `Serialize` 구현으로 프론트에도 그대로 문자열화되어 전달된다.
- **상태 관리**: `Arc<dyn ApiKeyStore>`와 `AppState`(Arc<AtomicBool>)를 `.manage()`로 Tauri에 등록 후 `State<'_, ...>`로 주입.
- **Retry/Backoff**: `deepl::client::translate_with_retry`는 429(RateLimited)에만 1s/2s/4s/8s 지수 백오프 재시도.

## Dependencies

### External (Rust)
- tauri 2 + plugins (global-shortcut, clipboard-manager, notification, store)
- reqwest (JSON), serde/serde_json, async-trait, tokio (time)
- enigo 0.3 (키 시뮬레이션), keyring 3 (OS 키체인)
- tracing, tracing-subscriber, tracing-appender (로깅)
- thiserror 2 (에러)
- macos-accessibility-client (macOS only)
