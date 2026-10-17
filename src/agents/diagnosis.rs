use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{AssessedIncident, DiagnosisResult};

#[async_trait]
pub trait DiagnoseIncident: Send + Sync {
    async fn diagnose(&self, incident: &AssessedIncident) -> Result<DiagnosisResult>;
}
