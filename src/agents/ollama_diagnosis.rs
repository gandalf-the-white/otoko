use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use rig::{prelude::*, providers::ollama::Ollama};

use tracing::{debug, info};

use crate::{
    config::DiagnosisConfig,
    domain::{AssessedIncident, Diagnosis, DiagnosisResult},
    probes::ReadOnlyFreeBsdProbe,
    tools::{
        DiskUsageTool, ObservationHistory, RecentLoginsTool, ServiceStatusTool, SocketListTool,
    },
};

use super::{
    DiagnoseIncident, DiagnosisValidator,
    prompt::{DIAGNOSIS_PREAMBLE, build_diagnosis_prompt},
};

pub struct OllamaDiagnosisAgent {
    config: DiagnosisConfig,
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
    validator: DiagnosisValidator,
}

impl OllamaDiagnosisAgent {
    pub fn new(config: DiagnosisConfig, probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Result<Self> {
        Ok(Self {
            config,
            probe,
            validator: DiagnosisValidator,
        })
    }
}

#[async_trait]
impl DiagnoseIncident for OllamaDiagnosisAgent {
    async fn diagnose(&self, incident: &AssessedIncident) -> Result<DiagnosisResult> {
        info!(
            incident_title =
                %incident.incident.title,
            severity =
                ?incident.severity.severity,
            "starting incident diagnosis"
        );

        let history = ObservationHistory::new();

        let client = Ollama::new();

        let model = client.completion(&self.config.model);

        let agent = AgentBuilder::new(model)
            .preamble(DIAGNOSIS_PREAMBLE)
            .default_max_turns(6)
            .tool(ServiceStatusTool::new(
                Arc::clone(&self.probe),
                history.clone(),
            ))
            .tool(DiskUsageTool::new(Arc::clone(&self.probe), history.clone()))
            .tool(SocketListTool::new(
                Arc::clone(&self.probe),
                history.clone(),
            ))
            .tool(RecentLoginsTool::new(
                Arc::clone(&self.probe),
                history.clone(),
            ))
            .build();

        let prompt = build_diagnosis_prompt(incident)?;

        let diagnosis = agent.prompt_typed::<Diagnosis>(prompt).await?.output;

        self.validator.validate(&diagnosis)?;

        debug!(
            probable_cause_count = diagnosis.probable_causes.len(),
            investigation_count = diagnosis.investigations.len(),
            recommendation_count = diagnosis.recommendations.len(),
            "diagnosis validated"
        );

        Ok(DiagnosisResult {
            diagnosis,
            observations: history.snapshot().await,
        })
    }
}
