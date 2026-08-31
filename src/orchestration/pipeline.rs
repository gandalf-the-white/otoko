use anyhow::Result;

use futures::future::try_join_all;
use tokio::sync::Semaphore;

use tracing::{debug, info};

use crate::{
    agents::{AnalyzeLogs, AssessSeverity, CorrelateEvents, DiagnoseIncident},
    config::PipelineConfig,
    domain::{AnalysisResult, AssessedIncident, DiagnosedIncident, Incident, LogBatch},
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("max_concurrent_severity_assessments must be greater than zero")]
    InvalidConcurrencyLimit,

    #[error("max_concurrent_diagnoses must be greater than zero")]
    InvalidDiagnosisConcurrencyLimit,
}

pub struct AnalysisPipeline<A, C, S, D> {
    analyzer: A,
    correlator: C,
    severity: S,
    diagnosis: D,
    config: PipelineConfig,
}

impl<A, C, S, D> AnalysisPipeline<A, C, S, D> {
    pub fn new(
        analyzer: A,
        correlator: C,
        severity: S,
        diagnosis: D,
        config: PipelineConfig,
    ) -> Result<Self, PipelineError> {
        if config.max_concurrent_severity_assessments == 0 {
            return Err(PipelineError::InvalidConcurrencyLimit);
        }

        if config.max_concurrent_diagnoses == 0 {
            return Err(PipelineError::InvalidDiagnosisConcurrencyLimit);
        }

        Ok(Self {
            analyzer,
            correlator,
            severity,
            diagnosis,
            config,
        })
    }
}

impl<A, C, S, D> AnalysisPipeline<A, C, S, D>
where
    A: AnalyzeLogs + Sync,
    C: CorrelateEvents + Sync,
    S: AssessSeverity + Sync,
    D: DiagnoseIncident + Sync,
{
    pub async fn run(&self, batch: &LogBatch) -> Result<AnalysisResult> {
        info!(log_count = batch.len(), "analysis pipeline started");

        let analysis = self.analyzer.analyze(batch).await?;

        info!(
            event_count = analysis.events.len(),
            "log analysis completed"
        );

        let incidents = self.correlator.correlate(&analysis).await?;

        info!(
            incident_count = incidents.len(),
            "incident correlation completed"
        );

        let assessed_incidents = self.assess_incidents(incidents).await?;

        info!(
            assessed_incident_count = assessed_incidents.len(),
            "severity assessments completed"
        );

        let diagnosed_incidents = self.diagnose_incidents(assessed_incidents).await?;

        info!(
            diagnosed_incident_count = diagnosed_incidents.len(),
            "diagnosis completed"
        );

        Ok(AnalysisResult {
            analysis,
            incidents: diagnosed_incidents,
        })
    }

    async fn assess_incidents(&self, incidents: Vec<Incident>) -> Result<Vec<AssessedIncident>> {
        debug!(
            incident_count = incidents.len(),
            concurrency_limit = self.config.max_concurrent_severity_assessments,
            "starting concurrent severity assessments"
        );

        let semaphore = Semaphore::new(self.config.max_concurrent_severity_assessments);

        let futures = incidents.into_iter().map(|incident| {
            let semaphore = &semaphore;

            let severity = &self.severity;

            async move {
                let _permit = semaphore.acquire().await?;

                let assessment = severity.assess(&incident).await?;

                Ok::<AssessedIncident, anyhow::Error>(AssessedIncident {
                    incident,
                    severity: assessment,
                })
            }
        });

        try_join_all(futures).await
    }

    async fn diagnose_incidents(
        &self,
        incidents: Vec<AssessedIncident>,
    ) -> Result<Vec<DiagnosedIncident>> {
        debug!(
            incident_count = incidents.len(),
            concurrency_limit = self.config.max_concurrent_diagnoses,
            "starting concurrent diagnoses"
        );

        let semaphore = Semaphore::new(self.config.max_concurrent_diagnoses);

        let futures = incidents.into_iter().map(|incident| {
            let semaphore = &semaphore;

            let diagnosis = &self.diagnosis;

            async move {
                let _permit = semaphore.acquire().await?;

                let result = diagnosis.diagnose(&incident).await?;

                Ok::<DiagnosedIncident, anyhow::Error>(DiagnosedIncident {
                    assessed: incident,

                    diagnosis: result,
                })
            }
        });

        try_join_all(futures).await
    }
}
