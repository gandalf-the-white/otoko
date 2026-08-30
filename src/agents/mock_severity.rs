use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{Incident, SeverityAssessment};

use super::AssessSeverity;

#[derive(Debug, Clone)]
pub struct MockSeverityAgent {
    assessment: SeverityAssessment,
}

impl MockSeverityAgent {
    pub fn new(assessment: SeverityAssessment) -> Self {
        Self { assessment }
    }
}

#[async_trait]
impl AssessSeverity for MockSeverityAgent {
    async fn assess(&self, _incident: &Incident) -> Result<SeverityAssessment> {
        Ok(self.assessment.clone())
    }
}
