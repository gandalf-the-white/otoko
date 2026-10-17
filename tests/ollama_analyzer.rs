use otoko::{
    agents::{AnalyzeLogs, OllamaLogAnalyzer},
    collector::{FakeLogSource, LogScenario, LogSource},
    config::AnalyzerConfig,
    domain::{EventCategory, LogBatch},
    normalizer::{LogNormalizer, SyslogNormalizer},
};

#[tokio::test]
async fn empty_batch_returns_empty_analysis() {
    let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new("qwen3.8"))
        .expect("agent creation should succeed");

    let batch = LogBatch::new(Vec::new());

    let analysis = analyzer
        .analyze(&batch)
        .await
        .expect("empty analysis should succeed");

    assert!(analysis.events.is_empty());
}

#[tokio::test]
#[ignore = "requires a running Ollama server and model"]
async fn ollama_detect_ssh_authentication_event() {
    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect().expect("collection should succeed");

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&raw_logs)
        .expect("normalization should succeed");

    let config = AnalyzerConfig::new("qwen3.8");

    let analyzer = OllamaLogAnalyzer::new(config).expect("agent should be created");

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
