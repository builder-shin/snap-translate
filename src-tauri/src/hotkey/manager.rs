// Hotkey manager
// The actual global shortcut registration happens in lib.rs setup
// This module provides helper functions for shortcut management

use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub fn create_default_shortcut() -> Shortcut {
    #[cfg(target_os = "macos")]
    let modifiers = Some(Modifiers::SUPER | Modifiers::SHIFT);
    #[cfg(target_os = "windows")]
    let modifiers = Some(Modifiers::CONTROL | Modifiers::SHIFT);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let modifiers = Some(Modifiers::CONTROL | Modifiers::SHIFT);

    Shortcut::new(modifiers, Code::KeyD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_shortcut() {
        let shortcut = create_default_shortcut();
        // Just verify it doesn't panic
        let _ = shortcut;
    }
}
