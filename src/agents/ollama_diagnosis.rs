use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use rig::{Agent, client::Nothing, prelude::*, providers::ollama};

use tracing::{debug, info};

use crate::{
    config::DiagnosisConfig,
    domain::{AssessedIncident, Diagnosis},
    probes::ReadOnlyFreeBsdProbe,
    tools::{DiskUsageTool, RecentLoginsTool, ServiceStatusTool, SocketListTool},
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
    pub fn new(config: DiagnosisConfig, probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Result<Self> {
        let client = ollama::Client::new(Nothing)?;

        let agent = client
            .agent(&config.model)
            .preamble(DIAGNOSIS_PREAMBLE)
            .default_max_turns(6)
            .tool(ServiceStatusTool::new(Arc::clone(&probe)))
            .tool(DiskUsageTool::new(Arc::clone(&probe)))
            .tool(SocketListTool::new(Arc::clone(&probe)))
            .tool(RecentLoginsTool::new(probe))
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
