use anyhow::Result;

use crate::domain::{LogBatch, RawLog};

mod syslog;

pub use syslog::SyslogNormalizer;

pub trait LogNormalizer {
    fn normalize(&self, logs: &[RawLog]) -> Result<LogBatch>;
}
