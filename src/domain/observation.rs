use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationStatus {
    Success,
    Failed,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemObservation {
    pub source: String,
    pub status: ObservationStatus,
    pub stdout: String,
    pub stderr: String,
    pub exit_status: Option<u32>,
    pub error: Option<String>,
}
