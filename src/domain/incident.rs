use serde::{Deserialize, Serialize};

use crate::domain::DetectedEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    pub title: String,
    pub events: Vec<DetectedEvent>,
    pub explanation: String,
}
