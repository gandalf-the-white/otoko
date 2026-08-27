use anyhow::Result;
use rig::serde_json;

use crate::domain::LogBatch;

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

pub fn build_analyzer_prompt(batch: &LogBatch) -> Result<String> {
    let logs = serde_json::to_string_pretty(&batch.entries)?;

    Ok(format!(
        "Analyze the following normalized FreeBSD logs:\n\n{logs}"
    ))
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::domain::{LogBatch, LogEntry};

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
}
