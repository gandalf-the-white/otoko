#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PipelineConfigError {
    #[error("max_concurrent_severity must be greater than zero")]
    ZeroConcurrency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineConfig {
    pub max_concurrent_severity: usize,
}

// impl PipelineConfig {
//     pub fn new(max_concurrent_severity: usize) -> Self {
//         Self {
//             max_concurrent_severity,
//         }
//     }
// }

impl PipelineConfig {
    pub fn new(max_concurrent_severity: usize) -> Result<Self, PipelineConfigError> {
        if max_concurrent_severity == 0 {
            return Err(PipelineConfigError::ZeroConcurrency);
        }

        Ok(Self {
            max_concurrent_severity,
        })
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_severity: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_rejects_zero_concurrency() {
        let result = PipelineConfig::new(0);

        assert!(matches!(result, Err(PipelineConfigError::ZeroConcurrency)));
    }

    #[test]
    fn pipeline_accepts_positive_concurrency() {
        let config = PipelineConfig::new(3).expect("3 should be valid");

        assert_eq!(config.max_concurrent_severity, 3);
    }
}
