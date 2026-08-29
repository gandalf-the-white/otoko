use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{Incident, LogAnalysis};

use super::CorrelateEvents;

#[derive(Debug, Clone)]
pub struct MockCorrelator {
    incidents: Vec<Incident>,
}

impl MockCorrelator {
    pub fn new(incidents: Vec<Incident>) -> Self {
        Self { incidents }
    }
}

#[async_trait]
impl CorrelateEvents for MockCorrelator {
    async fn correlate(&self, _analysis: &LogAnalysis) -> Result<Vec<Incident>> {
        Ok(self.incidents.clone())
    }
}
