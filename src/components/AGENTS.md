<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# components

## Purpose
설정 페이지에서 사용되는 세 가지 UI 컴포넌트: DeepL API 키 입력, 번역 대상 언어 선택, 현재 단축키 표시. 각 컴포넌트는 자체 CSS Module과 1:1 매칭되며, 외부 통신은 `@tauri-apps/api/core`의 `invoke()`로 처리한다.

## Key Files
| File | Description |
|------|-------------|
| `ApiKeyInput.tsx` | DeepL API Key 입력/검증 (`save_api_key` invoke, password input) |
| `ApiKeyInput.module.css` | ApiKeyInput 스타일 |
| `LanguageSelect.tsx` | `TARGET_LANGUAGES` 드롭다운, 변경 시 `save_settings` invoke |
| `LanguageSelect.module.css` | LanguageSelect 스타일 |
| `HotkeyInput.tsx` | 현재 단축키를 `<kbd>` 조합으로 표시 (변경 불가 - 플랫폼 접두사 `CmdOrCtrl` → Cmd/Ctrl 치환) |
| `HotkeyInput.module.css` | HotkeyInput 스타일 |

## For AI Agents

### Working In This Directory
- 컴포넌트는 **default export** + props interface를 파일 상단에 선언하는 패턴을 사용한다.
- 사용자 메시지(에러, 상태)는 한국어 리터럴로 하드코딩. 테스트는 이 문자열을 직접 참조하므로 변경 시 테스트도 함께 수정.
- `invoke()` 실패 처리: `ApiKeyInput`은 `setError(String(err))`로 Rust `AppError` 한국어 메시지를 그대로 표시한다. 이 패턴 유지.

### Testing Requirements
- 각 컴포넌트는 `tests/components/<Name>.test.tsx`에 대응 스펙이 있어야 한다.
- `mockedInvoke.mockResolvedValue()` / `mockRejectedValue()`로 성공·실패 경로 모두 커버.
- `@testing-library/user-event`를 사용한 사용자 상호작용 중심 테스트.

### Common Patterns
- `hasApiKey: boolean` prop으로 설정 여부 표시만 하며, 실제 키는 노출하지 않음 (Keychain 보안 원칙).
- `LanguageSelect`는 낙관적 업데이트: invoke 성공 후 `onChange(newLang)`으로 부모 상태 반영.
- `HotkeyInput`은 `navigator.platform.includes("Mac")`으로 모디파이어 라벨 분기.

## Dependencies

### Internal
- `../types` - `TARGET_LANGUAGES`, `TargetLanguage`
- `../lib/tauri` (일부 컴포넌트는 직접 `invoke` 사용; 래퍼 사용이 권장됨)

### External
- react (hooks: useState)
- @tauri-apps/api/core (invoke)
