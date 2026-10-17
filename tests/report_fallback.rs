use otoko::{
    domain::{IncidentReport, Severity},
    report::FallbackReportSummary,
};

#[test]
fn fallback_handles_empty_incident_list() {
    let summary = FallbackReportSummary::generate(&[]);

    assert!(summary.executive_summary.contains("No incidents"));
}

#[test]
fn fallback_handles_multiple_incidents() {
    let incidents = vec![
        IncidentReport {
            title: "SSH failures".into(),

            summary: "Repeated SSH authentication failures".into(),

            severity: Severity::High,

            evidence: vec![],

            observations: vec![],

            probable_causes: vec![],

            investigations: vec![],

            recommendations: vec![],
        },
        IncidentReport {
            title: "Disk pressure".into(),

            summary: "Filesystem usage approaching capacity".into(),

            severity: Severity::Medium,

            evidence: vec![],

            observations: vec![],

            probable_causes: vec![],

            investigations: vec![],

            recommendations: vec![],
        },
    ];

    let summary = FallbackReportSummary::generate(&incidents);

    assert!(summary.executive_summary.contains("2 incidents"));

    assert_eq!(summary.key_findings.len(), 2);

    assert!(summary.key_findings[0].contains("SSH failures"));

    assert!(summary.key_findings[1].contains("Disk pressure"));
}
