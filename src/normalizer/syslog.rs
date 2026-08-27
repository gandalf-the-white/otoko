use anyhow::{Result, anyhow};
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};

use crate::domain::{LogBatch, LogEntry, RawLog};

use super::LogNormalizer;

#[derive(Debug, Clone, Copy)]
pub struct SyslogNormalizer {
    year: i32,
}

impl SyslogNormalizer {
    pub fn new(year: i32) -> Self {
        Self { year }
    }

    pub fn current_year() -> Self {
        Self {
            year: Utc::now().year(),
        }
    }

    fn parse_line(&self, line: &str) -> Result<LogEntry> {
        let (month, rest) = take_field(line).ok_or_else(|| anyhow!("missing month"))?;

        let (day, rest) = take_field(rest).ok_or_else(|| anyhow!("missing day"))?;

        let (time, rest) = take_field(rest).ok_or_else(|| anyhow!("missing time"))?;

        let (host, rest) = take_field(rest).ok_or_else(|| anyhow!("missing host"))?;

        let (service_field, rest) = take_field(rest).ok_or_else(|| anyhow!("missing service"))?;

        let timestamp = self.parse_timestamp(month, day, time)?;

        let service = parse_service(service_field);

        let message = rest.trim_start().to_string();

        Ok(LogEntry {
            timestamp,
            host: host.to_string(),
            service,
            message,
        })
    }

    fn parse_timestamp(&self, month: &str, day: &str, time: &str) -> Result<DateTime<Utc>> {
        let value = format!("{month} {day} {} {time}", self.year);

        let naive = NaiveDateTime::parse_from_str(&value, "%b %e %Y %H:%M:%S")?;

        Ok(Utc.from_utc_datetime(&naive))
    }
}

impl LogNormalizer for SyslogNormalizer {
    fn normalize(&self, logs: &[RawLog]) -> Result<LogBatch> {
        let mut entries = Vec::new();

        for raw_log in logs {
            for line in raw_log.content.lines() {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                if let Ok(entry) = self.parse_line(line) {
                    entries.push(entry);
                }
            }
        }

        Ok(LogBatch::new(entries))
    }
}

fn take_field(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();

    match input.find(char::is_whitespace) {
        Some(index) => {
            let field = &input[..index];
            let rest = &input[index..];

            Some((field, rest))
        }

        None if !input.is_empty() => Some((input, "")),

        None => None,
    }
}

fn parse_service(value: &str) -> String {
    let without_colon = value.trim_end_matches(':');

    match without_colon.find('[') {
        Some(index) => without_colon[..index].to_string(),

        None => without_colon.to_string(),
    }
}
