use serde::{Deserialize, Serialize};

use crate::domain::Diagnosis;

use super::{Incident, LogAnalysis, SeverityAssessment};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssessedIncident {
    pub incident: Incident,
    pub severity: SeverityAssessment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub analysis: LogAnalysis,
    pub incidents: Vec<DiagnosedIncident>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosedIncident {
    pub assessed: AssessedIncident,
    pub diagnosis: Diagnosis,
}
