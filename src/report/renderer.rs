use std::fmt::Write;

use crate::domain::AnalysisReport;

pub struct MarkdownReportRenderer;

impl MarkdownReportRenderer {
    pub fn render(report: &AnalysisReport) -> String {
        let mut output = String::new();

        writeln!(&mut output, "# FreeBSD Incident Analysis Report").unwrap();

        writeln!(&mut output, "\n**Report ID:** {}", report.metadata.id).unwrap();

        writeln!(
            &mut output,
            "**Generated at:** {}",
            report.metadata.generated_at.to_rfc3339()
        )
        .unwrap();

        writeln!(
            &mut output,
            "**Schema version:** {}",
            report.metadata.schema_version
        )
        .unwrap();

        writeln!(
            &mut output,
            "**Incident count:** {}",
            report.metadata.incident_count
        )
        .unwrap();

        writeln!(
            &mut output,
            "**Summary source:** {:?}",
            report.metadata.summary_source
        )
        .unwrap();

        writeln!(&mut output, "# FreeBSD Incident Analysis Report").unwrap();
        writeln!(&mut output).unwrap();

        writeln!(&mut output, "## Executive Summary").unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "{}", report.summary.executive_summary).unwrap();

        writeln!(&mut output, "\n## Key Findings").unwrap();

        for finding in &report.summary.key_findings {
            writeln!(&mut output, "- {finding}").unwrap();
        }

        writeln!(&mut output, "\n## Operational Impact").unwrap();
        writeln!(&mut output, "{}", report.summary.operational_impact).unwrap();

        writeln!(&mut output, "\n## Priorities").unwrap();

        for priority in &report.summary.priorities {
            writeln!(&mut output, "- {priority}").unwrap();
        }

        for (index, incident) in report.incidents.iter().enumerate() {
            writeln!(
                &mut output,
                "\n## Incident {}: {}",
                index + 1,
                incident.title
            )
            .unwrap();

            writeln!(&mut output, "\n**Severity:** {:?}", incident.severity).unwrap();

            writeln!(&mut output, "\n### Evidence").unwrap();

            for evidence in &incident.evidence {
                writeln!(&mut output, "- `{}`", evidence.replace('`', "\\`")).unwrap();
            }

            writeln!(&mut output, "\n### System Observations").unwrap();

            for observation in &incident.observations {
                writeln!(
                    &mut output,
                    "- **{}**: {:?}",
                    observation.source, observation.status
                )
                .unwrap();

                if !observation.stdout.is_empty() {
                    writeln!(&mut output, "```text").unwrap();
                    writeln!(&mut output, "{}", observation.stdout).unwrap();
                    writeln!(&mut output, "```").unwrap();
                }

                if let Some(error) = &observation.error {
                    writeln!(&mut output, "  Error: {error}").unwrap();
                }
            }

            writeln!(&mut output, "\n### Probable Causes").unwrap();

            for cause in &incident.probable_causes {
                writeln!(&mut output, "- {cause}").unwrap();
            }

            writeln!(&mut output, "\n### Investigations").unwrap();

            for investigation in &incident.investigations {
                writeln!(&mut output, "- {investigation}").unwrap();
            }

            writeln!(&mut output, "\n### Recommendations").unwrap();

            for recommendation in &incident.recommendations {
                writeln!(&mut output, "- {recommendation}").unwrap();
            }
        }

        output
    }
}
