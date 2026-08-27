use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{LogAnalysis, LogBatch};

use super::AnalyzeLogs;

#[derive(Debug, Clone)]
pub struct MockLogAnalyzer {
    result: LogAnalysis,
}

impl MockLogAnalyzer {
    pub fn new(result: LogAnalysis) -> Self {
        Self { result }
    }
}

#[async_trait]
impl AnalyzeLogs for MockLogAnalyzer {
    async fn analyze(&self, _batch: &LogBatch) -> Result<LogAnalysis> {
        Ok(self.result.clone())
    }
}
