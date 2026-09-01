use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key: PathBuf,
    pub known_hosts: PathBuf,
}

impl SshConfig {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        private_key: impl Into<PathBuf>,
        known_hosts: impl Into<PathBuf>,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            username: username.into(),
            private_key: private_key.into(),
            known_hosts: known_hosts.into(),
        }
    }
}
