#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosisConfig {
    pub model: String,
}

impl DiagnosisConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}
