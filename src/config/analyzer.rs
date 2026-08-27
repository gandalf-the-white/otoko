#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzerConfig {
    pub model: String,
}

impl AnalyzerConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}
