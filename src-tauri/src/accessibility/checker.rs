pub trait AccessibilityChecker: Send + Sync {
    fn is_trusted(&self) -> bool;
    fn check_and_prompt(&self) -> bool;
}

/// Real implementation using platform APIs
pub struct PlatformAccessibilityChecker;

impl PlatformAccessibilityChecker {
    pub fn new() -> Self {
        Self
    }
}

impl AccessibilityChecker for PlatformAccessibilityChecker {
    fn is_trusted(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            macos_accessibility_client::accessibility::application_is_trusted()
        }
        #[cfg(not(target_os = "macos"))]
        {
            true
        }
    }

    fn check_and_prompt(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            macos_accessibility_client::accessibility::application_is_trusted_with_prompt()
        }
        #[cfg(not(target_os = "macos"))]
        {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAccessibilityChecker {
        trusted: bool,
    }

    impl MockAccessibilityChecker {
        fn trusted() -> Self {
            Self { trusted: true }
        }

        fn not_trusted() -> Self {
            Self { trusted: false }
        }
    }

    impl AccessibilityChecker for MockAccessibilityChecker {
        fn is_trusted(&self) -> bool {
            self.trusted
        }

        fn check_and_prompt(&self) -> bool {
            self.trusted
        }
    }

    #[test]
    fn test_trusted() {
        let checker = MockAccessibilityChecker::trusted();
        assert!(checker.is_trusted());
    }

    #[test]
    fn test_not_trusted() {
        let checker = MockAccessibilityChecker::not_trusted();
        assert!(!checker.is_trusted());
    }

    #[test]
    fn test_check_and_prompt_when_trusted() {
        let checker = MockAccessibilityChecker::trusted();
        assert!(checker.check_and_prompt());
    }

    #[test]
    fn test_check_and_prompt_when_not_trusted() {
        let checker = MockAccessibilityChecker::not_trusted();
        assert!(!checker.check_and_prompt());
    }
}
