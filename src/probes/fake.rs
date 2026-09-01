use async_trait::async_trait;

use super::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

#[derive(Debug, Clone)]
pub struct FakeFreeBsdProbe;

fn observation(stdout: impl Into<String>) -> CommandObservation {
    CommandObservation {
        stdout: stdout.into(),
        stderr: String::new(),
        exit_status: 0,
    }
}

#[async_trait]
impl ReadOnlyFreeBsdProbe for FakeFreeBsdProbe {
    async fn service_status(&self, service: &str) -> Result<CommandObservation, ProbeError> {
        Ok(observation(format!("{service} is running")))
    }

    async fn disk_usage(&self) -> Result<CommandObservation, ProbeError> {
        Ok(observation("/dev/ada0p2  100G  20G  80G  20% /"))
    }

    async fn socket_list(&self) -> Result<CommandObservation, ProbeError> {
        Ok(observation("root sshd 812 4 tcp4 *:22 *:*"))
    }

    async fn recent_logins(&self) -> Result<CommandObservation, ProbeError> {
        Ok(observation("spike pts/0 10.0.0.52"))
    }
}
