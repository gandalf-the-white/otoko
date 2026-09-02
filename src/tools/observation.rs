use serde::{Deserialize, Serialize};

use crate::probes::{CommandObservation, ProbeError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationStatus {
    Success,
    Failed,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolObservation {
    pub tool: String,
    pub status: ObservationStatus,

    pub stdout: String,
    pub stderr: String,

    pub exit_status: Option<u32>,
    pub error: Option<String>,
}

impl ToolObservation {
    pub fn success(tool: impl Into<String>, observation: CommandObservation) -> Self {
        Self {
            tool: tool.into(),

            status: ObservationStatus::Success,

            stdout: observation.stdout,

            stderr: observation.stderr,

            exit_status: Some(observation.exit_status),

            error: None,
        }
    }

    pub fn from_probe_error(tool: impl Into<String>, error: ProbeError) -> Self {
        let status = match &error {
            ProbeError::Timeout { .. } => ObservationStatus::TimedOut,

            _ => ObservationStatus::Failed,
        };

        Self {
            tool: tool.into(),

            status,

            stdout: String::new(),

            stderr: String::new(),

            exit_status: None,

            error: Some(error.to_string()),
        }
    }
}
