<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# tests

## Purpose
프론트엔드 Vitest + React Testing Library + jsdom 테스트 스위트. `@tauri-apps/api/core`의 `invoke`, `plugin-store`, `plugin-notification`은 `setup.ts`에서 전역 mock으로 교체되므로 실제 Tauri 런타임 없이 컴포넌트/라이브러리 단위 테스트가 가능하다. Rust 백엔드 테스트는 `src-tauri/` 내 인라인 `#[cfg(test)] mod tests`로 별도 관리된다.

## Key Files
| File | Description |
|------|-------------|
| `setup.ts` | `@testing-library/jest-dom` import + Tauri 모듈 전역 vi.mock (invoke, Store, notification 권한) |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `components/` | 각 UI 컴포넌트 스펙 (ApiKeyInput, LanguageSelect, HotkeyInput) (see `components/AGENTS.md`) |
| `lib/` | `src/lib/tauri.ts` 래퍼 테스트 (see `lib/AGENTS.md`) |
| `hooks/` | (현재 비어 있음) |

## For AI Agents

### Working In This Directory
- 실행: `pnpm test` (한 번) 또는 `pnpm test:watch`.
- 모든 테스트 파일은 `*.test.ts(x)` 네이밍. `vitest.config.ts`는 `setupFiles: ["./tests/setup.ts"]`로 자동 로드.
- Tauri API를 사용하는 코드 테스트 시 **항상** `vi.mocked(invoke)`를 통해 응답을 설정한다 (`mockResolvedValue` / `mockRejectedValue`). `beforeEach(() => vi.clearAllMocks())` 패턴 유지.
- CSS Modules는 `vitest.config.ts`의 `classNameStrategy: "non-scoped"`로 원본 클래스명이 보존된다 — `toHaveClass("container")` 같은 검증 가능.

### Testing Requirements
- 새 컴포넌트/래퍼 추가 시 최소 `렌더링 + 성공 경로 + 에러 경로` 3개 테스트 권장.
- 테스트에서 한국어 문자열을 직접 참조(`getByText("저장")`)하므로 UI 메시지 변경 시 테스트 동기화 필수.

### Common Patterns
- `userEvent`(v14): `await userEvent.type(input, ...)`, `await userEvent.click(button)` — 모두 `await` 필수.
- 비동기 UI 변화는 `waitFor(() => expect(...))` 로 감싼다.
- `screen.getByPlaceholderText(...)` / `getByRole("combobox")` 등 의미 기반 선택자를 우선.

## Dependencies

### Internal
- `../src/**` 를 상대경로로 import (`../../src/components/...`).

### External
- vitest 4, jsdom 28
- @testing-library/react 16, @testing-library/user-event 14, @testing-library/jest-dom 6
