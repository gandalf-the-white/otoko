use std::sync::Arc;

use otoko::{
    agents::{OllamaCorrelator, OllamaDiagnosisAgent, OllamaLogAnalyzer, OllamaSeverityAgent},
    collector::{FakeLogSource, LogScenario, LogSource},
    config::{
        AnalyzerConfig, CorrelatorConfig, DiagnosisConfig, PipelineConfig, SeverityConfig,
        SshConfig,
    },
    normalizer::{LogNormalizer, SyslogNormalizer},
    orchestration::AnalysisPipeline,
    probes::SshFreeBsdProbe,
};

const MODEL: &str = "qwen3.8:latest";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let source = FakeLogSource::new(vec![RawLog {
    //     source: "/var/log/auth.log".into(),
    //     content: "Failed password for root".into(),
    // }]);

    // let logs = source.collect()?;

    // println!("{} source(s) de logs récupérée(s)", logs.len());

    // ------------------------------------------

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let logs = source.collect()?;

    // for log in logs {
    //     println!("Source : {}", log.source);
    //     println!("------------------------------");
    //     println!("{}", log.content);
    // }

    // ------------------------------------------

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // println!("{} événements syslog normalisés", batch.len());

    // for entry in batch.entries {
    //     println!(
    //         "{} | {} | {} | {}",
    //         entry.timestamp, entry.host, entry.service, entry.message,
    //     );
    // }

    // ------------------------------------------

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let config = AnalyzerConfig::new(MODEL);

    // let analyzer = OllamaLogAnalyzer::new(config)?;

    // let analysis = analyzer.analyze(&batch).await?;

    // println!("{:#?}", analysis);

    // ------------------------------------------

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    // let analysis = analyzer.analyze(&batch).await?;

    // let incidents = correlator.correlate(&analysis).await?;

    // println!("{incidents:#?}");

    // ------------------------------------------

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    // let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    // let analysis = analyzer.analyze(&batch).await?;

    // let incidents = correlator.correlate(&analysis).await?;

    // let severity_agent = OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?;

    // for incident in &incidents {
    //     let assessment = severity_agent.assess(incident).await?;

    //     println!("Incident: {}", incident.title);

    //     println!("Severity: {:?}", assessment.severity);

    //     println!("Confidence: {:.2}", assessment.confidence);

    //     println!("Justification: {}", assessment.justification);
    // }

    // ------------------------------------------

    // tracing_subscriber::fmt().with_target(false).init();

    // let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    // let raw_logs = source.collect()?;

    // let normalizer = SyslogNormalizer::new(2026);

    // let batch = normalizer.normalize(&raw_logs)?;

    // let analyzer = Arc::new(OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?);

    // let correlator = Arc::new(OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?);

    // let severity_agent = Arc::new(OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?);

    // let pipeline = AnalysisPipeline::new(
    //     analyzer,
    //     correlator,
    //     severity_agent,
    //     PipelineConfig::new(2)?,
    // );

    // let result = pipeline.run(&batch).await?;

    // println!("{result:#?}");

    // ------------------------------------------

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

    // ------------------------------------------
    tracing_subscriber::fmt().with_target(false).init();

    let source = FakeLogSource::from_scenario(LogScenario::SuspiciousSshSession);

    let raw_logs = source.collect()?;

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer.normalize(&raw_logs)?;

    let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    let severity = OllamaSeverityAgent::new(SeverityConfig::new(MODEL))?;

    let ssh_config = SshConfig::new(
        "freebsd",
        22,
        "spike",
        "/Users/laurent/.ssh/id_ed25519_proxmox",
        "/Users/laurent/.ssh/known_hosts",
    );

    let probe = Arc::new(SshFreeBsdProbe::connect(&ssh_config).await?);

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
