use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeConfig {
    pub timeout: Duration,
}

impl ProbeConfig {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}
