mod fake;
mod ssh;

pub use fake::FakeFreeBsdProbe;
pub use ssh::SshFreeBsdProbe;

use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandObservation {
    pub stdout: String,
    pub stderr: String,
    pub exit_status: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
    #[error("invalid FreeBSD service name: {0}")]
    InvalidServiceName(String),

    #[error("SSH error: {0}")]
    Ssh(#[from] async_ssh2_tokio::Error),
}

#[async_trait]
pub trait ReadOnlyFreeBsdProbe: Send + Sync {
    async fn service_status(&self, service: &str) -> Result<CommandObservation, ProbeError>;

    async fn disk_usage(&self) -> Result<CommandObservation, ProbeError>;

    async fn socket_list(&self) -> Result<CommandObservation, ProbeError>;

    async fn recent_logins(&self) -> Result<CommandObservation, ProbeError>;
}
