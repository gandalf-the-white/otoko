use async_trait::async_trait;
use std::future::Future;
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

use super::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

pub struct TimedFreeBsdProbe {
    inner: Arc<dyn ReadOnlyFreeBsdProbe>,

    timeout: Duration,
}

impl TimedFreeBsdProbe {
    pub fn new(inner: Arc<dyn ReadOnlyFreeBsdProbe>, timeout: Duration) -> Self {
        Self { inner, timeout }
    }
}

impl TimedFreeBsdProbe {
    async fn run<F>(
        &self,
        operation: &'static str,
        future: F,
    ) -> Result<CommandObservation, ProbeError>
    where
        F: Future<Output = Result<CommandObservation, ProbeError>>,
    {
        match timeout(self.timeout, future).await {
            Ok(result) => result,

            Err(_) => Err(ProbeError::Timeout {
                operation: operation.into(),
            }),
        }
    }
}

#[async_trait]
impl ReadOnlyFreeBsdProbe for TimedFreeBsdProbe {
    async fn service_status(&self, service: &str) -> Result<CommandObservation, ProbeError> {
        self.run("service_status", self.inner.service_status(service))
            .await
    }

    async fn disk_usage(&self) -> Result<CommandObservation, ProbeError> {
        self.run("disk_usage", self.inner.disk_usage()).await
    }

    async fn socket_list(&self) -> Result<CommandObservation, ProbeError> {
        self.run("socket_list", self.inner.socket_list()).await
    }

    async fn recent_logins(&self) -> Result<CommandObservation, ProbeError> {
        self.run("recent_logins", self.inner.recent_logins()).await
    }
}
