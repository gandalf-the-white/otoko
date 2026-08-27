use otoko::{
    agents::{AnalyzeLogs, OllamaLogAnalyzer},
    collector::{FakeLogSource, LogScenario, LogSource},
    domain::{EventCategory, LogBatch},
    normalizer::{LogNormalizer, SyslogNormalizer},
};

#[tokio::test]
async fn empty_batch_returns_empty_analysis() {
    let analyzer = OllamaLogAnalyzer::new("qwen3.6:latest").expect("agent creation should succeed");

    let batch = LogBatch::new(Vec::new());

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("empty analysis should succeed");

    assert!(analysis.events.is_empty());
}

// cargo test ollama_detects_ssh_authentication_event -- --ignored --nocapture

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn ollama_detects_ssh_authentication_event() {
    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect().expect("collection should succeed");

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&raw_logs)
        .expect("normalization should succeed");

    let analyzer = OllamaLogAnalyzer::new("qwen3.6:latest").expect("agent should be created");

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("LLM analysis should succeed");

    assert!(!analysis.events.is_empty());

    assert!(
        analysis
            .events
            .iter()
            .any(|event| { event.category == EventCategory::Authentication })
    );
}

// cargo test ollama_does_not_flag_normal_logs_as_security_incident  -- --ignored --nocapture

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn ollama_does_not_flag_normal_logs_as_security_incident() {
    let source = FakeLogSource::from_scenario(LogScenario::Normal);

    let raw_logs = source.collect().expect("collection should succeed");

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&raw_logs)
        .expect("normalization should succeed");

    let analyzer = OllamaLogAnalyzer::new("qwen3.6:latest").expect("agent should be created");

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("analysis should succeed");

    assert!(
        analysis
            .events
            .iter()
            .all(|event| { event.category != EventCategory::Security })
    );
}
