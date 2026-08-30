use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama};

use tracing::{debug, info};

use crate::{
    config::SeverityConfig,
    domain::{Incident, SeverityAssessment},
};

use super::{
    AssessSeverity, SeverityValidator,
    prompt::{SEVERITY_PREAMBLE, build_severity_prompt},
};

pub struct OllamaSeverityAgent {
    agent: Agent,
    validator: SeverityValidator,
}

impl OllamaSeverityAgent {
    pub fn new(config: SeverityConfig) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client
            .agent(&config.model)
            .preamble(SEVERITY_PREAMBLE)
            .build();

        Ok(Self {
            agent,
            validator: SeverityValidator,
        })
    }
}

#[async_trait]
impl AssessSeverity for OllamaSeverityAgent {
    async fn assess(&self, incident: &Incident) -> Result<SeverityAssessment> {
        info!(
            incident_title =
                %incident.title,
            event_count =
                incident.events.len(),
            "starting severity assessment"
        );

        let prompt = build_severity_prompt(incident)?;

        let assessment = self
            .agent
            .prompt_typed::<SeverityAssessment>(prompt)
            .await?;

        self.validator.validate(&assessment)?;

        debug!(
            severity =
                ?assessment.severity,
            confidence =
                assessment.confidence,
            "severity assessment validated"
        );

        Ok(assessment)
    }
}
