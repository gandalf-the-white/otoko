use std::sync::Arc;

use otoko::{
    agents::{DiagnoseIncident, OllamaDiagnosisAgent},
    config::DiagnosisConfig,
    domain::{
        AssessedIncident, DetectedEvent, EventCategory, Incident, Severity, SeverityAssessment,
    },
    probes::FakeFreeBsdProbe,
};

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn diagnosis_agent_can_use_read_only_tools() {
    let incident = AssessedIncident {
        incident: Incident {
            title: "Possible service failure".into(),

            events: vec![DetectedEvent {
                category: EventCategory::Service,

                summary: "Service stopped unexpectedly".into(),

                evidence: vec!["database system is stopped".into()],
            }],

            explanation: "A service failure was detected".into(),
        },

        severity: SeverityAssessment {
            severity: Severity::Medium,

            confidence: 0.8,

            justification: "Service availability may be affected".into(),
        },
    };

    let probe = Arc::new(FakeFreeBsdProbe);

    let agent = OllamaDiagnosisAgent::new(DiagnosisConfig::new("qwen3:8b"), probe)
        .expect("agent should be created");

    let diagnosis = agent
        .diagnose(&incident)
        .await
        .expect("diagnosis should succeed");

    for cause in &diagnosis.probable_causes {
        assert!((0.0..=1.0).contains(&cause.confidence,));
    }
}
