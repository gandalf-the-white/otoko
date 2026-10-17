use crate::{
    domain::{ObservationStatus, SystemObservation},
    probes::{CommandObservation, ProbeError},
};

pub fn to_system_observation(
    source: &str,
    result: Result<CommandObservation, ProbeError>,
) -> SystemObservation {
    match result {
        Ok(value) if value.exit_status == 0 => SystemObservation {
            source: source.to_string(),
            status: ObservationStatus::Success,
            stdout: value.stdout,
            stderr: value.stderr,
            exit_status: Some(value.exit_status),
            error: None,
        },

        Ok(value) => SystemObservation {
            source: source.to_string(),
            status: ObservationStatus::Failed,
            stdout: value.stdout,
            stderr: value.stderr,
            exit_status: Some(value.exit_status),
            error: Some(format!(
                "remote command exited with status {}",
                value.exit_status
            )),
        },

        Err(ProbeError::Timeout { operation }) => SystemObservation {
            source: source.to_string(),
            status: ObservationStatus::TimedOut,
            stdout: String::new(),
            stderr: String::new(),
            exit_status: None,
            error: Some(format!("probe timed out: {operation}")),
        },

        Err(error) => SystemObservation {
            source: source.to_string(),
            status: ObservationStatus::Failed,
            stdout: String::new(),
            stderr: String::new(),
            exit_status: None,
            error: Some(error.to_string()),
        },
    }
}
