use crate::errors::AppError;

pub trait KeySimulator: Send + Sync {
    fn simulate_copy(&self) -> Result<(), AppError>;
}

/// Real implementation using enigo
pub struct EnigoKeySimulator;

impl EnigoKeySimulator {
    pub fn new() -> Self {
        Self
    }
}

impl KeySimulator for EnigoKeySimulator {
    fn simulate_copy(&self) -> Result<(), AppError> {
        use enigo::{Enigo, Key, Keyboard, Settings, Direction};

        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| AppError::KeySimulationError(e.to_string()))?;

        #[cfg(target_os = "macos")]
        let modifier = Key::Meta;
        #[cfg(target_os = "windows")]
        let modifier = Key::Control;
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let modifier = Key::Control;

        enigo.key(modifier, Direction::Press)
            .map_err(|e| AppError::KeySimulationError(e.to_string()))?;
        enigo.key(Key::Unicode('c'), Direction::Click)
            .map_err(|e| AppError::KeySimulationError(e.to_string()))?;
        enigo.key(modifier, Direction::Release)
            .map_err(|e| AppError::KeySimulationError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct MockKeySimulator {
        call_count: AtomicU32,
        should_fail: bool,
    }

    impl MockKeySimulator {
        fn new() -> Self {
            Self {
                call_count: AtomicU32::new(0),
                should_fail: false,
            }
        }

        fn failing() -> Self {
            Self {
                call_count: AtomicU32::new(0),
                should_fail: true,
            }
        }
    }

    impl KeySimulator for MockKeySimulator {
        fn simulate_copy(&self) -> Result<(), AppError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.should_fail {
                Err(AppError::KeySimulationError("mock error".to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_mock_simulate_copy_success() {
        let sim = MockKeySimulator::new();
        sim.simulate_copy().unwrap();
        assert_eq!(sim.call_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_mock_simulate_copy_failure() {
        let sim = MockKeySimulator::failing();
        let result = sim.simulate_copy();
        assert!(matches!(result, Err(AppError::KeySimulationError(_))));
    }
}
