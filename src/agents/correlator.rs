use std::collections::HashSet;

use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{Incident, LogAnalysis};

#[async_trait]
pub trait CorrelateEvents {
    async fn correlate(&self, analysis: &LogAnalysis) -> Result<Vec<Incident>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct CorrelationPlan {
    pub incidents: Vec<IncidentCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct IncidentCandidate {
    pub title: String,
    pub event_indices: Vec<usize>,
    pub explanation: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CorrelationError {
    #[error("event index {index} does not exist")]
    InvalidEventIndex { index: usize },

    #[error("event index {index} appears more than once in an incident")]
    DuplicateEventIndex { index: usize },

    #[error("an incident must contain at least one event")]
    EmptyIncident,
}

pub(crate) fn build_incidents(
    analysis: &LogAnalysis,
    plan: CorrelationPlan,
) -> Result<Vec<Incident>, CorrelationError> {
    let mut incidents = Vec::new();

    for candidate in plan.incidents {
        validate_candidate(&candidate, analysis.events.len())?;

        let events = candidate
            .event_indices
            .iter()
            .map(|&index| analysis.events[index].clone())
            .collect();

        incidents.push(Incident {
            title: candidate.title,

            events,

            explanation: candidate.explanation,
        });
    }

    Ok(incidents)
}

fn validate_candidate(
    candidate: &IncidentCandidate,
    event_count: usize,
) -> Result<(), CorrelationError> {
    if candidate.event_indices.is_empty() {
        return Err(CorrelationError::EmptyIncident);
    }

    let mut seen = HashSet::new();

    for &index in &candidate.event_indices {
        if index >= event_count {
            return Err(CorrelationError::InvalidEventIndex { index });
        }

        if !seen.insert(index) {
            return Err(CorrelationError::DuplicateEventIndex { index });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::domain::{DetectedEvent, EventCategory, LogAnalysis};

    use super::*;

    fn analysis() -> LogAnalysis {
        LogAnalysis {
            events: vec![
                DetectedEvent {
                    category: EventCategory::Authentication,

                    summary: "SSH failures".into(),

                    evidence: vec!["failed".into()],
                },
                DetectedEvent {
                    category: EventCategory::Security,

                    summary: "sudo activity".into(),

                    evidence: vec!["sudo".into()],
                },
            ],
        }
    }

    #[test]
    fn builds_incident_from_existing_events() {
        let analysis = analysis();

        let plan = CorrelationPlan {
            incidents: vec![IncidentCandidate {
                title: "Suspicious session".into(),

                event_indices: vec![0, 1],

                explanation: "Events appear related".into(),
            }],
        };

        let incidents = build_incidents(&analysis, plan).expect("plan should be valid");

        assert_eq!(incidents.len(), 1);

        assert_eq!(incidents[0].events.len(), 2);

        assert_eq!(incidents[0].events[0], analysis.events[0]);

        assert_eq!(incidents[0].events[1], analysis.events[1]);
    }

    #[test]
    fn rejects_unknown_event_index() {
        let analysis = analysis();

        let plan = CorrelationPlan {
            incidents: vec![IncidentCandidate {
                title: "Invalid incident".into(),

                event_indices: vec![0, 42],

                explanation: "Invalid test".into(),
            }],
        };

        let result = build_incidents(&analysis, plan);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_event_index() {
        let analysis = analysis();

        let plan = CorrelationPlan {
            incidents: vec![IncidentCandidate {
                title: "Duplicated".into(),

                event_indices: vec![0, 0],

                explanation: "test".into(),
            }],
        };

        let error = build_incidents(&analysis, plan).expect_err("duplicate should fail");

        assert!(matches!(
            error,
            CorrelationError::DuplicateEventIndex { index: 0 }
        ));
    }

    #[test]
    fn rejects_empty_incident() {
        let analysis = analysis();

        let plan = CorrelationPlan {
            incidents: vec![IncidentCandidate {
                title: "Empty".into(),

                event_indices: Vec::new(),

                explanation: "test".into(),
            }],
        };

        let error = build_incidents(&analysis, plan).expect_err("empty incident should fail");

        assert!(matches!(error, CorrelationError::EmptyIncident));
    }
}
