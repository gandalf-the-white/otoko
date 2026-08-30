#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeverityConfig {
    pub model: String,
}

impl SeverityConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}
