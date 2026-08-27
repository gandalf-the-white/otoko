use crate::domain::{LogAnalysis, LogBatch};

#[derive(Debug, thiserror::Error)]
pub enum EvidenceValidationError {
    #[error("evidence not found in input logs: {0}")]
    EvidenceNotFound(String),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EvidenceValidator;

impl EvidenceValidator {
    pub fn validate(
        &self,
        batch: &LogBatch,
        analysis: &LogAnalysis,
    ) -> Result<(), EvidenceValidationError> {
        for event in &analysis.events {
            for evidence in &event.evidence {
                if !evidence_exists(batch, evidence) {
                    return Err(EvidenceValidationError::EvidenceNotFound(evidence.clone()));
                }
            }
        }

        Ok(())
    }
}

fn evidence_exists(batch: &LogBatch, evidence: &str) -> bool {
    batch
        .entries
        .iter()
        .any(|entry| entry.message.contains(evidence))
}
