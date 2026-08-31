use otoko::{
    agents::{DiagnoseIncident, OllamaDiagnosisAgent},
    config::DiagnosisConfig,
    domain::{
        AssessedIncident, DetectedEvent, EventCategory, Incident, Severity, SeverityAssessment,
    },
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

    let agent = OllamaDiagnosisAgent::new(DiagnosisConfig::new("qwen3.8"))
        .expect("agent should be created");

    let diagnosis = agent
        .diagnose(&incident)
        .await
        .expect("diagnosis should succeed");

    for cause in &diagnosis.probable_causes {
        assert!((0.0..=1.0).contains(&cause.confidence,));

        assert!(!cause.description.trim().is_empty());
    }

    for investigation in &diagnosis.investigations {
        assert!(!investigation.description.trim().is_empty());
    }

    for recommendation in &diagnosis.recommendations {
        assert!(!recommendation.description.trim().is_empty());
    }
}
