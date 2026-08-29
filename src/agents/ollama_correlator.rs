use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama};

use tracing::{debug, info};

use crate::{
    config::CorrelatorConfig,
    domain::{Incident, LogAnalysis},
};

use super::{
    CorrelateEvents,
    correlator::{CorrelationPlan, build_incidents},
    prompt::{CORRELATOR_PREAMBLE, build_correlator_prompt},
};

pub struct OllamaCorrelator {
    agent: Agent,
}

impl OllamaCorrelator {
    pub fn new(config: CorrelatorConfig) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client
            .agent(&config.model)
            .preamble(CORRELATOR_PREAMBLE)
            .build();

        Ok(Self { agent })
    }
}

#[async_trait]
impl CorrelateEvents for OllamaCorrelator {
    async fn correlate(&self, analysis: &LogAnalysis) -> Result<Vec<Incident>> {
        info!(
            event_count = analysis.events.len(),
            "starting event correlation"
        );

        if analysis.events.is_empty() {
            debug!("no events, skipping correlator LLM");

            return Ok(Vec::new());
        }

        let prompt = build_correlator_prompt(analysis)?;

        let plan = self.agent.prompt_typed::<CorrelationPlan>(prompt).await?;

        info!(
            incident_count = plan.incidents.len(),
            "LLM correlation completed"
        );

        let incidents = build_incidents(analysis, plan)?;

        debug!("correlation plan validated");

        Ok(incidents)
    }
}
