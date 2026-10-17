use otoko::{
    agents::{CorrelateEvents, MockCorrelator},
    domain::{DetectedEvent, EventCategory, Incident, LogAnalysis},
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
