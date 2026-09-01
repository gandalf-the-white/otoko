use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use async_trait::async_trait;

use crate::config::SshConfig;

use super::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

pub struct SshFreeBsdProbe {
    client: Client,
}

impl SshFreeBsdProbe {
    pub async fn connect(config: &SshConfig) -> Result<Self, ProbeError> {
        let auth = AuthMethod::with_key_file(&config.private_key, None);

        let known_hosts = config.known_hosts.to_string_lossy().into_owned();

        let server_check = ServerCheckMethod::with_known_hosts_file(&known_hosts);

        let client = Client::connect(
            (config.host.as_str(), config.port),
            &config.username,
            auth,
            server_check,
        )
        .await?;

        Ok(Self { client })
    }

    async fn execute(&self, command: &str) -> Result<CommandObservation, ProbeError> {
        let result = self.client.execute(command).await?;

        Ok(CommandObservation {
            stdout: result.stdout,
            stderr: result.stderr,
            exit_status: result.exit_status,
        })
    }
}

#[async_trait]
impl ReadOnlyFreeBsdProbe for SshFreeBsdProbe {
    async fn disk_usage(&self) -> Result<CommandObservation, ProbeError> {
        self.execute("/bin/df -h").await
    }

    async fn socket_list(&self) -> Result<CommandObservation, ProbeError> {
        self.execute("/usr/bin/sockstat -46").await
    }

    async fn recent_logins(&self) -> Result<CommandObservation, ProbeError> {
        self.execute("/usr/bin/last -20").await
    }

    async fn service_status(&self, service: &str) -> Result<CommandObservation, ProbeError> {
        validate_service_name(service)?;

        let command = format!("/usr/sbin/service {service} onestatus");

        self.execute(&command).await
    }
}

fn validate_service_name(service: &str) -> Result<(), ProbeError> {
    let valid = !service.is_empty()
        && service
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');

    if !valid {
        return Err(ProbeError::InvalidServiceName(service.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_safe_service_name() {
        assert!(validate_service_name("postgresql").is_ok());
    }

    #[test]
    fn accepts_service_name_with_underscore() {
        assert!(validate_service_name("my_service").is_ok());
    }

    #[test]
    fn accepts_service_name_with_dash() {
        assert!(validate_service_name("service-01").is_ok());
    }

    #[test]
    fn rejects_empty_service_name() {
        assert!(validate_service_name("").is_err());
    }

    #[test]
    fn rejects_shell_injection() {
        assert!(validate_service_name("sshd; reboot").is_err());
    }

    #[test]
    fn rejects_command_substitution() {
        assert!(validate_service_name("$(reboot)").is_err());
    }

    #[test]
    fn rejects_shell_and_operator() {
        assert!(validate_service_name("sshd && reboot").is_err());
    }

    #[test]
    fn rejects_path() {
        assert!(validate_service_name("../../bin/sh").is_err());
    }
}
