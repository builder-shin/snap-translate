<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# hotkey

## Purpose
글로벌 단축키 관리 헬퍼. `create_default_shortcut()`이 플랫폼별 기본 조합(macOS `Cmd+Shift+D`, Windows/Linux `Ctrl+Shift+D`)을 반환한다. 실제 단축키 등록/핸들러 바인딩은 `lib.rs::setup_global_shortcut`에서 `tauri-plugin-global-shortcut`로 수행된다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `manager` 재export |
| `manager.rs` | `create_default_shortcut()` - `Modifiers::SUPER\|SHIFT`(mac) / `CONTROL\|SHIFT`(win·linux) + `Code::KeyD` |

## For AI Agents

### Working In This Directory
- 단축키 값이 변경되면 README, `src/lib/constants.ts::DEFAULT_HOTKEY`, `config::store::DEFAULT_HOTKEY`, `lib.rs`의 `tracing::info!` 로그 문자열, `HotkeyInput` 컴포넌트 표시까지 모두 동기화 필요.
- `CmdOrCtrl` 문자열은 Tauri frontend convention이고, 실제 OS 네이티브 레벨은 `Modifiers::SUPER`(mac) / `CONTROL`로 분리된다. 새 단축키 형식을 파싱하는 로직은 현재 없음.
- 사용자 커스터마이징은 현재 미구현 (`HotkeyInput`이 "향후 버전" 문구 표시). 구현 시 기존 단축키 unregister → 새 shortcut register → 실패 롤백 패턴이 필요하다.

### Testing Requirements
- 현재는 `create_default_shortcut()`이 패닉 없이 생성되는지만 검증. 실제 OS 등록은 통합 환경에서만 가능.

### Common Patterns
- `#[cfg(target_os = ...)]` 분기로 플랫폼 기본값 제공.

## Dependencies

### External
- tauri-plugin-global-shortcut (`Code`, `Modifiers`, `Shortcut`)
