use std::{future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};
use serde_json::{Value, json};

use crate::{
    domain::SystemObservation,
    probes::ReadOnlyFreeBsdProbe,
    tools::{ObservationHistory, result::to_system_observation},
};

use super::NoArgs;

pub struct DiskUsageTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
    history: ObservationHistory,
}

impl DiskUsageTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>, history: ObservationHistory) -> Self {
        Self { probe, history }
    }
}

impl Tool for DiskUsageTool {
    const NAME: &'static str = "get_disk_usage";

    type Args = NoArgs;

    type Output = SystemObservation;

    type Error = std::convert::Infallible;

    fn description(&self) -> String {
        concat!(
            "Read filesystem disk usage from the FreeBSD host. ",
            "This tool is read-only."
        )
        .into()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties":
                false
        })
    }

    fn call(
        &self,
        _context: &mut ToolContext,

        _args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send {
        let probe = Arc::clone(&self.probe);

        let history = self.history.clone();

        async move {
            let observation = to_system_observation(Self::NAME, probe.disk_usage().await);

            history.record(observation.clone()).await;

            Ok(observation)
        }
    }
}
