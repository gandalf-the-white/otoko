use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{Incident, SeverityAssessment};

#[async_trait]
pub trait AssessSeverity {
    async fn assess(&self, incident: &Incident) -> Result<SeverityAssessment>;
}
