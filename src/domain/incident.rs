use serde::{Deserialize, Serialize};

use crate::domain::DetectedEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    pub title: String,
    pub events: Vec<DetectedEvent>,
    pub explanation: String,
}

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Diagnosis {
//     pub probable_causes: Vec<String>,
//     pub investigations: Vec<String>,
//     pub recommendations: Vec<String>,
// }
