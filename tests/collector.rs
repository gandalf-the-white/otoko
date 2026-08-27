use otoko::{
    collector::{FakeLogSource, LogScenario, LogSource},
    domain::RawLog,
};

#[test]
fn fake_source_returns_configured_logs() {
    let source = FakeLogSource::new(vec![
        RawLog {
            source: "/var/log/messages".to_string(),
            content: "system started".to_string(),
        },
        RawLog {
            source: "/var/log/auth.log".to_string(),
            content: "Failed password".to_string(),
        },
    ]);

    let logs = source.collect().expect("fake source should collect logs");

    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0].source, "/var/log/messages");
    assert_eq!(logs[1].source, "/var/log/auth.log");
}

#[test]
fn fake_source_can_be_empty() {
    let source = FakeLogSource::new(Vec::new());

    let logs = source
        .collect()
        .expect("empty fake source should still succeed");

    assert!(logs.is_empty());
}

#[test]
fn ssh_brute_force_scenario_contains_failed_passwords() {
    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let logs = source
        .collect()
        .expect("scenario collection should succeed");

    assert_eq!(logs.len(), 1);

    assert!(logs[0].content.contains("Failed password for root"));

    assert!(logs[0].content.contains("10.0.0.52"));
}

#[test]
fn disk_full_scenario_contains_no_space_error() {
    let source = FakeLogSource::from_scenario(LogScenario::DiskFull);

    let logs = source
        .collect()
        .expect("scenario collection should succeed");

    assert!(
        logs.iter()
            .any(|log| log.content.contains("No space left on device")
                || log.content.contains("no space left on device"))
    );
}

#[test]
fn all_scenarios_can_be_collected() {
    let scenarios = [
        LogScenario::Normal,
        LogScenario::SshBruteForce,
        LogScenario::ServiceFailure,
        LogScenario::DiskFull,
    ];

    for scenario in scenarios {
        let source = FakeLogSource::from_scenario(scenario);

        let logs = source
            .collect()
            .expect("every scenario should be collectable");

        assert!(!logs.is_empty());
    }
}

#[test]
fn log_source_can_be_used_through_the_trait() {
    fn count_logs(source: &impl LogSource) -> anyhow::Result<usize> {
        Ok(source.collect()?.len())
    }

    let source = FakeLogSource::from_scenario(LogScenario::Normal);

    let count = count_logs(&source).expect("counting should succeed");

    assert_eq!(count, 2);
}
