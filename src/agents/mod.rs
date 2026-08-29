use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{LogAnalysis, LogBatch};

mod correlator;
mod mock_analyzer;
mod mock_correlator;
mod ollama_analyzer;
mod ollama_correlator;
mod prompt;
mod validator;

pub use correlator::CorrelateEvents;
pub use mock_analyzer::MockLogAnalyzer;
pub use mock_correlator::MockCorrelator;
pub use ollama_analyzer::OllamaLogAnalyzer;
pub use ollama_correlator::OllamaCorrelator;
pub use validator::{EvidenceValidationError, EvidenceValidator};

#[async_trait]
pub trait AnalyzeLogs {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis>;
}
