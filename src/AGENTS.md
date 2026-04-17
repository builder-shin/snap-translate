<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# src

## Purpose
React 19 + TypeScript 프론트엔드 소스. 앱 UI는 단일 설정 창(`Settings`)으로 구성되며, DeepL API Key 입력, 번역 대상 언어 선택, 단축키 표시 기능을 제공한다. Tauri 런타임에서 실행되며 모든 외부 작업은 `invoke()`로 Rust 백엔드에 위임한다.

## Key Files
| File | Description |
|------|-------------|
| `main.tsx` | React DOM 엔트리 (StrictMode로 `App` 렌더링) |
| `App.tsx` | 루트 컴포넌트 (현재는 `Settings`만 렌더링) |
| `vite-env.d.ts` | Vite 전용 타입 선언 |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `components/` | 재사용 가능한 설정 UI 컴포넌트 (see `components/AGENTS.md`) |
| `pages/` | 페이지 수준 컴포넌트 (현재 Settings 1개) (see `pages/AGENTS.md`) |
| `lib/` | Tauri invoke 래퍼 및 상수 (see `lib/AGENTS.md`) |
| `types/` | 공유 TS 타입과 번역 언어 enum (see `types/AGENTS.md`) |
| `hooks/` | (현재 비어 있음) |

## For AI Agents

### Working In This Directory
- TypeScript `strict` + `noUnusedLocals/Parameters` 활성화 - 미사용 심볼 금지.
- Tauri API 호출은 가능한 한 `src/lib/tauri.ts`의 래퍼를 사용하되, 간단한 한 번 호출은 컴포넌트 내부에서 `invoke()` 직접 호출도 허용된다 (기존 코드가 혼용).
- 스타일링은 CSS Modules (`*.module.css`). Vitest 설정에서 `classNameStrategy: "non-scoped"`로 원본 이름이 노출된다.
- **로딩 UI 규칙**: 로딩 상태에는 텍스트("로딩 중...") 대신 스피너/스켈레톤을 사용해야 한다 (전역 user rule). 기존 `Settings.tsx`의 텍스트 로딩은 리팩터 대상.

### Testing Requirements
- 모든 컴포넌트 로직은 `tests/`의 대응 스펙으로 커버. Tauri `invoke`는 `tests/setup.ts`에서 `vi.mock()`으로 모킹된다.
- `pnpm test` 실행 후 신규/변경 컴포넌트의 테스트가 없으면 추가할 것.

### Common Patterns
- `AppSettings`/`TranslateResult`/`TargetLanguage` 타입은 `src/types/index.ts`에서 import.
- Rust의 snake_case 필드는 serde 기본 직렬화 (`has_api_key` 등). 현재 프론트 `AppSettings`는 camelCase를 기대하는데, Rust `Settings` 구조체는 snake_case로 직렬화된다는 점에 유의 (잠재적 불일치 — 새 필드 추가 시 확인 필요).
- Korean 문자열 유지. 사용자 문구 하드코딩은 컴포넌트 내부에 둔다 (i18n 미도입).

## Dependencies

### Internal
- `src-tauri/src/commands/` - `invoke()`로 호출되는 Rust 커맨드 정의

### External
- react, react-dom 19.2
- @tauri-apps/api 2.x (core.invoke)
