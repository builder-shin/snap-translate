<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# lib (tests)

## Purpose
`src/lib/tauri.ts` 래퍼 함수들이 올바른 커맨드 이름과 인자 형태로 `invoke()`를 호출하는지 검증한다. Tauri 계약(command 이름, 파라미터 키)의 회귀를 방지하는 얇은 단위 테스트 레이어.

## Key Files
| File | Description |
|------|-------------|
| `tauri.test.ts` | `getSettings`, `saveApiKey`, `translate` 래퍼의 invoke 인자/응답 검증 |

## For AI Agents

### Working In This Directory
- 각 래퍼 함수에 대해 "invoke가 정확히 이 문자열과 객체로 호출된다"를 assertion (`toHaveBeenCalledWith("command_name", { ... })`).
- 래퍼 응답 형태(`AppSettings`, `TranslateResult`)도 `toEqual(...)`로 타입 외 필드명 검증까지 포함.
- 새 래퍼가 추가되면 여기에도 한 케이스 추가하여 계약을 고정.

### Testing Requirements
- `beforeEach`에서 `vi.clearAllMocks()` 호출 유지.
- 에러 경로는 래퍼가 단순 패스스루이므로 컴포넌트 테스트 쪽에서 커버 (래퍼 자체 에러 변환 로직이 없음).

### Common Patterns
- `vi.mocked(invoke)`로 타입-안전한 mock 참조.
- `await` 결과를 `result`에 받아 assertion, 그리고 `mockedInvoke`의 호출 인자를 별도로 검증 (결과와 호출 둘 다 확인).

## Dependencies

### Internal
- `../../src/lib/tauri`

### External
- vitest, @tauri-apps/api/core (mocked via setup.ts)
