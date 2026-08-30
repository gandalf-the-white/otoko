use otoko::{
    agents::{AssessSeverity, OllamaSeverityAgent},
    config::SeverityConfig,
    domain::{DetectedEvent, EventCategory, Incident},
};

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn severity_agent_assesses_suspicious_ssh_incident() {
    let incident =
        Incident {
            title:
                "Suspicious SSH session"
                    .into(),

            events: vec![
                DetectedEvent {
                    category:
                        EventCategory::
                            Authentication,

                    summary:
                        "Repeated SSH authentication failures"
                            .into(),

                    evidence: vec![
                        "Failed password for spike from 10.0.0.52"
                            .into(),
                    ],
                },

                DetectedEvent {
                    category:
                        EventCategory::
                            Authentication,

                    summary:
                        "Successful SSH authentication"
                            .into(),

                    evidence: vec![
                        "Accepted password for spike from 10.0.0.52"
                            .into(),
                    ],
                },

                DetectedEvent {
                    category:
                        EventCategory::
                            Security,

                    summary:
                        "Privileged command execution"
                            .into(),

                    evidence: vec![
                        "spike : COMMAND=/usr/bin/id"
                            .into(),
                    ],
                },
            ],

            explanation:
                "Repeated authentication failures were followed by a successful login and privileged activity"
                    .into(),
        };

    let agent =
        OllamaSeverityAgent::new(SeverityConfig::new("qwen3.8")).expect("agent should be created");

    let assessment = agent
        .assess(&incident)
        .await
        .expect("assessment should succeed");

    assert!((0.0..=1.0).contains(&assessment.confidence));

    assert!(!assessment.justification.trim().is_empty());
}
