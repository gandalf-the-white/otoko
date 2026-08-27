use otoko::{
    agents::{AnalyzeLogs, OllamaLogAnalyzer},
    collector::{FakeLogSource, LogScenario, LogSource},
    normalizer::{LogNormalizer, SyslogNormalizer},
};

const MODEL: &str = "qwen3.6:latest";

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

    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect()?;

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer.normalize(&raw_logs)?;

    println!("{} log entries normalized", batch.len());

    let analyzer = OllamaLogAnalyzer::new(MODEL)?;

    let analysis = analyzer.analyze(&batch).await?;

    println!("{:#?}", analysis);

    Ok(())
}
