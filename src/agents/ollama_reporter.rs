use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, AgentBuilder, providers::ollama::Ollama};

use crate::{
    agents::prompt::REPORTER_PREAMBLE,
    domain::{IncidentReport, ReportSummary},
};

use super::GenerateReportSummary;

pub struct OllamaReporterAgent {
    agent: Agent,
}

impl OllamaReporterAgent {
    pub fn new(model: &str) -> Result<Self> {
        let client = Ollama::new();
        let model = client.completion(model);
        let agent = AgentBuilder::new(model).preamble(REPORTER_PREAMBLE).build();

        Ok(Self { agent })
    }
}

#[async_trait]
impl GenerateReportSummary for OllamaReporterAgent {
    async fn summarize(&self, incidents: &[IncidentReport]) -> Result<ReportSummary> {
        let payload = serde_json::to_string_pretty(incidents)?;

        let prompt = format!("Summarize these validated FreeBSD incidents:\n\n{payload}");

        let summary = self
            .agent
            .prompt_typed::<ReportSummary>(prompt)
            .await?
            .output;

        Ok(summary)
    }
}
