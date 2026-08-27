use otoko::{
    agents::{AnalyzeLogs, MockLogAnalyzer},
    domain::{DetectedEvent, EventCategory, LogAnalysis, LogBatch},
};

#[tokio::test]
async fn mock_analyzer_returns_configured_analysis() {
    let expected = LogAnalysis {
        events: vec![DetectedEvent {
            category: EventCategory::Authentication,
            summary: "Repeated SSH authentication failures".into(),
            evidence: vec!["Failed password for root".into()],
        }],
    };

    let analyzer = MockLogAnalyzer::new(expected.clone());

    let batch = LogBatch::new(Vec::new());

    let result = analyzer
        .analyze(&batch)
        .await
        .expect("analysis should succeed");

    assert_eq!(result, expected);
}

#[tokio::test]
async fn mock_analyzer_can_return_multiple_events() {
    let expected = LogAnalysis {
        events: vec![
            DetectedEvent {
                category: EventCategory::Authentication,
                summary: "SSH failures".into(),
                evidence: vec!["failure".into()],
            },
            DetectedEvent {
                category: EventCategory::Storage,
                summary: "Disk full".into(),
                evidence: vec!["No space left on device".into()],
            },
        ],
    };

    let analyzer = MockLogAnalyzer::new(expected.clone());

    let result = analyzer
        .analyze(&LogBatch::new(Vec::new()))
        .await
        .expect("analysis should succeed");

    assert_eq!(result.events.len(), 2);
    assert_eq!(result, expected);
}

#[tokio::test]
async fn pipeline_can_use_mock_analyzer() {
    use freebsd_ai_monitor::{
        collector::{FakeLogSource, LogScenario, LogSource},
        normalizer::{LogNormalizer, SyslogNormalizer},
    };

    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect().expect("collection should succeed");

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&raw_logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 4);

    let expected = LogAnalysis {
        events: vec![DetectedEvent {
            category: EventCategory::Authentication,
            summary: "Repeated SSH failures".into(),
            evidence: vec!["Failed password".into()],
        }],
    };

    let analyzer = MockLogAnalyzer::new(expected.clone());

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("analysis should succeed");

    assert_eq!(analysis, expected);
}
