#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrelatorConfig {
    pub model: String,
}

impl CorrelatorConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}
