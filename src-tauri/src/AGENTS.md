<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# src (Rust)

## Purpose
Rust 라이브러리 크레이트 루트. 모든 도메인 모듈이 `lib.rs`에서 선언되며, `lib.rs::run()`이 Tauri 앱을 구성(상태·플러그인·트레이·글로벌 단축키)하고 실행한다. `main.rs`는 `lib.rs::run()`을 호출하는 얇은 바이너리 엔트리.

## Key Files
| File | Description |
|------|-------------|
| `main.rs` | 바이너리 엔트리. `snap_translate_lib::run()` 호출. Windows release 빌드 시 `windows_subsystem = "windows"` |
| `lib.rs` | 앱 조립: 플러그인 등록, 트레이 메뉴, 전역 단축키 핸들러, `AppState`, `TauriClipboard` adapter, `handle_hotkey`, `notify_flow_result`, `invoke_handler!` |
| `errors.rs` | `AppError` enum + `Serialize` (thiserror 기반, 한국어 메시지) |
| `translate_flow.rs` | 번역 전체 흐름 (guard → accessibility → 키체인 → 클립보드 백업/복원 → Cmd+C → 재시도 읽기 → 길이 검사 → DeepL → 결과 기록), `TranslationGuard` RAII |
| `logging.rs` | `init_logging`(일별 롤링 파일, EnvFilter), `get_log_directory` (플랫폼별 경로) |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `commands/` | `#[tauri::command]` 엔드포인트들 (translate, settings) (see `commands/AGENTS.md`) |
| `config/` | API 키 저장소 trait + Keychain 구현, 기본 상수 (see `config/AGENTS.md`) |
| `deepl/` | DeepL HTTP 클라이언트, 요청/응답 타입, 재시도 (see `deepl/AGENTS.md`) |
| `clipboard/` | 클립보드 trait, 백업/복원/재시도 읽기 (see `clipboard/AGENTS.md`) |
| `key_simulator/` | Enigo 기반 Cmd/Ctrl+C 시뮬레이션 (see `key_simulator/AGENTS.md`) |
| `hotkey/` | 플랫폼별 기본 단축키 생성 (see `hotkey/AGENTS.md`) |
| `accessibility/` | macOS accessibility 권한 확인/프롬프트 (see `accessibility/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- `lib.rs::run()` 수정 시 순서가 중요하다: logging → state/plugins → `.invoke_handler(...)` → `.setup(|app| { tray, global shortcut, accessibility })`.
- `handle_hotkey`는 `tauri::async_runtime::spawn`으로 비동기 실행하며 `TauriClipboard` adapter를 생성해 `execute_translate_flow`에 주입한다. 새 외부 의존성 추가 시 이 패턴으로 주입.
- `notify_flow_result`는 `AppError` 배리언트별로 한국어 사용자 메시지를 분기한다 — 새 에러 추가 시 여기도 케이스 추가 필요.
- 로그 경로는 `get_log_directory`가 결정: macOS `~/Library/Logs/SnapTranslate`, Windows `%APPDATA%\SnapTranslate\logs`. 트레이 메뉴 "로그 열기"가 이 디렉토리를 OS file manager로 연다.

### Testing Requirements
- 각 모듈의 `tests`에 mock 구현과 단위 테스트를 유지.
- `translate_flow`는 전체 흐름을 mock으로 end-to-end 검증 (성공·동시실행·텍스트 길이·API 키 없음·네트워크 에러·선택 없음·accessibility).
- `lib.rs`의 `AppState` 초기값, 로그 디렉토리 비어있지 않음 기본 테스트 유지.

### Common Patterns
- **모듈 구조**: `<feature>/mod.rs`에 `pub mod <impl>;`만 있고 실제 로직은 하위 파일 (예: `deepl/mod.rs` + `deepl/client.rs`, `deepl/types.rs`).
- **Trait 주입**: `execute_translate_flow`는 trait 참조만 받음 → 실제 타입과 mock이 동일 시그니처.
- **RAII Guard**: `TranslationGuard::try_acquire` → Drop 시 `AtomicBool::store(false)`. 패닉에도 플래그가 해제됨 (테스트로 검증).

## Dependencies

### Internal
- `lib.rs`가 모든 하위 모듈을 선언/사용하는 허브.
- `commands/`는 `config::store::ApiKeyStore`, `deepl::client`를 소비.
- `translate_flow`는 `clipboard`, `key_simulator`, `deepl`, `config`, `accessibility`, `errors`를 조합.

### External
- tauri (WebviewWindow, Manager, tray, menu), tauri-plugin-*
- tracing, thiserror, serde, async-trait, tokio
