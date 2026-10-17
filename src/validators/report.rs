use anyhow::{Result, bail};

use crate::domain::ReportSummary;

pub struct SummaryValidator;

impl SummaryValidator {
    pub fn validate(summary: &ReportSummary) -> Result<()> {
        if summary.executive_summary.trim().is_empty() {
            bail!("executive summary is empty");
        }

        if summary.operational_impact.trim().is_empty() {
            bail!("operational impact is empty");
        }

        if summary
            .key_findings
            .iter()
            .any(|finding| finding.trim().is_empty())
        {
            bail!("key findings contain empty entries");
        }

        if summary
            .priorities
            .iter()
            .any(|priority| priority.trim().is_empty())
        {
            bail!("priorities contain empty entries");
        }

        Ok(())
    }
}
