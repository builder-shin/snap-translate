<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# capabilities

## Purpose
Tauri 2 권한(capability) 매니페스트 디렉토리. Tauri는 이곳의 JSON을 빌드 시 읽어 앱이 사용할 수 있는 플러그인 permission을 화이트리스트한다. 새 플러그인 기능 사용 시 여기 명시하지 않으면 런타임에 permission 에러로 거부된다.

## Key Files
| File | Description |
|------|-------------|
| `default.json` | 기본 capability (tray, notification, clipboard, global-shortcut 등 앱 전역 권한) |

## For AI Agents

### Working In This Directory
- 새 Tauri 플러그인 API를 호출하기 전에 해당 permission 식별자를 `default.json`의 `permissions` 배열에 추가해야 한다. 누락 시 프론트 `invoke` 또는 JS API 호출이 `not allowed by the capability` 에러로 거부된다.
- capability 범위(windows/webviews)는 본 앱의 구성상 기본 창 및 `settings` 창 모두 포함되어야 한다.
- Tauri 공식 권한 식별자 형식: `<plugin>:<action>` (예: `clipboard-manager:allow-read-text`). 커스텀 커맨드에는 capability가 불필요.

### Testing Requirements
- `pnpm tauri dev` 실행 시 플러그인 기능이 정상 동작하는지 수동 확인. permission 오류는 콘솔에 명시적으로 표시된다.

### Common Patterns
- 앱은 단일 capability 파일만 사용. 환경별 분리가 필요해지면 `development.json`/`production.json`으로 추가 가능 (현재 미사용).

## Dependencies

### External
- tauri build (capability schema는 `gen/schemas`에서 생성됨)
