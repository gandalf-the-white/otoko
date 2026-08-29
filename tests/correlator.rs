use otoko::{
    agents::{AnalyzeLogs, CorrelateEvents, MockCorrelator, MockLogAnalyzer},
    domain::{DetectedEvent, EventCategory, Incident, LogAnalysis, LogBatch},
};

#[tokio::test]
async fn mock_correlator_returns_configured_incidents() {
    let event = DetectedEvent {
        category: EventCategory::Authentication,

        summary: "Repeated SSH failures".into(),

        evidence: vec!["Failed password".into()],
    };

    let expected = vec![Incident {
        title: "Suspicious SSH activity".into(),

        events: vec![event],

        explanation: "Repeated authentication failures".into(),
    }];

    let correlator = MockCorrelator::new(expected.clone());

    let analysis = LogAnalysis { events: Vec::new() };

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    assert_eq!(incidents, expected);
}

#[tokio::test]
async fn mock_correlator_can_return_multiple_incidents() {
    let ssh_event = DetectedEvent {
        category: EventCategory::Authentication,
        summary: "Repeated SSH failures".into(),
        evidence: vec!["Failed password".into()],
    };

    let service_event = DetectedEvent {
        category: EventCategory::Service,
        summary: "PostgreSQL restarted".into(),
        evidence: vec!["postgresql restarted".into()],
    };

    let expected = vec![
        Incident {
            title: "Suspicious SSH activity".into(),
            events: vec![ssh_event],
            explanation: "Repeated authentication failures".into(),
        },
        Incident {
            title: "PostgreSQL restart".into(),
            events: vec![service_event],
            explanation: "PostgreSQL service was restarted".into(),
        },
    ];

    let correlator = MockCorrelator::new(expected.clone());

    let analysis = LogAnalysis { events: Vec::new() };

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    assert_eq!(incidents.len(), 2);
    assert_eq!(incidents, expected);
}

#[tokio::test]
async fn mock_correlator_can_return_no_incident() {
    let correlator = MockCorrelator::new(Vec::new());

    let analysis = LogAnalysis { events: Vec::new() };

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    assert!(incidents.is_empty());
}

#[tokio::test]
async fn analyzer_and_correlator_can_work_together() {
    let event = DetectedEvent {
        category: EventCategory::Authentication,
        summary: "Repeated SSH failures".into(),
        evidence: vec!["Failed password".into()],
    };

    let expected_analysis = LogAnalysis {
        events: vec![event.clone()],
    };

    let analyzer = MockLogAnalyzer::new(expected_analysis);

    let expected_incident = Incident {
        title: "Suspicious SSH activity".into(),
        events: vec![event],
        explanation: "Authentication failures detected".into(),
    };

    let correlator = MockCorrelator::new(vec![expected_incident.clone()]);

    let batch = LogBatch::new(Vec::new());

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("analysis should succeed");

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    assert_eq!(incidents, vec![expected_incident],);
}
