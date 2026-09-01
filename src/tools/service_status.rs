use std::{future::Future, sync::Arc};

use rig::{
    serde_json,
    tool::{Tool, ToolContext},
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::probes::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

#[derive(Debug, Deserialize)]
pub struct ServiceStatusArgs {
    pub service: String,
}

pub struct ServiceStatusTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
}

impl ServiceStatusTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Self {
        Self { probe }
    }
}

impl Tool for ServiceStatusTool {
    const NAME: &'static str = "get_service_status";

    type Args = ServiceStatusArgs;

    type Output = CommandObservation;

    type Error = ProbeError;

    fn description(&self) -> String {
        concat!(
            "Read the current status of a FreeBSD rc.d service. ",
            "This tool is read-only and does not start, stop, ",
            "restart, enable, or modify the service."
        )
        .into()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "service": {
                    "type": "string",
                    "description":
                        "FreeBSD service name such as sshd or postgresql"
                }
            },
            "required": [
                "service"
            ],
            "additionalProperties":
                false
        })
    }

    fn call(
        &self,
        _context: &mut ToolContext,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send {
        let probe = Arc::clone(&self.probe);

        async move { probe.service_status(&args.service).await }
    }
}
