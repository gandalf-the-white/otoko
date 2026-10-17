use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Severity, SystemObservation};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentReport {
    pub title: String,
    pub severity: Severity,
    pub summary: String,
    pub evidence: Vec<String>,
    pub observations: Vec<SystemObservation>,
    pub probable_causes: Vec<String>,
    pub investigations: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReportSummary {
    pub executive_summary: String,
    pub key_findings: Vec<String>,
    pub operational_impact: String,
    pub priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub metadata: ReportMetadata,
    pub summary: ReportSummary,
    pub incidents: Vec<IncidentReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub schema_version: String,
    pub incident_count: usize,
    pub summary_source: SummarySource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummarySource {
    Ollama,
    Fallback,
}
