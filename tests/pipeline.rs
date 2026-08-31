use anyhow::Result;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use async_trait::async_trait;
use otoko::{
    agents::{AssessSeverity, MockCorrelator, MockLogAnalyzer, MockSeverityAgent},
    config::PipelineConfig,
    domain::{
        DetectedEvent, EventCategory, Incident, LogAnalysis, LogBatch, Severity, SeverityAssessment,
    },
    orchestration::AnalysisPipeline,
};
use tokio::time::sleep;

#[tokio::test]
async fn pipeline_produces_assessed_incidents() {
    let event = DetectedEvent {
        category: EventCategory::Authentication,

        summary: "SSH failures".into(),

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

        justification: "Suspicious activity".into(),
    };

    let pipeline = AnalysisPipeline::new(
        Arc::new(MockLogAnalyzer::new(analysis.clone())),
        Arc::new(MockCorrelator::new(vec![incident.clone()])),
        Arc::new(MockSeverityAgent::new(severity.clone())),
        PipelineConfig::new(2).expect("config should be valid"),
    );

    let result = pipeline
        .run(&LogBatch::new(Vec::new()))
        .await
        .expect("pipeline should succeed");

    assert_eq!(result.analysis, analysis);

    assert_eq!(result.incidents.len(), 1);

    assert_eq!(result.incidents[0].incident, incident);

    assert_eq!(result.incidents[0].severity, severity);
}

#[tokio::test]
async fn pipeline_handles_no_incidents() {
    let pipeline = AnalysisPipeline::new(
        Arc::new(MockLogAnalyzer::new(LogAnalysis { events: Vec::new() })),
        Arc::new(MockCorrelator::new(Vec::new())),
        Arc::new(MockSeverityAgent::new(SeverityAssessment {
            severity: Severity::Informational,

            confidence: 1.0,

            justification: "unused".into(),
        })),
        PipelineConfig::new(2).expect("config should be valid"),
    );

    let result = pipeline
        .run(&LogBatch::new(Vec::new()))
        .await
        .expect("pipeline should succeed");

    assert!(result.incidents.is_empty());
}

struct TrackingSeverityAgent {
    active: Arc<AtomicUsize>,

    maximum: Arc<AtomicUsize>,
}

#[async_trait]
impl AssessSeverity for TrackingSeverityAgent {
    async fn assess(&self, _incident: &Incident) -> Result<SeverityAssessment> {
        let active = self.active.fetch_add(1, Ordering::SeqCst) + 1;

        self.maximum.fetch_max(active, Ordering::SeqCst);

        sleep(Duration::from_millis(50)).await;

        self.active.fetch_sub(1, Ordering::SeqCst);

        Ok(SeverityAssessment {
            severity: Severity::Medium,

            confidence: 0.8,

            justification: "test".into(),
        })
    }
}

#[tokio::test]
async fn pipeline_limits_concurrent_severity_calls() {
    let active = Arc::new(AtomicUsize::new(0));

    let maximum = Arc::new(AtomicUsize::new(0));

    let severity_agent = TrackingSeverityAgent {
        active: Arc::clone(&active),

        maximum: Arc::clone(&maximum),
    };

    let incidents = (0..5)
        .map(|index| Incident {
            title: format!("Incident {index}"),

            events: Vec::new(),

            explanation: "test".into(),
        })
        .collect();

    let pipeline = AnalysisPipeline::new(
        Arc::new(MockLogAnalyzer::new(LogAnalysis { events: Vec::new() })),
        Arc::new(MockCorrelator::new(incidents)),
        Arc::new(severity_agent),
        PipelineConfig::new(2).expect("config should be valid"),
    );

    pipeline
        .run(&LogBatch::new(Vec::new()))
        .await
        .expect("pipeline should succeed");

    assert_eq!(maximum.load(Ordering::SeqCst), 2);
}
