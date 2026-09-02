use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;

use otoko::probes::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe, TimedFreeBsdProbe};

struct SlowProbe;

fn observation() -> CommandObservation {
    CommandObservation {
        stdout: "ok".into(),

        stderr: String::new(),

        exit_status: 0,
    }
}

#[async_trait]
impl ReadOnlyFreeBsdProbe for SlowProbe {
    async fn disk_usage(&self) -> Result<CommandObservation, ProbeError> {
        sleep(Duration::from_millis(200)).await;

        Ok(observation())
    }

    async fn service_status(&self, _service: &str) -> Result<CommandObservation, ProbeError> {
        Ok(observation())
    }

    async fn socket_list(&self) -> Result<CommandObservation, ProbeError> {
        Ok(observation())
    }

    async fn recent_logins(&self) -> Result<CommandObservation, ProbeError> {
        Ok(observation())
    }
}

#[tokio::test]
async fn timed_probe_rejects_slow_operation() {
    let probe = TimedFreeBsdProbe::new(Arc::new(SlowProbe), Duration::from_millis(20));

    let error = probe
        .disk_usage()
        .await
        .expect_err("operation should time out");

    assert!(matches!(error, ProbeError::Timeout { .. }));
}
