use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{LogAnalysis, LogBatch};

mod mock_analyzer;
mod ollama_analyzer;

pub use mock_analyzer::MockLogAnalyzer;
pub use ollama_analyzer::OllamaLogAnalyzer;

#[async_trait]
pub trait AnalyzeLogs {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis>;
}
