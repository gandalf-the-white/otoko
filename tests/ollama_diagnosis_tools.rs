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
async fn diagnosis_agent_diagnoses_ssh_incident() {
    let incident = AssessedIncident {
        incident: Incident {
            title: "Suspicious SSH session".into(),

            events: vec![
                DetectedEvent {
                    category: EventCategory::Authentication,

                    summary: "Repeated SSH authentication failures".into(),

                    evidence: vec!["Failed password for spike from 10.0.0.52".into()],
                },
                DetectedEvent {
                    category: EventCategory::Authentication,

                    summary: "Successful SSH authentication".into(),

                    evidence: vec!["Accepted password for spike from 10.0.0.52".into()],
                },
                DetectedEvent {
                    category: EventCategory::Security,

                    summary: "Privileged command execution".into(),

                    evidence: vec!["spike : COMMAND=/usr/bin/id".into()],
                },
            ],

            explanation:
                "Authentication failures were followed by successful access and privileged activity"
                    .into(),
        },

        severity: SeverityAssessment {
            severity: Severity::High,

            confidence: 0.9,

            justification: "Suspicious authentication sequence".into(),
        },
    };

    let probe = Arc::new(FakeFreeBsdProbe);

    let agent = OllamaDiagnosisAgent::new(DiagnosisConfig::new("qwen3.8"), probe)
        .expect("agent should be created");

    let diagnosis = agent
        .diagnose(&incident)
        .await
        .expect("diagnosis should succeed");

    for cause in &diagnosis.diagnosis.probable_causes {
        assert!((0.0..=1.0).contains(&cause.confidence,));

        assert!(!cause.description.trim().is_empty());
    }

    for investigation in &diagnosis.diagnosis.investigations {
        assert!(!investigation.description.trim().is_empty());
    }

    for recommendation in &diagnosis.diagnosis.recommendations {
        assert!(!recommendation.description.trim().is_empty());
    }
}

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

    let agent = OllamaDiagnosisAgent::new(DiagnosisConfig::new("qwen3.8"), probe)
        .expect("agent should be created");

    let diagnosis = agent
        .diagnose(&incident)
        .await
        .expect("diagnosis should succeed");

    for cause in &diagnosis.diagnosis.probable_causes {
        assert!((0.0..=1.0).contains(&cause.confidence,));
    }
}
