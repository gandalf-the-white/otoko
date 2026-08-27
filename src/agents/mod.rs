use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{LogAnalysis, LogBatch};

mod mock_analyzer;
mod ollama_analyzer;
mod prompt;
mod validator;

pub use mock_analyzer::MockLogAnalyzer;
pub use ollama_analyzer::OllamaLogAnalyzer;
pub use validator::{EvidenceValidationError, EvidenceValidator};

#[async_trait]
pub trait AnalyzeLogs {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis>;
}
