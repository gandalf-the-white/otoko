use otoko::{
    agents::{AssessSeverity, MockSeverityAgent},
    domain::{Incident, Severity, SeverityAssessment},
};

#[tokio::test]
async fn mock_severity_returns_configured_assessment() {
    let expected = SeverityAssessment {
        severity: Severity::High,

        confidence: 0.87,

        justification: "Several related security events".into(),
    };

    let agent = MockSeverityAgent::new(expected.clone());

    let incident = Incident {
        title: "Suspicious SSH activity".into(),

        events: Vec::new(),

        explanation: "Related SSH events".into(),
    };

    let assessment = agent
        .assess(&incident)
        .await
        .expect("assessment should succeed");

    assert_eq!(assessment, expected);
}

#[tokio::test]
async fn multi_agent_pipeline_can_use_mocks() {
    use otoko::{
        agents::{
            AnalyzeLogs, AssessSeverity, CorrelateEvents, MockCorrelator, MockLogAnalyzer,
            MockSeverityAgent,
        },
        domain::{
            DetectedEvent, EventCategory, Incident, LogAnalysis, LogBatch, Severity,
            SeverityAssessment,
        },
    };

    let event = DetectedEvent {
        category: EventCategory::Authentication,

        summary: "Repeated SSH failures".into(),

        evidence: vec!["Failed password".into()],
    };

    let analysis = LogAnalysis {
        events: vec![event.clone()],
    };

    let incident = Incident {
        title: "Suspicious SSH activity".into(),

        events: vec![event],

        explanation: "Related authentication activity".into(),
    };

    let severity = SeverityAssessment {
        severity: Severity::High,

        confidence: 0.9,

        justification: "Suspicious authentication activity".into(),
    };

    let analyzer = MockLogAnalyzer::new(analysis);

    let correlator = MockCorrelator::new(vec![incident]);

    let severity_agent = MockSeverityAgent::new(severity.clone());

    let batch = LogBatch::new(Vec::new());

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("analysis should succeed");

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    let assessment = severity_agent
        .assess(&incidents[0])
        .await
        .expect("severity assessment should succeed");

    assert_eq!(assessment, severity);
}
