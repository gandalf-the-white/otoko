use chrono::{TimeZone, Utc};
use otoko::{
    agents::{AnalyzeLogs, EvidenceValidator, MockLogAnalyzer},
    domain::{DetectedEvent, EventCategory, LogAnalysis, LogBatch, LogEntry},
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
    use otoko::{
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

#[test]
fn validator_accepts_existing_evidence() {
    let batch = LogBatch::new(vec![LogEntry {
        timestamp: Utc.with_ymd_and_hms(2026, 8, 26, 14, 1, 2).unwrap(),

        host: "freebsd".into(),

        service: "sshd".into(),

        message: "Failed password for root from 10.0.0.52".into(),
    }]);

    let analysis = LogAnalysis {
        events: vec![DetectedEvent {
            category: EventCategory::Authentication,

            summary: "SSH authentication failure".into(),

            evidence: vec!["Failed password for root".into()],
        }],
    };

    let validator = EvidenceValidator;

    validator
        .validate(&batch, &analysis)
        .expect("evidence should be valid");
}

#[test]
fn validator_rejects_invented_evidence() {
    let batch = LogBatch::new(vec![LogEntry {
        timestamp: Utc.with_ymd_and_hms(2026, 8, 26, 14, 1, 2).unwrap(),

        host: "freebsd".into(),

        service: "sshd".into(),

        message: "Failed password for root from 10.0.0.52".into(),
    }]);

    let analysis = LogAnalysis {
        events: vec![DetectedEvent {
            category: EventCategory::Authentication,

            summary: "Possible account compromise".into(),

            evidence: vec!["Successful root login from 10.0.0.52".into()],
        }],
    };

    let validator = EvidenceValidator;

    let result = validator.validate(&batch, &analysis);

    assert!(result.is_err());
}

#[test]
fn validator_reports_missing_evidence() {
    use otoko::agents::EvidenceValidationError;

    let batch = LogBatch::new(Vec::new());

    let analysis = LogAnalysis {
        events: vec![DetectedEvent {
            category: EventCategory::Security,

            summary: "Suspicious activity".into(),

            evidence: vec!["unknown evidence".into()],
        }],
    };

    let validator = EvidenceValidator;

    let error = validator
        .validate(&batch, &analysis)
        .expect_err("validation should fail");

    match error {
        EvidenceValidationError::EvidenceNotFound(value) => {
            assert_eq!(value, "unknown evidence");
        }
    }
}
