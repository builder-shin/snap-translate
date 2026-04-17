# Snap Translate

> 📖 **Language**: **English** · [한국어](./README.ko.md)

A lightweight, keyboard-first translation tool for macOS and Windows. Select text in any application, press a global hotkey, and get an instant DeepL translation copied to your clipboard — without ever leaving your workflow.

![platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)
![framework](https://img.shields.io/badge/framework-Tauri%202-blue)
![license](https://img.shields.io/badge/license-MIT-green)

---

## Table of Contents

- [Why Snap Translate?](#why-snap-translate)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [How to Use](#how-to-use)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Architecture](#architecture)
- [Development](#development)
- [Security & Privacy](#security--privacy)
- [License](#license)

---

## Why Snap Translate?

Most translation tools are built as separate apps or browser extensions, forcing you to **context-switch**: copy the text, open a new window, paste, click translate, read the result, and switch back. That friction adds up — especially when you read documentation, emails, or code comments in languages you do not speak fluently.

**Snap Translate was built to remove that friction entirely.**

### Design Goals

1. **Zero context switching** — The tool lives in your system tray. You never see a new window unless you need to change settings.
2. **One-shot workflow** — Select text → press one hotkey → the translation is in your clipboard. That is the entire interaction.
3. **Works everywhere** — Because it simulates `Cmd+C` / `Ctrl+C` on the active selection, it works in any native app, any browser, any terminal, any IDE. No per-app integration needed.
4. **Respect the clipboard** — Your original clipboard content is backed up before the translation runs and restored automatically afterward, so this tool never silently destroys what you had copied.
5. **Native performance, small footprint** — Built on Tauri 2 with a Rust backend. Idle RAM usage is a fraction of equivalent Electron apps and startup is instant.
6. **Secure by default** — Your DeepL API key is stored in the OS keychain (macOS Keychain / Windows Credential Manager), never in a plain-text config file.

### Who Is It For?

- Developers reading English documentation, stack traces, or GitHub issues.
- Researchers skimming foreign-language papers and sources.
- Anyone who reads emails, articles, or messages across languages several times a day.
- Users who prefer **keyboard-driven, distraction-free tools** over bloated GUIs.

### Why DeepL?

DeepL consistently produces more natural, context-aware translations than most alternatives, particularly for European and East Asian languages. It also offers a generous free tier (500,000 characters/month) that covers most individual usage. Snap Translate auto-detects whether you are using a free (`:fx` suffix) or pro API key and routes requests to the correct endpoint.

---

## Features

- 🖱️ **System tray resident** — No window taking up screen space.
- ⌨️ **Global hotkey** — `Cmd+Shift+D` (macOS) / `Ctrl+Shift+D` (Windows).
- 🌐 **DeepL-powered translation** — High-quality output with automatic source-language detection.
- 🔁 **Automatic retry** — Exponential backoff (1s → 2s → 4s → 8s) on rate limits.
- 📋 **Clipboard backup & restore** — Your original clipboard content is never lost.
- 🔐 **Secure API key storage** — OS-native keychain, never plain text on disk.
- 🌍 **9 target languages** — Korean, English, Japanese, Simplified Chinese, German, French, Spanish, Brazilian Portuguese, Russian.
- 🔔 **Desktop notifications** — See the translation preview without opening a window.
- 📝 **Daily rolling logs** — Automatic log file management for debugging.
- 🛡️ **Concurrency guard** — Prevents double-translation from repeated hotkey presses.

---

## Requirements

| Requirement | Details |
|---|---|
| **OS** | macOS 10.15 (Catalina) or later · Windows 10 or later |
| **DeepL API Key** | Free or Pro tier — [get one here](https://www.deepl.com/pro-api) |
| **macOS Accessibility Permission** | Required so the app can simulate `Cmd+C` on your selection. Prompted on first launch. |
| **Network** | Outbound HTTPS to `api.deepl.com` or `api-free.deepl.com` |

---

## Installation

### Option A — Download a Release (recommended)

Download the latest installer for your platform from the **Releases** page and run it. Both `.dmg` (macOS) and `.msi` (Windows) bundles are provided.

### Option B — Build from Source

See the [Development](#development) section below.

---

## Getting Started

### 1. Get a DeepL API Key

1. Sign up at [DeepL API](https://www.deepl.com/pro-api).
2. Copy your authentication key from the account page.
3. Keys ending in `:fx` are free-tier and use `api-free.deepl.com` automatically; keys without the suffix use `api.deepl.com`.

### 2. Launch Snap Translate

- On first launch, a tray icon appears in the menu bar (macOS) or system tray (Windows).
- **macOS users**: A system dialog will request **Accessibility** permission. This is required for the app to simulate the copy shortcut on your selected text. Grant it via *System Settings → Privacy & Security → Accessibility*.

### 3. Configure Your API Key

1. Click the tray icon (or right-click → **Settings**).
2. Paste your DeepL API key and click **Save**. The app validates the key against DeepL's `/v2/usage` endpoint before storing it.
3. Select your preferred target language from the dropdown.

### 4. Translate Anything

- Select text anywhere — a browser, Slack, a PDF viewer, your terminal.
- Press **`Cmd+Shift+D`** (macOS) or **`Ctrl+Shift+D`** (Windows).
- A desktop notification shows the first 100 characters of the translation.
- The full translation is now in your clipboard — paste it wherever you need.

---

## How to Use

### The Core Workflow

```
  ┌────────────────────┐      ┌──────────────────┐     ┌─────────────────┐
  │ 1. Select text in  │  →   │ 2. Press hotkey  │  →  │ 3. Paste result │
  │    any application │      │    Cmd+Shift+D   │     │    Cmd+V        │
  └────────────────────┘      └──────────────────┘     └─────────────────┘
```

### What Happens Under the Hood

When you press the hotkey, Snap Translate performs these steps atomically:

1. **Concurrency guard** — If a translation is already in progress, the new request is ignored.
2. **Accessibility check** (macOS) — Verifies the app still has permission; re-prompts if revoked.
3. **API key check** — Fetches your key from the OS keychain; opens Settings if missing.
4. **Clipboard backup** — Saves your current clipboard content (text or image).
5. **Simulated copy** — Sends `Cmd+C` / `Ctrl+C` to the focused application.
6. **Retry-read** — Polls the clipboard at 50/100/200/400/800ms intervals for new content.
7. **Length check** — Rejects text over 5,000 characters (DeepL's per-request limit).
8. **Translate** — Calls DeepL `/v2/translate` with exponential-backoff retry on 429 rate limits.
9. **Restore & replace** — Your original clipboard is restored first, then the translation is written.
10. **Notify** — A desktop notification shows the result.

If anything fails, you get a human-readable Korean notification explaining which step failed (network error, invalid key, quota exceeded, nothing selected, text too long, etc.) and your original clipboard is always restored.

### Tray Menu

Right-click (or click on Windows) the tray icon to access:

| Item | Action |
|---|---|
| **설정 (Settings)** | Open the settings window |
| **로그 열기 (Open Log)** | Open the log directory in Finder/Explorer |
| **종료 (Quit)** | Exit the app |

### Example Use Cases

- **Reading docs** — Select a confusing paragraph in MDN, press the hotkey, paste into your notes.
- **Code comments** — Translate Japanese comments in a library you are integrating.
- **Email** — Understand a client email in German without opening a browser tab.
- **Chat** — Quickly translate a message in Slack, Discord, or KakaoTalk.

---

## Configuration

Settings are persisted in two places by design:

| Setting | Where | Why |
|---|---|---|
| **DeepL API key** | OS Keychain (`service="snap-translate"`, `username="deepl-api-key"`) | Secret — must not be on disk in plaintext |
| **Target language, hotkey** | `settings.json` in the app config directory | Non-secret preferences |

**Config directory locations:**
- macOS: `~/Library/Application Support/SnapTranslate/`
- Windows: `%APPDATA%\SnapTranslate\`

**Log directory locations** (daily-rotated files named `snap-translate.log.YYYY-MM-DD`):
- macOS: `~/Library/Logs/SnapTranslate/`
- Windows: `%APPDATA%\SnapTranslate\logs\`

### Supported Target Languages

| Code | Language |
|---|---|
| `KO` | Korean (한국어) — *default* |
| `EN` | English |
| `JA` | Japanese (日本語) |
| `ZH` | Chinese, Simplified (简体中文) |
| `DE` | German (Deutsch) |
| `FR` | French (Français) |
| `ES` | Spanish (Español) |
| `PT` | Portuguese, Brazilian (PT-BR) |
| `RU` | Russian (Русский) |

### Changing the Hotkey

Hotkey customization is planned for a future release. The current build is hard-coded to `Cmd/Ctrl+Shift+D`.

---

## Troubleshooting

### "선택된 텍스트가 없습니다" (Nothing selected)

- On macOS, verify Accessibility permission is granted under *System Settings → Privacy & Security → Accessibility*. Without it, the simulated `Cmd+C` is silently suppressed by the OS.
- Make sure text is actually selected (highlighted) in the focused app.
- Some apps (e.g. locked PDF viewers, DRM'd content) disable copy entirely — these cannot be translated.

### "API Key가 유효하지 않습니다" (Invalid API key)

- Double-check the key you pasted — free-tier keys must include the `:fx` suffix exactly as DeepL provides it.
- Verify your DeepL account is active at [DeepL API Dashboard](https://www.deepl.com/pro-account).

### "번역 할당량이 초과되었습니다" (Quota exceeded)

- You have hit your DeepL character quota for the month. Free tier has 500,000 characters; Pro varies by plan.
- Reset date is shown in your DeepL dashboard.

### No notification appears

- macOS: allow notifications for Snap Translate in *System Settings → Notifications*.
- Windows: check *Settings → System → Notifications*.

### Logs

Open the log directory via the tray menu (**로그 열기**) and share the latest daily log if reporting an issue.

---

## Architecture

Snap Translate is split cleanly between a **Rust backend** (Tauri) and a **React frontend** (settings UI only).

```
┌─────────────────────────────────────────────────┐
│                  System Tray                    │
│  [Settings]  [Open Log]  [Quit]                 │
└─────────────┬───────────────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────────┐
│         React 19 Settings Window                │
│  ApiKeyInput · LanguageSelect · HotkeyInput     │
│       ↕ invoke("command_name", args)            │
└─────────────┬───────────────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────────┐
│          Tauri 2 Rust Backend                   │
│                                                 │
│  Global Hotkey ──► translate_flow               │
│     │                 │                         │
│     ▼                 ▼                         │
│  [Accessibility]  [Clipboard]  [KeySim]         │
│  [ApiKeyStore]    [DeepL HTTP] [Notification]   │
└─────────────────────────────────────────────────┘
              │
              ▼
     DeepL API (free/pro auto-routed)
```

### Key Design Patterns

- **Trait-based dependency injection** — Every external I/O (clipboard, HTTP, keychain, key simulation, accessibility) is behind a trait. This enables full-coverage unit testing with mocks, and makes the translate flow pure and testable.
- **RAII guard** — `TranslationGuard` uses an `AtomicBool` to prevent concurrent translations; the flag is released via `Drop` even on panic.
- **Typed errors** — `AppError` (thiserror-based) carries Korean user-facing messages and serializes as plain strings across the Tauri boundary.
- **Retry with exponential backoff** — `translate_with_retry` handles only `429 Rate Limited` with 1/2/4/8s delays; other errors fail fast.

For deeper documentation, see the hierarchical [`AGENTS.md`](./AGENTS.md) files throughout the repository.

---

## Development

### Prerequisites

- **Node.js** 18+ and **pnpm** 8+ (this project uses pnpm exclusively)
- **Rust** 1.70+ (`rustup` recommended)
- **Tauri prerequisites** for your platform — see [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

### Commands

```bash
# Install frontend dependencies
pnpm install

# Run the app in development mode (hot-reload frontend + Rust)
pnpm tauri dev

# Run frontend tests (Vitest + React Testing Library + jsdom)
pnpm test

# Run Rust backend tests
cd src-tauri && cargo test

# Production build (TypeScript check + Vite bundle + Tauri bundle)
pnpm tauri build
```

### Project Layout

```
snap-translate/
├── src/                 # React 19 frontend (settings UI)
│   ├── components/      # ApiKeyInput, LanguageSelect, HotkeyInput
│   ├── pages/           # Settings page
│   ├── lib/             # Tauri invoke wrappers, constants
│   └── types/           # Shared TS types
├── src-tauri/           # Rust backend
│   └── src/
│       ├── commands/    # #[tauri::command] endpoints
│       ├── config/      # Keychain-based API key storage
│       ├── deepl/       # HTTP client + retry
│       ├── clipboard/   # Backup/restore/retry-read trait
│       ├── key_simulator/   # Enigo Cmd+C / Ctrl+C
│       ├── hotkey/      # Platform-specific shortcut defaults
│       ├── accessibility/   # macOS permission check
│       ├── translate_flow.rs # End-to-end flow (fully unit-tested)
│       ├── errors.rs    # AppError enum
│       └── logging.rs   # Daily-rotating tracing logs
└── tests/               # Vitest frontend tests
```

Every directory contains an `AGENTS.md` with detailed AI-readable documentation.

### Contributing

1. Fork the repository and create a feature branch.
2. Run `pnpm test` and `cargo test` before committing.
3. For changes that touch the translation flow, add or update tests in `src-tauri/src/translate_flow.rs`.
4. User-facing strings are in Korean — preserve that convention.

---

## Security & Privacy

- **Your DeepL API key never leaves your machine** except in the request headers sent directly to DeepL over HTTPS.
- **No telemetry, no analytics, no tracking.** The app makes network calls only to DeepL.
- **No clipboard logging.** Clipboard contents are held in memory only for the duration of a single translation request.
- **Logs contain operational metadata** (timestamps, errors, retry counts) but never translation content or API keys.
- The API key is stored via the `keyring` crate using `apple-native` (macOS Keychain) and `windows-native` (Windows Credential Manager) backends.

---

## License

MIT — see [LICENSE](./LICENSE) if present.

---

<sub>Built with [Tauri 2](https://tauri.app/), [React 19](https://react.dev/), and [DeepL](https://www.deepl.com/).</sub>
