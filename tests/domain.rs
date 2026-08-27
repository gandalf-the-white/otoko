use chrono::{TimeZone, Utc};
use otoko::domain::{
    DetectedEvent, EventCategory, IncidentReport, LogAnalysis, LogBatch, LogEntry, RawLog,
    Severity, SeverityAssessment,
};

#[test]
fn raw_log_contains_source_and_content() {
    let raw_log = RawLog {
        source: "/var/log/auth.log".to_string(),
        content: "Failed password for root".to_string(),
    };

    assert_eq!(raw_log.source, "/var/log/auth.log");
    assert_eq!(raw_log.content, "Failed password for root");
}

#[test]
fn log_entry_contains_normalized_information() {
    let timestamp = Utc.with_ymd_and_hms(2026, 8, 26, 13, 42, 1).unwrap();

    let entry = LogEntry {
        timestamp,
        host: "freebsd-server".to_string(),
        service: "sshd".to_string(),
        message: "Failed password for root".to_string(),
    };

    assert_eq!(entry.host, "freebsd-server");
    assert_eq!(entry.service, "sshd");
    assert_eq!(entry.message, "Failed password for root");
}

#[test]
fn log_batch_reports_its_size() {
    let timestamp = Utc.with_ymd_and_hms(2026, 8, 26, 13, 42, 1).unwrap();

    let entries = vec![
        LogEntry {
            timestamp,
            host: "freebsd-server".to_string(),
            service: "sshd".to_string(),
            message: "Failed password".to_string(),
        },
        LogEntry {
            timestamp,
            host: "freebsd-server".to_string(),
            service: "sshd".to_string(),
            message: "Accepted publickey".to_string(),
        },
    ];

    let batch = LogBatch::new(entries);

    assert_eq!(batch.len(), 2);
    assert!(!batch.is_empty());
}

#[test]
fn empty_log_batch_is_empty() {
    let batch = LogBatch::new(Vec::new());

    assert_eq!(batch.len(), 0);
    assert!(batch.is_empty());
}

#[test]
fn analysis_can_contain_detected_events() {
    let event = DetectedEvent {
        category: EventCategory::Authentication,
        summary: "Repeated SSH authentication failures".to_string(),
        evidence: vec![
            "Failed password for root from 10.0.0.52".to_string(),
            "Failed password for admin from 10.0.0.52".to_string(),
        ],
    };

    let analysis = LogAnalysis {
        events: vec![event],
    };

    assert_eq!(analysis.events.len(), 1);

    assert_eq!(analysis.events[0].category, EventCategory::Authentication);
}

#[test]
fn severity_assessment_contains_confidence() {
    let assessment = SeverityAssessment {
        severity: Severity::High,
        confidence: 0.9,
        justification: "Repeated failed logins followed by successful authentication".to_string(),
    };

    assert_eq!(assessment.severity, Severity::High);
    assert_eq!(assessment.confidence, 0.9);
}

#[test]
fn incident_report_contains_recommendations() {
    let report = IncidentReport {
        title: "Suspicious SSH activity".to_string(),
        severity: Severity::High,
        summary: "Multiple failed SSH attempts".to_string(),
        evidence: vec!["Failed login from 10.0.0.52".to_string()],
        probable_causes: vec!["SSH brute force".to_string()],
        investigations: vec!["Inspect auth.log".to_string()],
        recommendations: vec!["Review SSH configuration".to_string()],
    };

    assert_eq!(report.severity, Severity::High);
    assert_eq!(report.recommendations.len(), 1);
}
