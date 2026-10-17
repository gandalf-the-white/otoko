use crate::domain::{IncidentReport, ReportSummary};

pub struct FallbackReportSummary;

impl FallbackReportSummary {
    pub fn generate(incidents: &[IncidentReport]) -> ReportSummary {
        let incident_count = incidents.len();

        let executive_summary = match incident_count {
            0 => "No incidents were identified in the analyzed data.".to_string(),

            1 => "The analysis identified 1 incident. \
                 The AI-generated narrative summary is unavailable."
                .to_string(),

            count => {
                format!(
                    "The analysis identified {count} incidents. \
                     The AI-generated narrative summary is unavailable."
                )
            }
        };

        let key_findings = incidents
            .iter()
            .map(|incident| format!("{} (severity: {:?})", incident.title, incident.severity))
            .collect();

        ReportSummary {
            executive_summary,

            key_findings,

            operational_impact: "Operational impact has not been independently \
                 established by the report generator."
                .into(),

            priorities: vec!["Review the incidents and their supporting evidence.".into()],
        }
    }
}
