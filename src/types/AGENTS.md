<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# types

## Purpose
프론트엔드 전역 TypeScript 타입 정의. Rust 백엔드 구조체 (`commands::settings::Settings`, `commands::translate::TranslateResult`)의 TS 미러와, DeepL이 지원하는 번역 대상 언어 enum 및 한국어 표시명 매핑을 담당한다.

## Key Files
| File | Description |
|------|-------------|
| `index.ts` | `AppSettings`, `TranslateResult`, `TargetLanguage` union, `TARGET_LANGUAGES` 라벨 맵 |

## For AI Agents

### Working In This Directory
- Rust 구조체가 변경되면 이 파일도 함께 수정해야 한다 (자동 생성 없음).
- `AppSettings`는 `hasApiKey`/`targetLanguage`/`hotkey` (camelCase)로 정의되어 있으나, Rust `Settings`는 serde 기본으로 snake_case 직렬화 → 프론트 타입과 불일치 가능성이 있음. 동작 검증 시 확인 필요.
- `TARGET_LANGUAGES` 맵에 항목 추가 시 Rust `deepl::types::TargetLanguage` enum과 `Display` impl에도 동일하게 반영해야 한다.

### Testing Requirements
- 타입 전용 파일이므로 런타임 테스트는 없음. `tsc --noEmit`(= `pnpm build`)로 검증.

### Common Patterns
- `TargetLanguage` union을 `Record<TargetLanguage, string>`의 키로 사용하여 누락된 언어를 컴파일 타임에 감지.
- 언어 코드 값: `KO, EN, JA, ZH, DE, FR, ES, PT, RU` — 단, Rust `Display`는 PT를 `"PT-BR"`로 반환하는 차이가 있음.

## Dependencies

### Internal
- 다른 프론트엔드 모듈이 이 파일에서 타입을 import.

### External
- 없음 (타입만 정의)
