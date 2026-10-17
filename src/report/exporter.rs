use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::domain::AnalysisReport;

use super::MarkdownReportRenderer;

pub struct MarkdownReportExporter {
    output_dir: PathBuf,
}

impl MarkdownReportExporter {
    pub fn new(output_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }

    pub async fn export(&self, report: &AnalysisReport) -> Result<PathBuf> {
        tokio::fs::create_dir_all(&self.output_dir)
            .await
            .context("unable to create report directory")?;

        let filename = format!("freebsd-report-{}.md", report.metadata.id);

        let path = self.output_dir.join(filename);

        let markdown = MarkdownReportRenderer::render(report);

        tokio::fs::write(&path, markdown)
            .await
            .with_context(|| format!("unable to write report to {}", path.display()))?;

        Ok(path)
    }

    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
}
