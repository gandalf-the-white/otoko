use anyhow::Result;
use chrono::Utc;
use tracing::warn;
use uuid::Uuid;

use crate::{
    agents::GenerateReportSummary,
    domain::{AnalysisReport, IncidentReport, ReportMetadata, SummarySource},
    report::FallbackReportSummary,
    validators::SummaryValidator,
};

pub struct ReportGenerator<R> {
    reporter: R,
}

impl<R> ReportGenerator<R>
where
    R: GenerateReportSummary,
{
    pub fn new(reporter: R) -> Self {
        Self { reporter }
    }

    pub async fn generate(&self, incidents: Vec<IncidentReport>) -> Result<AnalysisReport> {
        let (summary, summary_source) = match self.reporter.summarize(&incidents).await {
            Ok(summary) => match SummaryValidator::validate(&summary) {
                Ok(()) => (summary, SummarySource::Ollama),

                Err(error) => {
                    warn!(
                        %error,
                        "invalid AI summary, using fallback"
                    );

                    (
                        FallbackReportSummary::generate(&incidents),
                        SummarySource::Fallback,
                    )
                }
            },

            Err(error) => {
                warn!(
                    %error,
                    "reporter failed, using fallback"
                );

                (
                    FallbackReportSummary::generate(&incidents),
                    SummarySource::Fallback,
                )
            }
        };

        Ok(AnalysisReport {
            metadata: ReportMetadata {
                id: Uuid::new_v4(),
                generated_at: Utc::now(),
                schema_version: "1.0".into(),
                incident_count: incidents.len(),
                summary_source,
            },

            summary,
            incidents,
        })
    }
}
