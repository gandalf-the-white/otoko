use otoko::{
    agents::{DiagnoseIncident, MockDiagnosisAgent},
    domain::{
        AssessedIncident, Diagnosis, Incident, Investigation, ProbableCause, Recommendation,
        Severity, SeverityAssessment,
    },
};

#[tokio::test]
async fn mock_diagnosis_returns_configured_diagnosis() {
    let expected = Diagnosis {
        probable_causes: vec![ProbableCause {
            description: "Possible unauthorized access".into(),

            confidence: 0.75,
        }],

        investigations: vec![Investigation {
            description: "Review authentication activity".into(),
        }],

        recommendations: vec![Recommendation {
            description: "Review account access".into(),
        }],
    };

    let agent = MockDiagnosisAgent::new(expected.clone());

    let incident = AssessedIncident {
        incident: Incident {
            title: "Suspicious SSH activity".into(),

            events: Vec::new(),

            explanation: "Authentication anomalies".into(),
        },

        severity: SeverityAssessment {
            severity: Severity::High,

            confidence: 0.9,

            justification: "Suspicious sequence".into(),
        },
    };

    let diagnosis = agent
        .diagnose(&incident)
        .await
        .expect("diagnosis should succeed");

    assert_eq!(diagnosis, expected);
}
