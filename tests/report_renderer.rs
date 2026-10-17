use chrono::Utc;
use otoko::{
    domain::{AnalysisReport, ReportMetadata, ReportSummary, SummarySource},
    report::MarkdownReportRenderer,
};
use uuid::Uuid;

#[test]
fn markdown_report_contains_summary() {
    let report = AnalysisReport {
        metadata: ReportMetadata {
            id: Uuid::new_v4(),
            generated_at: Utc::now(),
            schema_version: "1.0".into(),
            incident_count: 0,
            summary_source: SummarySource::Fallback,
        },

        summary: ReportSummary {
            executive_summary: "One incident detected".into(),
            key_findings: vec!["SSH authentication failures".into()],
            operational_impact: "Potential unauthorized access".into(),
            priorities: vec!["Review authentication logs".into()],
        },

        incidents: vec![],
    };

    let markdown = MarkdownReportRenderer::render(&report);

    assert!(markdown.contains("# FreeBSD Incident Analysis Report"));

    assert!(markdown.contains("One incident detected"));

    assert!(markdown.contains("SSH authentication failures"));

    assert!(markdown.contains("Review authentication logs"));

    assert!(markdown.contains(&report.metadata.id.to_string()));

    assert!(markdown.contains("Fallback"));
}
