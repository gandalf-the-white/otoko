use otoko::{
    agents::{CorrelateEvents, OllamaCorrelator},
    config::CorrelatorConfig,
    domain::{DetectedEvent, EventCategory, LogAnalysis},
};

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn correlator_groups_related_ssh_events() {
    let analysis = LogAnalysis {
        events: vec![
            DetectedEvent {
                category: EventCategory::Authentication,

                summary: "Repeated failed SSH logins".into(),

                evidence: vec!["Failed password for root".into()],
            },
            DetectedEvent {
                category: EventCategory::Authentication,

                summary: "Successful SSH login".into(),

                evidence: vec!["Accepted publickey for spike".into()],
            },
            DetectedEvent {
                category: EventCategory::Security,

                summary: "Privileged sudo activity".into(),

                evidence: vec!["sudo command executed".into()],
            },
        ],
    };

    let config = CorrelatorConfig::new("qwen3:8b");

    let correlator = OllamaCorrelator::new(config).expect("correlator should be created");

    let incidents = correlator
        .correlate(&analysis)
        .await
        .expect("correlation should succeed");

    assert!(!incidents.is_empty());

    assert!(
        incidents
            .iter()
            .any(|incident| { incident.events.len() >= 2 })
    );
}

#[tokio::test]
async fn analyzer_and_correlator_can_work_together() {
    use otoko::{
        agents::{AnalyzeLogs, CorrelateEvents, MockCorrelator, MockLogAnalyzer},
        domain::{DetectedEvent, EventCategory, Incident, LogAnalysis, LogBatch},
    };

    let event = DetectedEvent {
        category: EventCategory::Authentication,

        summary: "Repeated SSH failures".into(),

        evidence: vec!["Failed password".into()],
    };

    let expected_analysis = LogAnalysis {
        events: vec![event.clone()],
    };

    let analyzer = MockLogAnalyzer::new(expected_analysis.clone());

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

    assert_eq!(incidents, vec![expected_incident]);
}
