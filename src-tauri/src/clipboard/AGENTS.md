<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-04-17 | Updated: 2026-04-17 -->

# clipboard

## Purpose
클립보드 추상화. `ClipboardAccess` trait으로 텍스트/이미지 read/write/clear를 정의하고, `backup_clipboard`/`restore_clipboard`로 번역 직전 상태를 안전하게 백업·복원한다. `read_clipboard_with_retry`는 Cmd+C 시뮬레이션 후 새로 복사된 텍스트가 나타날 때까지 지수 지연(50/100/200/400/800ms)으로 재시도한다.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `handler` 재export |
| `handler.rs` | `ClipboardAccess` trait, `ClipboardBackup` enum(Text/Image/Empty), `backup_clipboard`, `restore_clipboard`, `read_clipboard_with_retry`, 테스트용 `MockClipboard` |

## For AI Agents

### Working In This Directory
- 실제 Tauri 클립보드 플러그인 어댑터는 이 모듈이 아니라 `lib.rs`의 `TauriClipboard`가 담당한다 (plugin 의존성을 여기 끌어들이지 않기 위함). 새 실구현 필요 시 동일 패턴(별도 파일)으로 작성.
- `read_clipboard_with_retry`의 "새 텍스트 감지" 규칙: `Text(old) → text != old`, `Image(_) → 항상 new`, `Empty → 비지 않으면 new`. 이 규칙을 바꾸면 `translate_flow`의 "NothingSelected" 시나리오가 깨진다.
- 딜레이 스케줄 `[50, 100, 200, 400, 800]`ms는 수동 튜닝된 값이며, 합계 약 1.55초 내 감지 실패 시 `NothingSelected`.
- `write_image`의 실구현(`TauriClipboard`)은 현재 bytes만으로는 이미지를 재기록할 수 없어 `ClipboardWriteError`를 반환한다 — 텍스트 복원만 보장된다.

### Testing Requirements
- `MockClipboard`는 `text_sequence`로 연속 read 호출에 다른 값을 반환할 수 있다. 재시도 테스트는 이 구조에 의존한다.
- 추가/수정 시 backup/restore/retry 경로 7개 케이스 유지 (text/image/empty 각각 + 재시도 1st/3rd/all-fail/empty-backup/image-backup).

### Common Patterns
- Send + Sync trait 객체 (`&dyn ClipboardAccess`) 주입.
- `ClipboardBackup::Empty`는 복원 시 `clear()` 호출 — 원본이 비어있었음을 의미.

## Dependencies

### Internal
- `crate::errors::AppError::{ClipboardReadError, ClipboardWriteError, NothingSelected}`

### External
- tokio::time::sleep (재시도 지연)
