use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama};

use tracing::{debug, info};

use crate::{
    config::DiagnosisConfig,
    domain::{AssessedIncident, Diagnosis},
};

use super::{
    DiagnoseIncident, DiagnosisValidator,
    prompt::{DIAGNOSIS_PREAMBLE, build_diagnosis_prompt},
};

pub struct OllamaDiagnosisAgent {
    agent: Agent,
    validator: DiagnosisValidator,
}

impl OllamaDiagnosisAgent {
    pub fn new(config: DiagnosisConfig) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client
            .agent(&config.model)
            .preamble(DIAGNOSIS_PREAMBLE)
            .build();

        Ok(Self {
            agent,
            validator: DiagnosisValidator,
        })
    }
}

#[async_trait]
impl DiagnoseIncident for OllamaDiagnosisAgent {
    async fn diagnose(&self, incident: &AssessedIncident) -> Result<Diagnosis> {
        info!(
            incident_title =
                %incident.incident.title,
            severity =
                ?incident.severity.severity,
            "starting incident diagnosis"
        );

        let prompt = build_diagnosis_prompt(incident)?;

        let diagnosis = self.agent.prompt_typed::<Diagnosis>(prompt).await?;

        self.validator.validate(&diagnosis)?;

        debug!(
            probable_cause_count = diagnosis.probable_causes.len(),
            investigation_count = diagnosis.investigations.len(),
            recommendation_count = diagnosis.recommendations.len(),
            "diagnosis validated"
        );

        Ok(diagnosis)
    }
}
