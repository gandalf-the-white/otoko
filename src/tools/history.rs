use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::SystemObservation;

#[derive(Debug, Clone, Default)]
pub struct ObservationHistory {
    inner: Arc<Mutex<Vec<SystemObservation>>>,
}

impl ObservationHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn record(&self, observation: SystemObservation) {
        self.inner.lock().await.push(observation);
    }

    pub async fn snapshot(&self) -> Vec<SystemObservation> {
        self.inner.lock().await.clone()
    }

    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.lock().await.is_empty()
    }
}
