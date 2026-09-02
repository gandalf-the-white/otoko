use std::sync::Arc;

use tokio::sync::Mutex;

use super::ToolObservation;

#[derive(Debug, Clone, Default)]
pub struct ObservationHistory {
    inner: Arc<Mutex<Vec<ToolObservation>>>,
}

impl ObservationHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn record(&self, observation: ToolObservation) {
        self.inner.lock().await.push(observation);
    }

    pub async fn snapshot(&self) -> Vec<ToolObservation> {
        self.inner.lock().await.clone()
    }

    pub async fn clear(&self) {
        self.inner.lock().await.clear();
    }

    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.lock().await.is_empty()
    }

    pub async fn since(&self, index: usize) -> Vec<ToolObservation> {
        self.inner
            .lock()
            .await
            .get(index..)
            .unwrap_or_default()
            .to_vec()
    }
}
