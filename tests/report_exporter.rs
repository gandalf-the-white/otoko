use chrono::Utc;
use tempfile::tempdir;
use uuid::Uuid;

use otoko::{
    domain::{AnalysisReport, ReportMetadata, ReportSummary, SummarySource},
    report::MarkdownReportExporter,
};

#[tokio::test]
async fn exporter_creates_markdown_file() {
    let directory = tempdir().expect("temporary directory");

    let exporter = MarkdownReportExporter::new(directory.path());

    let report = AnalysisReport {
        metadata: ReportMetadata {
            id: Uuid::new_v4(),
            generated_at: Utc::now(),
            schema_version: "1.0".into(),
            incident_count: 0,
            summary_source: SummarySource::Fallback,
        },

        summary: ReportSummary {
            executive_summary: "No incidents detected".into(),
            key_findings: vec![],
            operational_impact: "No confirmed impact".into(),
            priorities: vec![],
        },

        incidents: vec![],
    };

    let path = exporter
        .export(&report)
        .await
        .expect("export should succeed");

    assert!(path.exists());

    let content = tokio::fs::read_to_string(path)
        .await
        .expect("report should be readable");

    assert!(content.contains("No incidents detected"));
    assert!(content.contains(&report.metadata.id.to_string()));
}
