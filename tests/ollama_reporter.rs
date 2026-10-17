use otoko::agents::{GenerateReportSummary, OllamaReporterAgent};

#[tokio::test]
#[ignore = "requires Ollama"]
async fn ollama_generates_report_summary() {
    let reporter = OllamaReporterAgent::new("qwen3.8").expect("reporter should initialize");

    let summary = reporter
        .summarize(&[])
        .await
        .expect("summary generation should succeed");

    assert!(!summary.executive_summary.trim().is_empty());
}
