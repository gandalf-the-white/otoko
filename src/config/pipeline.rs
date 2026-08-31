// #[derive(Debug, thiserror::Error, PartialEq, Eq)]
// pub enum PipelineConfigError {
//     #[error("max_concurrent_severity must be greater than zero")]
//     ZeroConcurrency,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineConfig {
    pub max_concurrent_severity_assessments: usize,

    pub max_concurrent_diagnoses: usize,
}

impl PipelineConfig {
    pub fn new(
        max_concurrent_severity_assessments: usize,

        max_concurrent_diagnoses: usize,
    ) -> Self {
        Self {
            max_concurrent_severity_assessments,
            max_concurrent_diagnoses,
        }
    }
}
