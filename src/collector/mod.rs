use anyhow::Result;

mod fake;

pub use fake::{FakeLogSource, LogScenario};

use crate::domain::RawLog;

pub trait LogSource {
    fn collect(&self) -> Result<Vec<RawLog>>;
}
