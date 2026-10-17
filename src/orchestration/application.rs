use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    agents::GenerateReportSummary,
    domain::{AnalysisReport, AnalysisResult, LogBatch},
    report::{MarkdownReportExporter, ReportDataBuilder, ReportGenerator},
};

pub struct MonitoringOutput {
    pub analysis: AnalysisResult,
    pub report: AnalysisReport,
    pub report_path: PathBuf,
}

pub struct MonitoringApplication<P, R> {
    pipeline: P,
    generator: ReportGenerator<R>,
    exporter: MarkdownReportExporter,
}

impl<P, R> MonitoringApplication<P, R>
where
    P: RunAnalysis,
    R: GenerateReportSummary,
{
    pub fn new(pipeline: P, reporter: R, exporter: MarkdownReportExporter) -> Self {
        Self {
            pipeline,
            generator: ReportGenerator::new(reporter),
            exporter,
        }
    }

    pub async fn run(&self, batch: &LogBatch) -> Result<MonitoringOutput> {
        let analysis = self.pipeline.run_analysis(batch).await?;

        let incidents = analysis
            .incidents
            .iter()
            .map(ReportDataBuilder::build_incident)
            .collect();

        let report = self.generator.generate(incidents).await?;

        let report_path = self.exporter.export(&report).await?;

        Ok(MonitoringOutput {
            analysis,
            report,
            report_path,
        })
    }
}

#[async_trait]
pub trait RunAnalysis: Send + Sync {
    async fn run_analysis(&self, batch: &LogBatch) -> Result<AnalysisResult>;
}
