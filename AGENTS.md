<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# snap-translate

## Purpose
macOS/Windows 데스크톱 번역 도구. Tauri 2.x + React 19 + Rust 기반으로 제작된 시스템 트레이 상주형 앱으로, 글로벌 단축키(`Cmd/Ctrl+Shift+D`)를 사용하여 어느 앱에서든 선택된 텍스트를 DeepL API로 즉시 번역한다. 번역 결과는 클립보드에 자동 복사되며 데스크톱 알림으로 표시된다.

## Key Files
| File | Description |
|------|-------------|
| `package.json` | Frontend dependencies and scripts (pnpm, vite, vitest) |
| `tsconfig.json` | TypeScript strict 설정 (ES2020, react-jsx) |
| `tsconfig.node.json` | Vite/Node 빌드용 TS 설정 |
| `vite.config.ts` | Vite 개발 서버 설정 (포트 1420, src-tauri 제외) |
| `vitest.config.ts` | Vitest jsdom + React 테스트 설정 |
| `index.html` | React 앱 엔트리 HTML |
| `pnpm-lock.yaml` | pnpm lock file (package manager: pnpm) |
| `README.md` | 한국어 사용자 가이드 |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | React 프론트엔드 소스 (설정 UI) (see `src/AGENTS.md`) |
| `src-tauri/` | Rust 백엔드 (Tauri 앱, DeepL, 클립보드, 핫키) (see `src-tauri/AGENTS.md`) |
| `tests/` | 프론트엔드 Vitest 테스트 스위트 (see `tests/AGENTS.md`) |
| `.omc/` | oh-my-claudecode 상태/세션/플랜 캐시 (툴링 전용, 직접 편집 금지) |

## For AI Agents

### Working In This Directory
- 패키지 매니저는 **pnpm**. `npm`/`yarn`은 사용하지 말 것.
- 프론트엔드 의존성 변경 후 `pnpm install` 필수. Rust 의존성 변경 후 `cargo build` 필요.
- Tauri 커맨드는 Rust에서 `#[tauri::command]`로 정의하고, 프론트에서 `invoke("<snake_case>")`로 호출. `lib.rs::run()`의 `invoke_handler!` 매크로에 반드시 등록.
- 텍스트/메시지는 한국어로 작성한다 (사용자 UI 문자열, 에러 메시지 포함).

### Testing Requirements
- 프론트: `pnpm test` (Vitest, 한 번 실행) 또는 `pnpm test:watch`.
- 백엔드: `cd src-tauri && cargo test`. `translate_flow`의 mock 기반 E2E 흐름 테스트를 반드시 유지.
- 빌드 확인: `pnpm build` (TS 타입체크 + Vite 번들링).
- 전체 앱 실행: `pnpm tauri dev`.

### Common Patterns
- **Trait-based DI**: Rust 측은 `TranslationClient`, `ClipboardAccess`, `KeySimulator`, `ApiKeyStore`, `AccessibilityChecker` 트레이트로 추상화하여 mock 테스트를 지원한다. 새 외부 의존성 추가 시 동일 패턴을 따를 것.
- **TranslationGuard**: `AtomicBool` 기반 RAII 가드로 동시 번역을 차단. `translate_flow.rs`의 `try_acquire` 패턴을 준수.
- **플랫폼 분기**: `#[cfg(target_os = "macos")]` / `windows` / 기타로 분기. 로그/설정 경로, 키 시뮬레이션, accessibility 체크가 해당된다.
- **DeepL base URL 자동 감지**: `:fx` suffix면 free, 아니면 pro. `DeepLClient::detect_base_url` 참조.

## Dependencies

### External (Frontend)
- React 19.2 - UI 프레임워크
- @tauri-apps/api 2.x - Rust ↔ TS bridge
- @tauri-apps/plugin-notification, plugin-store - 시스템 통합
- TypeScript 5.9 (strict), Vite 7.3, Vitest 4, jsdom 28

### External (Backend, Rust)
- tauri 2 (`tray-icon`, `image-png`) - 데스크톱 셸
- tauri-plugin-global-shortcut / clipboard-manager / notification / store
- reqwest - DeepL HTTP 클라이언트
- enigo - 키 입력 시뮬레이션 (Cmd/Ctrl+C)
- keyring - OS 키체인 API 키 저장
- macos-accessibility-client (macOS 전용)
- tracing + tracing-appender - 일별 롤링 파일 로그
- tokio, async-trait, thiserror, serde

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
