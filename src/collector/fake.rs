use anyhow::Result;

use crate::domain::RawLog;

use super::LogSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogScenario {
    Normal,
    SshBruteForce,
    SuspiciousSshSession,
    ServiceFailure,
    DiskFull,
}

#[derive(Debug, Clone)]
pub struct FakeLogSource {
    logs: Vec<RawLog>,
}

impl FakeLogSource {
    pub fn new(logs: Vec<RawLog>) -> Self {
        Self { logs }
    }

    pub fn from_scenario(scenario: LogScenario) -> Self {
        let logs = match scenario {
            LogScenario::Normal => normal_logs(),
            LogScenario::SshBruteForce => ssh_brute_force_logs(),
            LogScenario::SuspiciousSshSession => suspicious_ssh_session_logs(),
            LogScenario::ServiceFailure => service_failure_logs(),
            LogScenario::DiskFull => disk_full_logs(),
        };

        Self { logs }
    }
}

impl LogSource for FakeLogSource {
    fn collect(&self) -> Result<Vec<RawLog>> {
        Ok(self.logs.clone())
    }
}

fn normal_logs() -> Vec<RawLog> {
    vec![
        RawLog {
            source: "/var/log/messages".into(),
            content: concat!(
                "Aug 26 13:00:01 freebsd cron[101]: ",
                "job completed successfully\n",
                "Aug 26 13:05:01 freebsd ntpd[320]: ",
                "time synchronized\n",
            )
            .into(),
        },
        RawLog {
            source: "/var/log/auth.log".into(),
            content: concat!(
                "Aug 26 13:10:12 freebsd sshd[720]: ",
                "Accepted publickey for spike from ",
                "192.168.1.20 port 51022 ssh2\n",
            )
            .into(),
        },
    ]
}

fn ssh_brute_force_logs() -> Vec<RawLog> {
    vec![RawLog {
        source: "/var/log/auth.log".into(),
        content: concat!(
            "Aug 26 14:01:02 freebsd sshd[800]: ",
            "Failed password for root from 10.0.0.52 port 50100 ssh2\n",
            "Aug 26 14:01:08 freebsd sshd[801]: ",
            "Failed password for root from 10.0.0.52 port 50101 ssh2\n",
            "Aug 26 14:01:15 freebsd sshd[802]: ",
            "Failed password for admin from 10.0.0.52 port 50102 ssh2\n",
            "Aug 26 14:01:22 freebsd sshd[803]: ",
            "Failed password for spike from 10.0.0.52 port 50103 ssh2\n",
        )
        .into(),
    }]
}

fn service_failure_logs() -> Vec<RawLog> {
    vec![RawLog {
        source: "/var/log/messages".into(),
        content: concat!(
            "Aug 26 15:02:11 freebsd postgres[910]: ",
            "database system is shutting down\n",
            "Aug 26 15:02:13 freebsd postgres[910]: ",
            "server process exited with exit code 1\n",
            "Aug 26 15:02:14 freebsd postgres[910]: ",
            "database system is stopped\n",
        )
        .into(),
    }]
}

fn disk_full_logs() -> Vec<RawLog> {
    vec![RawLog {
        source: "/var/log/messages".into(),
        content: concat!(
            "Aug 26 16:20:01 freebsd kernel: ",
            "filesystem /var: write failed, no space left on device\n",
            "Aug 26 16:20:03 freebsd syslogd[540]: ",
            "write error: No space left on device\n",
        )
        .into(),
    }]
}

fn suspicious_ssh_session_logs() -> Vec<RawLog> {
    vec![RawLog {
        source: "/var/log/auth.log".into(),

        content: concat!(
            "Aug 26 14:01:02 freebsd sshd[800]: ",
            "Failed password for spike from 10.0.0.52 port 50100 ssh2\n",
            "Aug 26 14:01:08 freebsd sshd[801]: ",
            "Failed password for spike from 10.0.0.52 port 50101 ssh2\n",
            "Aug 26 14:01:15 freebsd sshd[802]: ",
            "Failed password for spike from 10.0.0.52 port 50102 ssh2\n",
            "Aug 26 14:04:10 freebsd sshd[810]: ",
            "Accepted password for spike from 10.0.0.52 port 50110 ssh2\n",
            "Aug 26 14:06:21 freebsd sudo[830]: ",
            "spike : COMMAND=/usr/bin/id\n",
        )
        .into(),
    }]
}
