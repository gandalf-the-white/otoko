use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{AssessedIncident, Diagnosis, DiagnosisResult};

use super::DiagnoseIncident;

#[derive(Debug, Clone)]
pub struct MockDiagnosisAgent {
    diagnosis: Diagnosis,
}

impl MockDiagnosisAgent {
    pub fn new(diagnosis: Diagnosis) -> Self {
        Self { diagnosis }
    }
}

#[async_trait]
impl DiagnoseIncident for MockDiagnosisAgent {
    async fn diagnose(&self, _incident: &AssessedIncident) -> Result<DiagnosisResult> {
        Ok(DiagnosisResult {
            diagnosis: self.diagnosis.clone(),
            observations: Vec::new(),
        })
    }
}
