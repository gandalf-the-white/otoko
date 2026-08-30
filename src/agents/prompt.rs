use anyhow::Result;
use rig::serde_json;

use crate::domain::{Incident, LogAnalysis, LogBatch};

pub const ANALYZER_PREAMBLE: &str = r#"
You are a system log analysis agent specialized in FreeBSD systems.

Your responsibility is limited to identifying significant events
from the supplied normalized log entries.

Rules:

- Base every conclusion only on the supplied logs.
- Do not invent evidence.
- Do not diagnose root causes.
- Do not assign severity.
- Do not recommend remediation.
- Group related log entries into meaningful events.
- Keep summaries concise.
- Every evidence item must be copied verbatim from the message
  field of one supplied log entry.
- Evidence must not be paraphrased, summarized, or invented.
- If no significant event is present, return an empty event list.
"#;

pub const CORRELATOR_PREAMBLE: &str = r#"
You are an incident correlation agent.

You receive a numbered list of already detected system events.

Your only responsibility is to determine which existing events
belong to the same incident.

Rules:

- Do not create new events.
- Do not modify events.
- Refer to events only by their supplied numeric indices.
- Never use an index that is not present in the input.
- Include each event at most once in an incident.
- An incident must contain at least one event.
- Do not assign severity.
- Do not diagnose root causes.
- Do not recommend remediation.
- Keep the explanation concise and grounded in the supplied events.
- If events are unrelated, they may form separate incidents.
- If the input contains no events, return an empty incident list.
"#;

pub const SEVERITY_PREAMBLE: &str = r#"
You are an incident severity assessment agent specialized in
FreeBSD system incidents.

Your only responsibility is to assess the severity of the supplied
incident.

Severity levels:

- Informational:
  Expected or routine activity with no meaningful operational or
  security impact.

- Low:
  Minor anomaly with limited impact and no clear indication of
  significant degradation or security risk.

- Medium:
  Meaningful anomaly that deserves investigation but does not
  currently demonstrate major impact.

- High:
  Strong indication of significant operational impact or suspicious
  security activity requiring prompt attention.

- Critical:
  Clear evidence of severe service disruption, data loss, or
  immediately dangerous security impact.

Rules:

- Base the assessment only on the supplied incident.
- Do not invent evidence.
- Do not diagnose root causes.
- Do not recommend remediation.
- Do not modify the incident or its events.
- Confidence must be a number between 0.0 and 1.0 inclusive.
- Confidence expresses confidence in the severity classification,
  not the severity itself.
- Keep the justification concise.
"#;

pub fn build_analyzer_prompt(batch: &LogBatch) -> Result<String> {
    let logs = serde_json::to_string_pretty(&batch.entries)?;

    Ok(format!(
        "Analyze the following normalized FreeBSD logs:\n\n{logs}"
    ))
}

pub fn build_correlator_prompt(analysis: &LogAnalysis) -> Result<String> {
    let events = analysis
        .events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            format!(
                "Event {}\nCategory: {:?}\nSummary: {}\nEvidence: {}\n",
                index,
                event.category,
                event.summary,
                event.evidence.join(" | "),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!(
        "Correlate the following detected events into incidents:\n\n{events}"
    ))
}

pub fn build_severity_prompt(incident: &Incident) -> Result<String> {
    let incident = serde_json::to_string_pretty(incident)?;

    Ok(format!(
        "Assess the severity of the following incident:\n\n{incident}"
    ))
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::domain::{DetectedEvent, EventCategory, LogBatch, LogEntry};

    use super::*;

    #[test]
    fn prompt_contains_log_information() {
        let batch = LogBatch::new(vec![LogEntry {
            timestamp: Utc.with_ymd_and_hms(2026, 8, 26, 14, 1, 2).unwrap(),

            host: "freebsd".into(),

            service: "sshd".into(),

            message: "Failed password for root".into(),
        }]);

        let prompt = build_analyzer_prompt(&batch).expect("prompt generation should succeed");

        assert!(prompt.contains("Failed password for root"));

        assert!(prompt.contains("sshd"));

        assert!(prompt.contains("freebsd"));
    }

    #[test]
    fn correlator_prompt_numbers_events() {
        let analysis = LogAnalysis {
            events: vec![
                DetectedEvent {
                    category: EventCategory::Authentication,

                    summary: "SSH failures".into(),

                    evidence: vec!["failure".into()],
                },
                DetectedEvent {
                    category: EventCategory::Security,

                    summary: "sudo activity".into(),

                    evidence: vec!["sudo".into()],
                },
            ],
        };

        let prompt = build_correlator_prompt(&analysis).expect("prompt should be built");

        assert!(prompt.contains("Event 0"));

        assert!(prompt.contains("Event 1"));

        assert!(prompt.contains("SSH failures"));

        assert!(prompt.contains("sudo activity"));
    }

    #[test]
    fn severity_prompt_contains_incident_information() {
        let incident = Incident {
            title: "Suspicious SSH session".into(),

            events: vec![DetectedEvent {
                category: EventCategory::Authentication,

                summary: "Repeated SSH failures".into(),

                evidence: vec!["Failed password for spike".into()],
            }],

            explanation: "Related authentication activity".into(),
        };

        let prompt = build_severity_prompt(&incident).expect("prompt should be built");

        assert!(prompt.contains("Suspicious SSH session"));

        assert!(prompt.contains("Repeated SSH failures"));

        assert!(prompt.contains("Failed password for spike"));
    }
}
