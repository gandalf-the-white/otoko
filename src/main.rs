use std::sync::Arc;

use anyhow::Result;
use otoko::{
    agents::{OllamaCorrelator, OllamaDiagnosisAgent, OllamaLogAnalyzer, OllamaSeverityAgent},
    collector::{FakeLogSource, LogScenario, LogSource},
    config::{AnalyzerConfig, CorrelatorConfig, DiagnosisConfig, PipelineConfig, SeverityConfig},
    normalizer::{LogNormalizer, SyslogNormalizer},
    orchestration::AnalysisPipeline,
    probes::FakeFreeBsdProbe,
};

const MODEL: &str = "qwen3.8";

#[tokio::main]
async fn main() -> Result<()> {
    // ===========================================
    // SECTION 2
    // ===========================================

    // let source = FakeLogSource::new(vec![RawLog {
    //     source: "/var/log/auth.log".to_string(),
    //     content: "Failed password for root".to_string(),
    // }]);

    // let logs = source.collect()?;

    // println!("{} log files retrieved", logs.len());

    // ===========================================
    // SECTION 3
    // ===========================================

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let logs = source.collect()?;

    // for log in logs {
    //     println!("Source : {}", log.source);
    //     println!("---------------------------");
    //     println!("{}", log.content);
    // }

    // ===========================================
    // SECTION 4
    // ===========================================

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let raw_log = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_log)?;

    // println!("{} Standardized syslog events", batch.len());

    // for entry in batch.entries {
    //     println!(
    //         "{} | {} | {} | {}",
    //         entry.timestamp, entry.host, entry.service, entry.message,
    //     );
    // }

    // ===========================================
    // SECTION 5
    // ===========================================

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // println!("{} entries normalized", batch.len());

    // println!("---------------------------");

    // let config = AnalyzerConfig::new("qwen3.8");

    // let analyzer = OllamaLogAnalyzer::new(config)?;

    // let analysis = analyzer.analyze(&batch).await?;

    // println!("{:#?}", analysis);

    // ===========================================
    // SECTION 6
    // ===========================================

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new("qwen3.8"))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new("qwen3.8"))?;

    // let analysis = analyzer.analyze(&batch).await?;

    // let incidents = correlator.correlate(&analysis).await?;

    // println!("{incidents:#?}");

    // ===========================================
    // SECTION 7
    // ===========================================

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new("qwen3.8"))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new("qwen3.8"))?;

    // let analysis = analyzer.analyze(&batch).await?;

    // let incidents = correlator.correlate(&analysis).await?;

    // let severity_agent = OllamaSeverityAgent::new(SeverityConfig::new("qwen3.8"))?;

    // for incident in &incidents {
    //     let assessment = severity_agent.assess(incident).await?;

    //     println!("Incident: {}", incident.title);

    //     println!("Severity: {:?}", assessment.severity);

    //     println!("Confidence: {:.2}", assessment.confidence);

    //     println!("Justification: {}", assessment.justification);
    // }

    // ===========================================
    // SECTION 8
    // ===========================================

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    // let severity = OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?;

    // let pipeline = AnalysisPipeline::new(analyzer, correlator, severity, PipelineConfig::new(2))?;

    // let result = pipeline.run(&batch).await?;

    // println!("{result:#?}");

    // ===========================================
    // SECTION 9
    // ===========================================

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    // let severity = OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?;

    // let diagnosis = OllamaDiagnosisAgent::new(DiagnosisConfig::new(MODEL))?;

    // let pipeline = AnalysisPipeline::new(
    //     analyzer,
    //     correlator,
    //     severity,
    //     diagnosis,
    //     PipelineConfig::new(2, 2),
    // )?;

    // let result = pipeline.run(&batch).await?;

    // println!("{result:#?}");

    // ===========================================
    // SECTION 10
    // ===========================================

    tracing_subscriber::fmt().with_target(false).init();

    let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    let raw_logs = source.collect()?;

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer.normalize(&raw_logs)?;

    let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    let severity = OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?;

    // let ssh_config = SshConfig::new(
    //     "freebsd",
    //     22,
    //     "spike",
    //     "/Users/laurent/.ssh/id_ed25519_proxmox",
    //     "/Users/laurent/.ssh/known_hosts",
    // );

    // let probe = Arc::new(SshFreeBsdProbe::connect(&ssh_config).await?);
    let probe = Arc::new(FakeFreeBsdProbe);

    let diagnosis = OllamaDiagnosisAgent::new(DiagnosisConfig::new(MODEL), probe)?;

    let pipeline = AnalysisPipeline::new(
        analyzer,
        correlator,
        severity,
        diagnosis,
        PipelineConfig::new(2, 2),
    )?;

    let result = pipeline.run(&batch).await?;

    println!("{result:#?}");
    Ok(())
}
