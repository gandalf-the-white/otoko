use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{AssessedIncident, Diagnosis};

#[async_trait]
pub trait DiagnoseIncident {
    async fn diagnose(&self, incident: &AssessedIncident) -> Result<Diagnosis>;
}
