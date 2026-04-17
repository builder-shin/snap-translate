<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# pages

## Purpose
페이지 수준 컴포넌트 컨테이너. 현재는 단일 설정 창 페이지 `Settings`만 존재하며, 초기 진입 시 `get_settings`를 호출하여 상태를 로드하고 하위 컴포넌트 3종(`ApiKeyInput`, `LanguageSelect`, `HotkeyInput`)을 조합한다.

## Key Files
| File | Description |
|------|-------------|
| `Settings.tsx` | 설정 페이지 루트 - `AppSettings` 로드/주입, 버전 표시 |
| `Settings.module.css` | 설정 페이지 레이아웃 스타일 |

## For AI Agents

### Working In This Directory
- 페이지 추가 시 라우팅 계층이 없으므로 `App.tsx`를 직접 수정해야 한다. 라우터 도입은 현재 범위 밖.
- `loading` 상태 UI는 현재 "로딩 중..." 텍스트 - 전역 user rule에 따라 스피너/스켈레톤으로 교체해야 한다 (`Loader2` 등).
- 에러 시 "설정을 불러올 수 없습니다." 폴백도 더 나은 UX로 개선 가능한 지점.

### Testing Requirements
- 현재 페이지 전용 테스트는 없음. 하위 컴포넌트는 개별 테스트로 커버됨.
- 페이지 수준 통합 테스트 추가 시 `invoke("get_settings")`를 모킹한 후 폼 전체 플로를 검증.

### Common Patterns
- `useState<AppSettings | null>(null)` 초기값 null + `loading` 플래그 조합.
- `loadSettings`를 자식 `onSaved` 콜백으로 재사용하여 API Key 저장 후 상태를 재조회.
- 언어 선택은 낙관적 업데이트: `setSettings((s) => s ? { ...s, targetLanguage: lang } : s)`.

## Dependencies

### Internal
- `../components/ApiKeyInput`, `LanguageSelect`, `HotkeyInput`
- `../types` - `AppSettings`

### External
- react (useState, useEffect)
- @tauri-apps/api/core (invoke)
