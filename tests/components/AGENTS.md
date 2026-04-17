<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# components (tests)

## Purpose
`src/components/`의 각 UI 컴포넌트에 대응하는 단위 테스트. `invoke` mock을 통해 Tauri 백엔드 호출을 시뮬레이션하고, React Testing Library로 렌더링/상호작용/상태 전이를 검증한다.

## Key Files
| File | Description |
|------|-------------|
| `ApiKeyInput.test.tsx` | 설정 표시, 저장 성공/실패, 검증 에러, password 마스킹, 빈 입력 버튼 비활성 |
| `LanguageSelect.test.tsx` | 9개 언어 렌더링, 현재 선택 반영, 변경 시 `save_settings` invoke + `onChange` 호출 |
| `HotkeyInput.test.tsx` | 키 조합 `<kbd>` 렌더링, "향후 버전" 안내 문구 표시 |

## For AI Agents

### Working In This Directory
- 테스트는 사용자 관점(role/text/placeholder) 기반 선택자를 사용한다. 구현 세부(class 이름, DOM 구조)에 결합하지 않도록 주의.
- `mockedInvoke.mockResolvedValue(undefined)` 패턴으로 success 경로를, `mockRejectedValue("...에러 메시지")`로 에러를 시뮬레이션한다. 에러 문자열은 `String(err)`로 UI에 표시되므로 테스트 기대값과 정확히 일치해야 한다.
- `userEvent.type`이 각 키 입력을 개별 이벤트로 발생시키므로 placeholder/disabled 전환 확인 시점에 유의.

### Testing Requirements
- 새 컴포넌트 추가 시 최소: 초기 렌더링, 주요 상호작용 1개, 에러 경로 1개 커버.
- `invoke` 인자 검증(`toHaveBeenCalledWith`)은 Tauri 계약 회귀 방지를 위해 반드시 포함.

### Common Patterns
- `beforeEach(() => vi.clearAllMocks())`로 테스트 간 mock 격리.
- `waitFor`로 비동기 상태 업데이트 대기.
- 한국어 사용자 문구를 직접 참조 — UI 변경 시 테스트도 함께 업데이트.

## Dependencies

### Internal
- `../../src/components/*`

### External
- vitest, @testing-library/react, @testing-library/user-event, @tauri-apps/api/core (mocked)
