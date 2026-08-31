use otoko::{
    agents::{MockCorrelator, MockDiagnosisAgent, MockLogAnalyzer, MockSeverityAgent},
    config::PipelineConfig,
    domain::{
        DetectedEvent, Diagnosis, EventCategory, Incident, LogAnalysis, LogBatch, ProbableCause,
        Severity, SeverityAssessment,
    },
    orchestration::AnalysisPipeline,
};

#[tokio::test]
async fn pipeline_runs_all_agents() {
    let event = DetectedEvent {
        category: EventCategory::Authentication,

        summary: "Repeated SSH failures".into(),

        evidence: vec!["Failed password".into()],
    };

    let analysis = LogAnalysis {
        events: vec![event.clone()],
    };

    let incident = Incident {
        title: "Suspicious SSH activity".into(),

        events: vec![event],

        explanation: "Related authentication events".into(),
    };

    let severity = SeverityAssessment {
        severity: Severity::High,

        confidence: 0.9,

        justification: "Suspicious activity".into(),
    };

    let diagnosis = Diagnosis {
        probable_causes: vec![ProbableCause {
            description: "Possible unauthorized access".into(),

            confidence: 0.7,
        }],

        investigations: Vec::new(),

        recommendations: Vec::new(),
    };

    let pipeline = AnalysisPipeline::new(
        MockLogAnalyzer::new(analysis.clone()),
        MockCorrelator::new(vec![incident.clone()]),
        MockSeverityAgent::new(severity.clone()),
        MockDiagnosisAgent::new(diagnosis.clone()),
        PipelineConfig::new(2, 2),
    )
    .expect("pipeline should be created");

    let result = pipeline
        .run(&LogBatch::new(Vec::new()))
        .await
        .expect("pipeline should succeed");

    assert_eq!(result.analysis, analysis);

    assert_eq!(result.incidents.len(), 1);

    assert_eq!(result.incidents[0].assessed.incident, incident);

    assert_eq!(result.incidents[0].assessed.severity, severity);

    assert_eq!(result.incidents[0].diagnosis, diagnosis);
}
