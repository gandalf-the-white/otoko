use std::{future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};
use serde_json::{Value, json};

use crate::probes::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

use super::NoArgs;

pub struct DiskUsageTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
}

impl DiskUsageTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Self {
        Self { probe }
    }
}

impl Tool for DiskUsageTool {
    const NAME: &'static str = "get_disk_usage";

    type Args = NoArgs;

    type Output = CommandObservation;

    type Error = ProbeError;

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

        async move { probe.disk_usage().await }
    }
}
