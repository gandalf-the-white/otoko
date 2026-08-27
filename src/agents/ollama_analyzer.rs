use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama, serde_json};

use crate::domain::{LogAnalysis, LogBatch};

use super::AnalyzeLogs;

const PREAMBLE: &str = r#"
You are a system log analysis agent specialized in FreeBSD systems.

Your responsibility is limited to identifying significant events
from the supplied normalized log entries.

Rules:

- Base every conclusion only on the supplied logs.
- Do not invent evidence.
- Do not diagnose root causes.
- Do not assign severity.
- Do not recommend remediation.
- Group related log entries into meaningful events.
- Keep summaries concise.
- Every detected event must reference evidence from the input logs.
- If no significant event is present, return an empty event list.
"#;

pub struct OllamaLogAnalyzer {
    agent: Agent,
}

impl OllamaLogAnalyzer {
    pub fn new(model: &str) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client.agent(model).preamble(PREAMBLE).build();

        Ok(Self { agent })
    }
}

#[async_trait]
impl AnalyzeLogs for OllamaLogAnalyzer {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis> {
        if batch.is_empty() {
            return Ok(LogAnalysis { events: Vec::new() });
        }

        let prompt = build_prompt(batch)?;

        let analysis = self.agent.prompt_typed::<LogAnalysis>(prompt).await?;

        Ok(analysis)
    }
}

fn build_prompt(batch: &LogBatch) -> Result<String> {
    let logs = serde_json::to_string_pretty(&batch.entries)?;

    Ok(format!(
        "Analyze the following normalized FreeBSD logs:\n\n{logs}"
    ))
}
