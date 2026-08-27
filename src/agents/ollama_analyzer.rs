use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama};
use tracing::{debug, info};

use crate::{
    agents::{
        EvidenceValidator,
        prompt::{ANALYZER_PREAMBLE, build_analyzer_prompt},
    },
    config::AnalyzerConfig,
    domain::{LogAnalysis, LogBatch},
};

use super::AnalyzeLogs;

pub struct OllamaLogAnalyzer {
    agent: Agent,
    validator: EvidenceValidator,
}

impl OllamaLogAnalyzer {
    pub fn new(config: AnalyzerConfig) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client
            .agent(&config.model)
            .preamble(ANALYZER_PREAMBLE)
            .build();

        Ok(Self {
            agent,
            validator: EvidenceValidator,
        })
    }
}

#[async_trait]
impl AnalyzeLogs for OllamaLogAnalyzer {
    async fn analyze(&self, batch: &LogBatch) -> Result<LogAnalysis> {
        info!(log_count = batch.len(), "starting log analysis");

        if batch.is_empty() {
            debug!("empty batch, skipping LLM");

            return Ok(LogAnalysis { events: Vec::new() });
        }

        let prompt = build_analyzer_prompt(batch)?;

        let analysis = self.agent.prompt_typed::<LogAnalysis>(prompt).await?;

        info!(
            event_count = analysis.events.len(),
            "LLM analysis completed"
        );

        self.validator.validate(batch, &analysis)?;

        debug!("evidence validation completed");

        Ok(analysis)
    }
}
