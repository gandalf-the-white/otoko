use serde::{Deserialize, Serialize};

use super::Severity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentReport {
    pub title: String,
    pub severity: Severity,
    pub summary: String,
    pub evidence: Vec<String>,
    pub probable_causes: Vec<String>,
    pub investigations: Vec<String>,
    pub recommendations: Vec<String>,
}
