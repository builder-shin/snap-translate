<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# key_simulator

## Purpose
`Cmd+C`(macOS) / `Ctrl+C`(Windows·Linux) 단축키 시뮬레이션 추상화. `KeySimulator` trait과 `enigo` 크레이트 기반 `EnigoKeySimulator` 실구현을 제공한다. 핫키 핸들러에서 현재 포커스된 앱의 선택 텍스트를 시스템 클립보드로 강제 복사하는 데 사용된다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `simulator` 재export |
| `simulator.rs` | `KeySimulator` trait, `EnigoKeySimulator`(Press modifier → Click 'c' → Release), 플랫폼별 `Key::Meta`/`Key::Control` 분기 |

## For AI Agents

### Working In This Directory
- `simulate_copy`는 의도적으로 Press/Click/Release 3단계로 나뉨 — Enigo가 modifier 홀드를 제대로 인식하려면 이 순서가 필요하다. `key(Meta+C, Click)`식 축약은 일부 macOS 환경에서 실패한 이력이 있으므로 변경 금지.
- macOS에서 `Key::Meta`가 Cmd에 매핑되지만 동작하려면 **Accessibility 권한**이 필수. 권한 미부여 시 `simulate_copy`는 성공 반환해도 실제 복사가 일어나지 않고, `translate_flow`는 `NothingSelected`로 귀결된다. 따라서 호출 전 `AccessibilityChecker::is_trusted()` 확인이 필수.
- 모든 Enigo 에러는 `AppError::KeySimulationError(String)`로 래핑.

### Testing Requirements
- 실제 Enigo 호출은 OS 포커스 상태에 의존하므로 CI에서 실행 금지. `MockKeySimulator`로 호출 횟수 / 실패 케이스만 검증.
- `translate_flow`에서 simulate_copy 실패 시 `restore_clipboard` 후 에러 반환 플로 테스트 유지.

### Common Patterns
- 상태 없는 struct(`EnigoKeySimulator`) — 매 호출마다 `Enigo::new(&Settings::default())` 인스턴스 생성. 성능 병목은 아니지만 핫패스에 있다는 점은 인지할 것.

## Dependencies

### Internal
- `crate::errors::AppError::KeySimulationError`

### External
- enigo 0.3 (Keyboard, Key, Settings, Direction)
