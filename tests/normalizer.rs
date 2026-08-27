use chrono::{TimeZone, Utc};

use otoko::{
    collector::{FakeLogSource, LogScenario, LogSource},
    domain::RawLog,
    normalizer::{LogNormalizer, SyslogNormalizer},
};

#[test]
fn normalizer_parses_standard_syslog_line() {
    let logs = vec![RawLog {
        source: "/var/log/auth.log".into(),
        content: concat!(
            "Aug 26 14:01:02 freebsd sshd[800]: ",
            "Failed password for root from 10.0.0.52"
        )
        .into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 1);

    let entry = &batch.entries[0];

    assert_eq!(
        entry.timestamp,
        Utc.with_ymd_and_hms(2026, 8, 26, 14, 1, 2).unwrap()
    );

    assert_eq!(entry.host, "freebsd");
    assert_eq!(entry.service, "sshd");

    assert_eq!(entry.message, "Failed password for root from 10.0.0.52");
}

#[test]
fn normalizer_removes_process_id_from_service() {
    let logs = vec![RawLog {
        source: "/var/log/auth.log".into(),
        content: "Aug 26 14:01:02 freebsd sshd[800]: test".into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.entries[0].service, "sshd");
}

#[test]
fn normalizer_parses_service_without_pid() {
    let logs = vec![RawLog {
        source: "/var/log/messages".into(),
        content: concat!(
            "Aug 26 16:20:01 freebsd kernel: ",
            "filesystem /var: write failed"
        )
        .into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 1);
    assert_eq!(batch.entries[0].service, "kernel");
}

#[test]
fn normalizer_parses_multiple_lines() {
    let logs = vec![RawLog {
        source: "/var/log/auth.log".into(),
        content: concat!(
            "Aug 26 14:01:02 freebsd sshd[800]: failed login\n",
            "Aug 26 14:01:08 freebsd sshd[801]: failed login\n",
            "Aug 26 14:01:15 freebsd sshd[802]: failed login\n",
        )
        .into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 3);
}

#[test]
fn normalizer_combines_multiple_raw_logs() {
    let logs = vec![
        RawLog {
            source: "/var/log/messages".into(),
            content: "Aug 26 10:00:00 freebsd kernel: system started".into(),
        },
        RawLog {
            source: "/var/log/auth.log".into(),
            content: "Aug 26 10:01:00 freebsd sshd[100]: login accepted".into(),
        },
    ];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 2);

    assert_eq!(batch.entries[0].service, "kernel");
    assert_eq!(batch.entries[1].service, "sshd");
}

#[test]
fn normalizer_accepts_empty_raw_log() {
    let logs = vec![RawLog {
        source: "/var/log/messages".into(),
        content: String::new(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("empty input should succeed");

    assert!(batch.is_empty());
}

#[test]
fn normalizer_ignores_invalid_line() {
    let logs = vec![RawLog {
        source: "/var/log/messages".into(),
        content: concat!(
            "this is not a valid syslog line\n",
            "Aug 26 10:00:00 freebsd kernel: valid message\n",
        )
        .into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 1);

    assert_eq!(batch.entries[0].message, "valid message");
}

#[test]
fn normalizer_parses_single_digit_day() {
    let logs = vec![RawLog {
        source: "/var/log/messages".into(),
        content: "Aug  6 14:01:02 freebsd kernel: test".into(),
    }];

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 1);

    assert_eq!(
        batch.entries[0].timestamp,
        Utc.with_ymd_and_hms(2026, 8, 6, 14, 1, 2).unwrap()
    );
}

#[test]
fn ssh_brute_force_scenario_is_normalized() {
    let source = FakeLogSource::from_scenario(LogScenario::SshBruteForce);

    let raw_logs = source.collect().expect("collection should succeed");

    let normalizer = SyslogNormalizer::new(2026);

    let batch = normalizer
        .normalize(&raw_logs)
        .expect("normalization should succeed");

    assert_eq!(batch.len(), 4);

    assert!(batch.entries.iter().all(|entry| entry.service == "sshd"));

    assert!(
        batch
            .entries
            .iter()
            .all(|entry| { entry.message.contains("Failed password") })
    );
}
