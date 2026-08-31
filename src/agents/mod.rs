use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{LogAnalysis, LogBatch};

mod correlator;
mod mock_analyzer;
mod mock_correlator;
mod mock_severity;
mod ollama_analyzer;
mod ollama_correlator;
mod ollama_severity;
mod prompt;
mod severity;
mod severity_validator;
mod validator;

pub use correlator::{CorrelateEvents, CorrelationError};
pub use mock_analyzer::MockLogAnalyzer;
pub use mock_correlator::MockCorrelator;
pub use mock_severity::MockSeverityAgent;
pub use ollama_analyzer::OllamaLogAnalyzer;
pub use ollama_correlator::OllamaCorrelator;
pub use ollama_severity::OllamaSeverityAgent;
pub use severity::AssessSeverity;
pub use severity_validator::{SeverityValidationError, SeverityValidator};
pub use validator::{EvidenceValidationError, EvidenceValidator};

// #[async_trait]
// pub trait AnalyzeLogs {
//     async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis>;
// }

#[async_trait]
pub trait AnalyzeLogs: Send + Sync {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis>;
}
