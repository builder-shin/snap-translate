<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# accessibility

## Purpose
macOS Accessibility 권한 상태 확인 및 첫 실행 시 시스템 프롬프트 노출. Non-macOS 플랫폼에서는 항상 `true`를 반환(권한 개념 없음). `AccessibilityChecker` trait으로 추상화되어 `translate_flow`에서 키 시뮬레이션 전 필수 선행 검사로 사용된다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `checker` 재export |
| `checker.rs` | `AccessibilityChecker` trait, `PlatformAccessibilityChecker` (mac: `macos_accessibility_client` 호출, 그 외: `true`) |

## For AI Agents

### Working In This Directory
- `is_trusted()` vs `check_and_prompt()`: 전자는 단순 조회(프롬프트 없음), 후자는 미허가 시 macOS 시스템 대화상자를 띄운다. 호출 순서는 `is_trusted() == false`일 때만 `check_and_prompt()` — 매 단축키 입력마다 프롬프트가 뜨면 UX가 크게 손상된다.
- `lib.rs`는 `setup()`에서 1회 조건부 프롬프트를, `translate_flow`는 `is_trusted()` 실패 시 `check_and_prompt()` 호출로 권한 요청을 재유도한다.
- macOS 미디어 의존성 `macos_accessibility_client`는 `Cargo.toml`의 `[target.'cfg(target_os = "macos")'.dependencies]`에만 있으므로 크로스 컴파일 시 Linux/Windows 코드에서 이 심볼을 참조하면 안 된다 (반드시 `#[cfg(target_os = "macos")]` 가드).
- Windows/Linux는 현재 `true` 반환 — 향후 Windows의 UIAccess 유사 권한을 추가할 경우 플랫폼 분기 확장.

### Testing Requirements
- `MockAccessibilityChecker::{trusted, not_trusted}`로 분기 테스트 유지. 실제 macOS API는 CI에서 호출 불가.

### Common Patterns
- 상태 없는 struct, `new()`만 존재.
- trait `Send + Sync` 경계로 Tauri async 컨텍스트에서 공유 가능.

## Dependencies

### External
- macos-accessibility-client 0.0.1 (macOS only)
