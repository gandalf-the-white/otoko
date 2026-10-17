use crate::domain::{DiagnosedIncident, IncidentReport};

pub struct ReportDataBuilder;

impl ReportDataBuilder {
    pub fn build_incident(incident: &DiagnosedIncident) -> IncidentReport {
        IncidentReport {
            title: incident.assessed.incident.title.clone(),

            summary: incident.assessed.incident.explanation.clone(),

            severity: incident.assessed.severity.severity,

            evidence: incident
                .assessed
                .incident
                .events
                .iter()
                .flat_map(|event| event.evidence.iter().cloned())
                .collect(),

            observations: incident.observations.clone(),

            probable_causes: incident
                .diagnosis
                .probable_causes
                .iter()
                .map(|cause| cause.description.clone())
                .collect(),

            investigations: incident
                .diagnosis
                .investigations
                .iter()
                .map(|investigation| investigation.description.clone())
                .collect(),

            recommendations: incident
                .diagnosis
                .recommendations
                .iter()
                .map(|recommendation| recommendation.description.clone())
                .collect(),
        }
    }
}
