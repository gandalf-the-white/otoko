use anyhow::Result;
use async_trait::async_trait;
use rig::{Agent, AgentBuilder, providers::ollama::Ollama};
use tracing::{debug, info};

use crate::{
    agents::{
        AnalyzeLogs, EvidenceValidator,
        prompt::{ANALYZER_PREAMBLE, build_analyzer_prompt},
    },
    config::AnalyzerConfig,
    domain::{LogAnalysis, LogBatch},
};

pub struct OllamaLogAnalyzer {
    agent: Agent,
    validator: EvidenceValidator,
}

impl OllamaLogAnalyzer {
    pub fn new(config: AnalyzerConfig) -> Result<Self> {
        let client = Ollama::new();
        let model = client.completion(&config.model);
        let agent = AgentBuilder::new(model).preamble(ANALYZER_PREAMBLE).build();

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

        let analysis = self.agent.prompt_typed::<LogAnalysis>(prompt).await?.output;

        info!(
            event_count = analysis.events.len(),
            "LLM analysis completed"
        );

        self.validator.validate(batch, &analysis)?;

        Ok(analysis)
    }
}
