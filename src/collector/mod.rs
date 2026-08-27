use anyhow::Result;

use crate::domain::RawLog;

mod fake;

pub use fake::{FakeLogSource, LogScenario};

pub trait LogSource {
    fn collect(&self) -> Result<Vec<RawLog>>;
}
