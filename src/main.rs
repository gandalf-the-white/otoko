use otoko::{
    agents::{AnalyzeLogs, CorrelateEvents, OllamaCorrelator, OllamaLogAnalyzer},
    collector::{FakeLogSource, LogScenario, LogSource},
    config::{AnalyzerConfig, CorrelatorConfig},
    normalizer::{LogNormalizer, SyslogNormalizer},
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

    tracing_subscriber::fmt().with_target(false).init();

    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect()?;

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer.normalize(&raw_logs)?;

    let analyzer = OllamaLogAnalyzer::new(AnalyzerConfig::new(MODEL))?;

    let correlator = OllamaCorrelator::new(CorrelatorConfig::new(MODEL))?;

    let analysis = analyzer.analyze(&batch).await?;

    let incidents = correlator.correlate(&analysis).await?;

    println!("{incidents:#?}");

    Ok(())
}
