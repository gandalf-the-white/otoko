use std::sync::Arc;

use anyhow::Result;

use tokio::{sync::Semaphore, task::JoinSet};

use tracing::info;

use crate::{
    agents::{AnalyzeLogs, AssessSeverity, CorrelateEvents},
    config::PipelineConfig,
    domain::{AnalysisResult, AssessedIncident, Incident, LogBatch},
};

pub struct AnalysisPipeline {
    analyzer: Arc<dyn AnalyzeLogs>,

    correlator: Arc<dyn CorrelateEvents>,

    severity_agent: Arc<dyn AssessSeverity>,

    config: PipelineConfig,
}

impl AnalysisPipeline {
    pub fn new(
        analyzer: Arc<dyn AnalyzeLogs>,

        correlator: Arc<dyn CorrelateEvents>,

        severity_agent: Arc<dyn AssessSeverity>,

        config: PipelineConfig,
    ) -> Self {
        Self {
            analyzer,
            correlator,
            severity_agent,
            config,
        }
    }

    pub async fn run(&self, batch: &LogBatch) -> Result<AnalysisResult> {
        info!(log_count = batch.len(), "starting analysis pipeline");

        let analysis = self.analyzer.analyze(batch).await?;

        let incidents = self.correlator.correlate(&analysis).await?;

        info!(incident_count = incidents.len(), "incidents correlated");

        let incidents = self.assess_incidents(incidents).await?;

        info!(
            assessed_incident_count = incidents.len(),
            "analysis pipeline completed"
        );

        Ok(AnalysisResult {
            analysis,
            incidents,
        })
    }

    async fn assess_incidents(&self, incidents: Vec<Incident>) -> Result<Vec<AssessedIncident>> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_severity));

        let mut tasks = JoinSet::new();

        for (index, incident) in incidents.into_iter().enumerate() {
            let severity_agent = Arc::clone(&self.severity_agent);

            let semaphore = Arc::clone(&semaphore);

            tasks.spawn(async move {
                let _permit = semaphore.acquire_owned().await?;

                let assessment = severity_agent.assess(&incident).await?;

                Ok::<_, anyhow::Error>((
                    index,
                    AssessedIncident {
                        incident,

                        severity: assessment,
                    },
                ))
            });
        }

        let mut results = Vec::new();

        while let Some(task_result) = tasks.join_next().await {
            let result = task_result??;

            results.push(result);
        }

        results.sort_by_key(|(index, _)| *index);

        Ok(results.into_iter().map(|(_, incident)| incident).collect())
    }
}
