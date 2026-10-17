use anyhow::Result;
use async_trait::async_trait;

use otoko::{
    agents::GenerateReportSummary,
    domain::{IncidentReport, ReportSummary, SummarySource},
    report::{MarkdownReportRenderer, ReportGenerator},
    validators::SummaryValidator,
};

struct MockReporter;

#[async_trait]
impl GenerateReportSummary for MockReporter {
    async fn summarize(&self, _incidents: &[IncidentReport]) -> Result<ReportSummary> {
        Ok(ReportSummary {
            executive_summary: "Mock summary".into(),

            key_findings: vec!["SSH authentication failures".into()],

            operational_impact: "No confirmed impact".into(),

            priorities: vec!["Review authentication logs".into()],
        })
    }
}

#[test]
fn empty_executive_summary_is_rejected() {
    let summary = ReportSummary {
        executive_summary: String::new(),
        key_findings: vec![],
        operational_impact: "Unknown".into(),
        priorities: vec![],
    };

    assert!(SummaryValidator::validate(&summary).is_err());
}

#[tokio::test]
async fn report_generator_uses_mock_reporter() {
    let generator = ReportGenerator::new(MockReporter);

    let report = generator
        .generate(vec![])
        .await
        .expect("report generation should succeed");

    assert_eq!(report.summary.executive_summary, "Mock summary");

    assert_eq!(report.metadata.summary_source, SummarySource::Ollama);

    assert_eq!(report.metadata.incident_count, 0);

    let markdown = MarkdownReportRenderer::render(&report);

    assert!(markdown.contains("Mock summary"));

    assert!(markdown.contains("SSH authentication failures"));
}
